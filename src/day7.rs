use nom::{multispace,digit, is_space};
use nom::IResult::*;

use std::collections::HashMap;
use std::collections::hash_map::Entry;

use std::str::FromStr;


#[derive(PartialEq, Hash, Eq, Debug, Clone)]
enum Term<'a> {
     Var(&'a str),
     Value(u16)
 }


#[allow(dead_code)]
#[derive(PartialEq, Eq, Debug)]
enum Expression<'a> {
    And(Term<'a>, Term<'a>),
    Or(Term<'a>, Term<'a>),
    LShift(Term<'a>, u16),//could be u8 as we can only shift by 16 max
    RShift(Term<'a>, u16),
    Not(Term<'a>),
    Term(Term<'a>)
}

#[allow(dead_code)]
named!(u16_digit<&str, u16>,
  map_res!(
    digit,
    FromStr::from_str
  )
);

fn is_space2(ch : char) -> bool { is_space(ch as u8)}

#[allow(dead_code)]
named!(parse_value<&str, Term>,
    map!(
         u16_digit,
        |n |  { Term::Value(n)}
    )
);

#[test]
fn parse_value_test() {
    assert_eq!(parse_value("16"), Done("", Term::Value(16)))
}

#[allow(dead_code)]
named!(parse_var<&str, Term>,
    map!(
         take_till_s!(is_space2),
        |var |  { Term::Var(var)}
    )
);

#[test]
fn parse_var_test() {
    assert_eq!(parse_var("g"), Done("", Term::Var("g")))
}

#[allow(dead_code)]
named!(parse_vv<&str, Term>,//value or expression
    alt_complete!(
        parse_value | parse_var
    )
);

#[test]
fn parse_vv_test() {
    assert_eq!(parse_vv("gh"), Done("", Term::Var("gh")));
    assert_eq!(parse_vv("134"), Done("", Term::Value(134)));
}

#[allow(dead_code)]
named!(parse_binop<&str, Expression>,
    chain!(
        var1 : parse_vv ~
        or : delimited!(opt!(multispace),
                alt!(
                  tag_s!("OR") => { |_|  true}
                | tag_s!("AND")  => { |_|  false}),
            opt!(multispace)) ~
        var2 : parse_vv,
        || {if or
            {Expression::Or(var1, var2)}
            else
            {Expression::And(var1, var2)}
        }
    )
);

#[test]
fn parse_binop_test() {
    assert_eq!(parse_binop("bf OR li"), Done("", Expression::Or(Term::Var("bf"), Term::Var("li"))))
}

#[allow(dead_code)]
named!(parse_shift<&str, Expression>,
    chain!(
        var1 : parse_vv ~
        or : delimited!(opt!(multispace),
                alt!(
                  tag_s!("LSHIFT") => { |_|  true}
                | tag_s!("RSHIFT")  => { |_|  false}),
            opt!(multispace)) ~
        var2 : u16_digit,
        || {if or
            {Expression::LShift(var1, var2)}
            else
            {Expression::RShift(var1, var2)}
        }
    )
);

#[test]
fn parse_shift_test() {
    assert_eq!(parse_shift("bf LSHIFT 3"), Done("", Expression::LShift(Term::Var("bf"), 3)))
}


#[allow(dead_code)]
named!(parse_unary<&str, Expression>,
    chain!( opt!(multispace) ~
            tag_s!("NOT") ~
            opt!(multispace) ~
        var : parse_vv,
        || {Expression::Not(var)}
    )
);

#[test]
fn parse_unary_test() {
    assert_eq!(parse_unary("NOT h"), Done("", Expression::Not(Term::Var("h"))))
}

#[allow(dead_code)]
named!(parse_affect<&str, Expression>,
    map!(
         parse_vv,
        |v |  {Expression::Term(v)}
    )
);

#[test]
fn parse_affect_test() {
    assert_eq!(parse_affect("g"), Done("", Expression::Term(Term::Var("g"))))
}


#[allow(dead_code)]
named!(parse_instr<&str, (&str, Expression)>,
    chain!(
        expr : alt_complete!(
            parse_binop | parse_unary | parse_shift | parse_affect
        ) ~
        opt!(multispace) ~
        tag_s!("->") ~
        opt!(multispace) ~
        output_name: take_till_s!(is_space2),
        || {(output_name, expr)}
    )
);



#[test]
fn parse_instr_test() {
    assert_eq!(parse_instr("bf OR li -> g"), Done("", ("g", Expression::Or(Term::Var("bf"), Term::Var("li")))));
    assert_eq!(parse_instr("17 -> g"), Done("", ("g", Expression::Term(Term::Value(17)))));
    assert_eq!(parse_instr("bf LSHIFT 3 -> g"), Done("", ("g", Expression::LShift(Term::Var("bf"), 3))));
    assert_eq!(parse_instr("NOT c -> g"), Done("", ("g", Expression::Not(Term::Var("c")))));
}


fn parse(s : &str) -> (&str, Expression) {
    match parse_instr(s) {
        Done(_, res) => res,
        Error(e) => panic!(format!("Parsing error: {}", e)),
        _ => panic!("Uncomplete input")
    }
}

fn interpreter<'a, 'b>(t : &'a Term<'b>, symbol_table : &HashMap<&str, Expression<'b> >, mut interm : &mut HashMap<&'b str, u16>) -> u16 {
    match t {
        &Term::Var(ref v1) => {
            //Check if the value has been calculated before

            //Bypass borrow checker
            let interm2 = interm as *mut HashMap<&'b str, u16>;

            match interm.entry(v1) {
                Entry::Vacant(e) => {
                    //println!("Seeing {} for the first time", v1);
                    let expr = symbol_table.get(v1).expect(&format!("Variable {} is not in the symbol table", v1));
                    let new_v = evaluate(expr, symbol_table,  unsafe {&mut *interm2 });
                    //insert the new value
                    e.insert(new_v);
                    new_v
                },
                Entry::Occupied(e) => *e.get()
            }
        },
        &Term::Value(n) => n
    }
}

fn evaluate<'a>(expr : &Expression<'a>, symbol_table : &HashMap<&str, Expression<'a> >, interm : &mut HashMap<&'a str, u16>) -> u16 {
        match expr {
        &Expression::And(ref v1, ref v2) => interpreter(v1, &symbol_table, interm) & interpreter(v2, symbol_table, interm),
        &Expression::Or(ref v1, ref v2) => interpreter(v1, symbol_table, interm) | interpreter(v2, symbol_table, interm),
        &Expression::LShift(ref v1, n) => interpreter(v1, symbol_table, interm) << n,
        &Expression::RShift(ref v1, n) => interpreter(v1, symbol_table, interm) >> n,
        &Expression::Not(ref v1) => ! interpreter(v1, symbol_table, interm),
        &Expression::Term(ref v1) => interpreter(v1, symbol_table, interm),
    }
}

fn interprete(target: &str, symbol_table : &HashMap<&str, Expression>) -> u16{
    //Using with_capacity here is very important here!!! It is extremely quick with it and extremely long whithout it
    let mut intermediate_results : HashMap<&str, u16> = HashMap::with_capacity(symbol_table.len());
    interpreter(&Term::Var(target), symbol_table, &mut intermediate_results)
}


pub fn day7_first(s : &str) -> u16 {
    let mut symbol_table = HashMap::with_capacity(339);

    println!("Starting parsing");
    for line in s.lines() {
        let (symbol, expr) = parse(line);

        symbol_table.insert(symbol, expr);
    };

    println!("Starting interpreting");
    let res = interprete("a", &symbol_table);
    println!("Result is {}", res);
    res
}


#[cfg(test)]
mod tests {
    use super::{day7_first, parse, interprete, Expression, Term};

    use std::collections::HashMap;

    #[test]
    fn day_first_test() {
        let prog = "123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i";
        let mut symbol_table = HashMap::new();

        for line in prog.lines() {
            let (symbol, expr) = parse(line);

            symbol_table.insert(symbol, expr);
        };

        /*
        d: 72
        e: 507
        f: 492
        g: 114
        h: 65412
        i: 65079
        x: 123
        y: 456
        */
        assert_eq!(interprete("d", &symbol_table), 72);
        assert_eq!(interprete("e", &symbol_table), 507);
        assert_eq!(interprete("f", &symbol_table), 492);
        assert_eq!(interprete("g", &symbol_table), 114);
        assert_eq!(interprete("h", &symbol_table), 65412);
        assert_eq!(interprete("i", &symbol_table), 65079);
        assert_eq!(interprete("x", &symbol_table), 123);
        assert_eq!(interprete("y", &symbol_table), 456);
    }
}
