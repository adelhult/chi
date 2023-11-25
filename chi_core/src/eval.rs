/// An interpreter for Chi.
/// Based on "Models of Computation: Section 6, An interpreter for χ in χ", by Bengt Nordström and Nils Anders Danielsson
/// and also the Agda specification: https://www.cse.chalmers.se/~nad/listings/chi/Chi.html
use crate::{
    parser::{Branch, Constructor, Variable},
    Expr, Program,
};
use Expr::*;

// NOTE: This is of course a very low limit, I should make it possible to set this dynamically when calling eval
const MAX_DEPTH: u32 = 250;

#[derive(Debug)]
pub enum Error {
    Crash(String), // TODO, replace with multiple variants, also keep track of the source position
}

fn lookup(const_name: &Constructor, branches: &[Branch]) -> Option<Branch> {
    branches
        .iter()
        .find(|Branch(c, ..)| c == const_name)
        .cloned()
}

fn substitute(var: &Variable, replacement: &Expr, expr: Expr) -> Expr {
    match expr {
        Apply(e1, e2) => Apply(
            Box::new(substitute(var, replacement, *e1)),
            Box::new(substitute(var, replacement, *e2)),
        ),
        Lambda(x, e) => {
            let e = if x == *var {
                // The name `var` is bound in this expression, so we stop substituting
                e
            } else {
                Box::new(substitute(var, replacement, *e))
            };

            Lambda(x, e)
        }
        Case(e, branches) => Case(
            Box::new(substitute(var, replacement, *e)),
            branches
                .into_iter()
                .map(|b| substitute_branch(var, replacement, b))
                .collect(),
        ),
        Rec(x, e) => {
            let e = if x == *var {
                // The name `var` is bound in this expression, so we stop substituting
                e
            } else {
                Box::new(substitute(var, replacement, *e))
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
                .map(|e| substitute(var, replacement, e))
                .collect(),
        ),
    }
}

fn substitute_branch(var: &Variable, replacement: &Expr, Branch(c, xs, e): Branch) -> Branch {
    // Check if the branch binds to the same variable name, if not we recursivly continue with the substitution
    if xs.contains(&var) {
        Branch(c, xs, e)
    } else {
        Branch(c, xs, substitute(var, replacement, e))
    }
}

// TODO: Convert a program to a single Chi expression using substitutin
fn substitute_program(var: &Variable, replacement: &Expr, program: Program) -> Program {
    match program {
        Program::Let(x, rhs, rest) => Program::Let(
            x,
            substitute(&var, replacement, rhs),
            Box::new(substitute_program(&var, replacement, *rest)),
        ),
        Program::Expr(expr) => Program::Expr(substitute(var, replacement, expr)),
    }
}

fn program_to_expr(program: Program) -> Expr {
    match program {
        Program::Let(var, rhs, rest) => program_to_expr(substitute_program(&var, &rhs, *rest)),
        Program::Expr(expr) => expr,
    }
}

pub fn eval(program: Program) -> Result<Expr, Error> {
    let expr = program_to_expr(program);
    eval_expr(expr, 0)
}

fn eval_expr(expr: Expr, depth: u32) -> Result<Expr, Error> {
    if depth >= MAX_DEPTH {
        return Err(Error::Crash(
            "Exceeded max depth, expression is assumed to not terminate".into(),
        ));
    }

    match expr {
        Apply(e1, e2) => {
            let Lambda(x, e) = eval_expr(*e1, depth + 1)? else {
                return Err(Error::Crash(
                    "LHS of application must be a lambda expression".into(),
                ));
            };
            eval_expr(substitute(&x, &eval_expr(*e2, depth + 1)?, *e), depth + 1)
        }
        Lambda(..) => Ok(expr),
        Case(e, branches) => {
            let Const(constructor_name, es) = eval_expr(*e, depth + 1)? else {
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
                .rfold(e, |e, (var, replacement)| substitute(var, &replacement, e));

            eval_expr(subst_expr, depth + 1)
        }
        Rec(x, e) => eval_expr(substitute(&x, &Rec(x.clone(), e.clone()), *e), depth + 1),
        Var(x) => Err(Error::Crash(format!(
            "Not a closed expression, variable '{x}' is not bound."
        ))),
        Const(c, es) => {
            let es: Result<Vec<_>, _> = es.into_iter().map(|e| eval_expr(e, depth + 1)).collect();
            Ok(Const(c, es?))
        }
    }
}
