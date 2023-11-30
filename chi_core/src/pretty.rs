use crate::{
    parser::Branch,
    Expr::{self, *},
};
use std::fmt::Write;

// Pretty printers for the concrete and abstract syntax used in the Computability course
// TODO: these printers are very wasteful with memory, would be a lot nicer if they just wrote to a single mutable buffer
// instead of creating new strings each recursive call

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

fn concrete_branches(branches: &Vec<Branch<Expr>>, indent: usize) -> String {
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

pub fn abstr(expr: &Expr) -> String {
    abstr_expr(expr)
}

fn abstr_expr(expr: &Expr) -> String {
    match expr {
        Apply(e1, e2) => format!("apply ({}) ({})", abstr_expr(e1), abstr_expr(e2)),
        Lambda(x, e) => format!(r"lambda <u>{x}</u> ({})", abstr_expr(e)),
        Case(e, branches) => {
            let branches: Vec<String> = branches.iter().map(abstr_branch).collect();
            format!("case ({}) ({})", abstr_expr(e), abstr_list(&branches))
        }
        Rec(x, e) => format!("rec <u>{x}</u> ({})", abstr_expr(e)),
        Var(x) => format!("var <u>{x}</u>"),
        Const(c, es) => {
            let es: Vec<String> = es.iter().map(|e| abstr_expr(e)).collect();
            format!("const <u>{c}</u> {es}", es = abstr_list(&es))
        }
    }
}

fn abstr_list(xs: &[String]) -> String {
    if let Some(x) = xs.get(0) {
        format!("(cons ({x}) {xs})", xs = abstr_list(&xs[1..]))
    } else {
        "nil".to_string()
    }
}

fn abstr_branch(Branch(c, vars, e): &Branch<Expr>) -> String {
    let vars: Vec<String> = vars.iter().map(|x| format!("<u>{x}</u>")).collect();
    format!(
        "branch <u>{c}</u> {vars} ({e})",
        vars = abstr_list(&vars),
        e = abstr_expr(e)
    )
}
