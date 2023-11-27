use crate::{parse, parser::Variable, Expr, Program};

#[test]
fn variable() {
    parse(r"x").unwrap();
}

#[test]
fn constructor_nullary() {
    parse(r"Nil()").unwrap();
}

#[test]
fn constructor_unary() {
    parse(r"Suc(n)").unwrap();
}

#[test]
fn constructor_list() {
    parse(r"Cons(Zero(), Cons(Suc(Zero())), Nil())").unwrap();
}

#[test]
fn constructor_list_trailing() {
    parse(r"Cons(Zero(), Cons(Suc(Zero())), Nil(),)").unwrap();
}

#[test]
fn lambda() {
    parse(r"\x. x").unwrap();
}

#[test]
fn lambda_nested() {
    assert_eq!(
        parse(r"\x.\y. y").unwrap(),
        Program::Expr(Expr::Lambda(
            Variable("x".to_string()),
            Box::new(Expr::Lambda(
                Variable("y".to_string()),
                Box::new(Expr::Var(Variable("y".to_string())))
            ))
        ))
    )
}

#[test]
fn case() {
    parse(
        r#"
        case x of {
            Foo() -> y;
            Bar() -> z;
            Baz(a,b,c) -> d
        }
    "#,
    )
    .unwrap();
}

#[test]
fn case_trailing() {
    parse(
        r#"
        case x of {
            Foo() -> y;
        }
    "#,
    )
    .unwrap();
}

#[test]
fn application() {
    parse(r"(\x. x) Foo()").unwrap();
}
