use std::env;
use std::fs::File;
use std::io::prelude::*;

use sexp::Atom::*;
use sexp::*;

enum Expr {
    Num(i32),
    Add1(Box<Expr>),
    Sub1(Box<Expr>),
}

fn parse_expr(s: &Sexp) -> Expr {
    match s {
        Sexp::Atom(I(n)) => Expr::Num(i32::try_from(*n).unwrap()),
        Sexp::List(vec) => match &vec[..] {
            [Sexp::Atom(S(op)), e] => match op.as_str() {
                "add1" => Expr::Add1(Box::new(parse_expr(e))),
                "sub1" => Expr::Sub1(Box::new(parse_expr(e))),
                _ => panic!("Unknown op"),
            },
            _ => panic!("Parse error"),
        },
        _ => panic!("Parse error"),
    }
}

/// Compile a source program into a string of x86-64 assembly code.
fn compile_expr(expr: &Expr) -> String {
    match expr {
        Expr::Num(n) => format!("mov rax, {}", n),
        Expr::Add1(subexpr) => compile_expr(subexpr) + "\nadd rax, 1",
        Expr::Sub1(subexpr) => compile_expr(subexpr) + "\nsub rax, 1",
    }
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let in_name = &args[1];
    let out_name = &args[2];

    let mut in_file = File::open(in_name)?;
    let mut in_contents = String::new();
    in_file.read_to_string(&mut in_contents)?;

    let expr = parse(&in_contents).unwrap();
    let expr = parse_expr(&expr);
    let result = compile_expr(&expr);

    let asm_program = format!(
        "
section .text
global our_code_starts_here
our_code_starts_here:
  {}
  ret
",
        result
    );

    let mut out_file = File::create(out_name)?;
    out_file.write_all(asm_program.as_bytes())?;

    Ok(())
}
