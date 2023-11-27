use crate::{
    parser::{Branch, CodedLiteral, Constructor, Variable},
    Expr, MetaExpr,
};

pub struct StandardCoder {
    counter: usize;
}

impl StandardCoder {
    fn 
}

pub trait Coder {
    fn code_constructor(&mut self, c: Constructor) -> Expr;
    fn code_variable(&mut self, c: Variable) -> Expr;

    fn code_literal(&mut self, literal: CodedLiteral) -> Expr {}

    // TODO: add decoding methods (that would make it possible to pretty print using the coding literals)
}

/// Convert a meta expression to a Chi expression by replacing source representation nodes with constructor trees.
/// Constructor and variable names in the source repr nodes are replaced using the given `Repr`
pub fn to_expr<T: Coder>(expr: MetaExpr, coder: &mut T) -> Expr {
    match expr {
        MetaExpr::Apply(e1, e2) => {
            let e1 = to_expr(e1, coder);
            let e2 = to_expr(e2, coder);
            Expr::Apply(Box::new(e1), Box::new(e2))
        }
        MetaExpr::Lambda(x, e) => {
            let e = to_expr(*e, coder);
            Expr::Lambda(x, Box::new(e))
        }
        MetaExpr::Case(e, branches) => Expr::Case(
            Box::new(to_expr(*e, coder)),
            branches
                .into_iter()
                .map(|Branch(c, vars, expr)| Branch(c, vars, to_expr(expr, coder)))
                .collect(),
        ),
        MetaExpr::Rec(x, es) => Expr::Rec(x, to_expr(*es, coder)),
        MetaExpr::Var(x) => Expr::Var(x),
        MetaExpr::Const(c, es) => Expr::Const(c, es.into_iter().map(|e| to_expr(e, coder))),
        MetaExpr::Coded(literal) => coder.code_literal(literal),
    }
}
