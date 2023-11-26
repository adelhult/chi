use crate::{
    parser::Branch,
    Expr::{self, *},
};
use std::fmt::Write;

// Pretty printers for the concrete and abstract syntax used in the Computability course

const INDENT: &'static str = "  ";

pub fn concrete(expr: &Expr) -> String {
    concrete_expr(expr, 0, 0)
}

fn concrete_expr(expr: &Expr, indent: usize, precedence_lvl: u8) -> String {
    let mut s = String::new();

    let current_precedence = precedence(expr);
    if current_precedence < precedence_lvl {
        s.push('(');
    }

    match expr {
        Apply(e1, e2) => write!(
            &mut s,
            "{} {}",
            concrete_expr(e1, indent, 1),
            concrete_expr(e2, indent, 2)
        )
        .unwrap(),
        Lambda(x, e) => write!(&mut s, r"\{x}. {}", concrete_expr(e, indent, 0)).unwrap(),
        Case(e, branches) => write!(
            &mut s,
            "case {} of {{\n{}\n{indent}}}",
            concrete_expr(e, indent, 0),
            concrete_branches(branches, indent + 1),
            indent = INDENT.repeat(indent)
        )
        .unwrap(),
        Rec(x, e) => write!(&mut s, "rec {x} = {}", concrete_expr(e, indent, 0)).unwrap(),
        Var(x) => write!(&mut s, "{x}").unwrap(),
        Const(c, es) => {
            let es: Vec<String> = es.iter().map(|e| concrete_expr(e, indent, 0)).collect();
            write!(&mut s, "{c}({es})", es = es.join(",")).unwrap()
        }
    }

    if current_precedence < precedence_lvl {
        s.push(')');
    }

    s
}

fn concrete_branches(branches: &Vec<Branch>, indent: usize) -> String {
    /*
    Foo() -> Bar();
    Baz() -> Bam()
    */
    let mut s = String::new();

    for (i, Branch(c, vars, expr)) in branches.iter().enumerate() {
        let vars: Vec<String> = vars.iter().map(|x| format!("{x}")).collect();
        write!(
            &mut s,
            "{indent}{c}({vars}) -> {expr}",
            indent = INDENT.repeat(indent),
            vars = vars.join(","),
            expr = concrete_expr(expr, indent, 0)
        )
        .unwrap();

        if i + 1 != branches.len() {
            s.push(';');
            s.push('\n');
        }
    }
    s
}

fn precedence(expr: &Expr) -> u8 {
    match expr {
        Apply(..) => 1,
        Lambda(..) => 0,
        Case(..) => 1,
        Rec(..) => 0,
        Var(..) => 2,
        Const(..) => 2,
    }
}

fn abstr(expr: &Expr) -> String {
    todo!()
}
