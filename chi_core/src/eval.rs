/// An interpreter for Chi.
/// Based on "Models of Computation: Section 6, An interpreter for χ in χ", by Bengt Nordström and Nils Anders Danielsson
/// and also the Agda specification: https://www.cse.chalmers.se/~nad/listings/chi/Chi.html
use crate::{
    parser::{Branch, ConstName, VarName},
    Expr,
};
use Expr::*;

// NOTE: This is of course a very low limit, I should make it possible to set this dynamically when calling eval
const MAX_DEPTH: u32 = 250;

#[derive(Debug)]
pub enum Error {
    Crash(String), // TODO, replace with multiple variants, also keep track of the source position
}

fn substitute(var: &VarName, expr: Expr, replacement: &Expr) -> Expr {
    match expr {
        Apply(e1, e2) => Apply(
            Box::new(substitute(var, *e1, replacement)),
            Box::new(substitute(var, *e2, replacement)),
        ),
        Lambda(x, e) => {
            let e = if x == *var {
                // The name `var` is bound in this expression, so we stop substituting
                e
            } else {
                Box::new(substitute(var, *e, replacement))
            };

            Lambda(x, e)
        }
        Case(e, branches) => Case(
            Box::new(substitute(var, *e, replacement)),
            branches
                .into_iter()
                .map(|b| substitute_branch(var, b, replacement))
                .collect(),
        ),
        Rec(x, e) => {
            let e = if x == *var {
                // The name `var` is bound in this expression, so we stop substituting
                e
            } else {
                Box::new(substitute(var, *e, replacement))
            };

            Rec(x, e)
        }
        Var(x) => {
            if x == *var {
                replacement.clone()
            } else {
                Var(x)
            }
        }
        Const(c, es) => Const(
            c,
            es.into_iter()
                .map(|e| substitute(var, e, replacement))
                .collect(),
        ),
    }
}

fn substitute_branch(var: &VarName, Branch(c, xs, e): Branch, replacement: &Expr) -> Branch {
    // Check if the branch binds to the same variable name, if not we recursivly continue with the substitution
    if xs.contains(&var) {
        Branch(c, xs, e)
    } else {
        Branch(c, xs, substitute(var, e, replacement))
    }
}

fn lookup(const_name: &ConstName, branches: &[Branch]) -> Option<Branch> {
    branches
        .iter()
        .find(|Branch(c, ..)| c == const_name)
        .cloned()
}

pub fn eval(expr: Expr) -> Result<Expr, Error> {
    eval_impl(expr, 0)
}

fn eval_impl(expr: Expr, depth: u32) -> Result<Expr, Error> {
    if depth >= MAX_DEPTH {
        return Err(Error::Crash(
            "Exceeded max depth, expression is assumed to not terminate".into(),
        ));
    }

    match expr {
        Apply(e1, e2) => {
            let Lambda(x, e) = eval_impl(*e1, depth + 1)? else {
                return Err(Error::Crash(
                    "LHS of application must be a lambda expression".into(),
                ));
            };
            eval_impl(substitute(&x, eval_impl(*e2, depth + 1)?, &e), depth + 1)
        }
        Lambda(..) => Ok(expr),
        Case(e, branches) => {
            let Const(constructor_name, es) = eval_impl(*e, depth + 1)? else {
                return Err(Error::Crash(
                    "Expected constructor in case expression".into(),
                ));
            };

            let Some(Branch(_, xs, e)) = lookup(&constructor_name, &branches) else {
                return Err(Error::Crash("No matching constructor name".into()));
            };

            // Ensure that xs and es are the same arity
            if xs.len() != es.len() {
                return Err(Error::Crash(
                    "Constructor application in branch has wrong arity".into(),
                ));
            }

            let subst_expr = xs
                .iter()
                .zip(es)
                .rfold(e, |e, (var, replacement)| substitute(var, e, &replacement));

            eval_impl(subst_expr, depth + 1)
        }
        Rec(x, e) => eval_impl(substitute(&x, Rec(x.clone(), e.clone()), &e), depth + 1),
        Var(x) => Err(Error::Crash(format!(
            "Not a closed expression, variable '{x}' is not bound."
        ))),
        Const(c, es) => {
            let es: Result<Vec<_>, _> = es.into_iter().map(|e| eval_impl(e, depth + 1)).collect();
            Ok(Const(c, es?))
        }
    }
}
