use crate::{
    eval::substitute,
    parser::{Branch, Constructor, Variable},
    Expr::*,
};

#[test]
fn subst_rec() {
    let x = Variable("x".into());
    let rec = Rec(x.clone(), Box::new(Var(x.clone())));
    let result = substitute(&x, &Const(Constructor("Z".into()), vec![]), rec.clone());
    assert_eq!(result, rec);
}

#[test]
fn subst_lambda() {
    let x = Variable("x".into());
    let y = Variable("y".into());
    let lambda = Lambda(
        x.clone(),
        Box::new(Apply(Box::new(Var(x.clone())), Box::new(Var(y.clone())))),
    );
    let result = substitute(
        &y,
        &Lambda(x.clone(), Box::new(Var(x.clone()))),
        lambda.clone(),
    );
    assert_eq!(
        result,
        Lambda(
            x.clone(),
            Box::new(Apply(
                Box::new(Var(x.clone())),
                Box::new(Lambda(x.clone(), Box::new(Var(x.clone()))))
            ))
        )
    );
}

#[test]
fn subst_case() {
    let z = Variable("z".into());
    let c = Constructor("C".into());
    let case = Case(
        Box::new(Var(z.clone())),
        vec![Branch(c.clone(), vec![z.clone()], Var(z.clone()))],
    );

    let result = substitute(
        &z,
        &Const(c.clone(), vec![Lambda(z.clone(), Box::new(Var(z.clone())))]),
        case,
    );

    assert_eq!(
        result,
        Case(
            Box::new(Const(
                c.clone(),
                vec![Lambda(z.clone(), Box::new(Var(z.clone())))]
            )),
            vec![Branch(c.clone(), vec![z.clone()], Var(z.clone()))]
        )
    );
}
