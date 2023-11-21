use crate::{
    eval, parse,
    parser::{ConstName, VarName},
    Expr,
};

// The following programs should fail to terminate:
#[test]
fn application_error() {
    let expr = parse("C() C()").unwrap();
    assert!(eval(expr).is_err());
}

#[test]
fn non_terminating() {
    let expr = parse("rec x = x").unwrap();
    assert!(eval(expr).is_err());
}

#[test]
fn case_no_constructor_error() {
    let expr = parse(r"case \x. x of {}").unwrap();
    assert!(eval(expr).is_err());
}

#[test]
fn case_arity_to_many_error() {
    let expr = parse(r"case C() of { C(x) -> C() }").unwrap();
    assert!(eval(expr).is_err());
}

#[test]
fn case_arity_to_few_error() {
    let expr = parse(r"case C(C()) of { C() -> C(); C(x) -> x }").unwrap();
    assert!(eval(expr).is_err());
}

#[test]
fn case_lookup_error() {
    let expr = parse(r"case C() of { D() -> D() }").unwrap();
    assert!(eval(expr).is_err());
}

// The following programs should terminate with specific results

#[test]
fn case_subst_order() {
    let expr = parse(r"case C(D(),E()) of { C(x, x) -> x } ").unwrap();
    assert_eq!(
        eval(expr).unwrap(),
        Expr::Const(ConstName("E".into()), vec![])
    );
}

#[test]
fn case_and_application() {
    let expr = parse(r"case C(\x.x, Zero()) of { C(f, x) -> f x }").unwrap();
    assert_eq!(
        eval(expr).unwrap(),
        Expr::Const(ConstName("Zero".into()), vec![])
    );
}

#[test]
fn case_and_application2() {
    let expr = parse(r"case (\x.x) C() of { C() -> C() } ").unwrap();
    assert_eq!(
        eval(expr).unwrap(),
        Expr::Const(ConstName("C".into()), vec![])
    );
}

#[test]
fn application() {
    let expr = parse(r"((\x.x)(\x.x))(\x.x)").unwrap();
    let x = VarName("x".into());
    assert_eq!(
        eval(expr).unwrap(),
        Expr::Lambda(x.clone(), Box::new(Expr::Var(x)))
    );
}

#[test]
fn application_left_assoc() {
    let program = parse(
        r#"
        (\_. \x. x) Foo() Bar() 
    "#,
    );
    let expr = eval(program.unwrap()).unwrap();
    assert_eq!(expr, Expr::Const(ConstName("Bar".into()), vec![]))
}

#[test]
fn sample_program_equiv() {
    let src = r#"
    let foo = rec foo = \m. \n. case m of
    { Zero() -> case n of
      { Zero() -> True()
      ; Suc(n) -> False()
      }
    ; Suc(m) -> case n of
      { Zero() -> False()
      ; Suc(n) -> foo m n
      }
    };

    foo Suc(Zero()) Suc(Suc(Zero()))
    "#;
    let program = dbg!(parse(src)).unwrap();
    let expr = dbg!(eval(program)).unwrap();
    assert_eq!(expr, Expr::Const(ConstName("False".into()), vec![]))
}
