use std::{collections::HashMap, ops::RangeFrom};

use crate::{
    parser::{Branch, CodedLiteral, Constructor, Variable},
    Expr, MetaExpr, Program,
};

/// Convert all meta expression to a Chi expression in a program by replacing source representation nodes with constructor trees.
/// Constructor and variable names in the source repr nodes are replaced using the given `Coder`
pub fn replace_coded_literals<T: Coder>(
    program: Program<MetaExpr>,
    coder: &mut T,
) -> Program<Expr> {
    match program {
        Program::Let(var, expr, rest) => Program::Let(
            var,
            to_expr(expr, coder),
            Box::new(replace_coded_literals(*rest, coder)),
        ),
        Program::Expr(expr) => Program::Expr(to_expr(expr, coder)),
    }
}

fn to_expr<T: Coder>(expr: MetaExpr, coder: &mut T) -> Expr {
    match expr {
        MetaExpr::Apply(e1, e2) => {
            let e1 = to_expr(*e1, coder);
            let e2 = to_expr(*e2, coder);
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
        MetaExpr::Rec(x, es) => Expr::Rec(x, Box::new(to_expr(*es, coder))),
        MetaExpr::Var(x) => Expr::Var(x),
        MetaExpr::Const(c, es) => {
            Expr::Const(c, es.into_iter().map(|e| to_expr(e, coder)).collect())
        }
        MetaExpr::Coded(literal) => coder.code_literal(*literal),
    }
}

pub trait Coder {
    fn code_constructor(&mut self, c: Constructor) -> Expr;
    fn code_variable(&mut self, c: Variable) -> Expr;
    fn code_literal(&mut self, literal: CodedLiteral) -> Expr;
    fn code_natural(&mut self, n: usize) -> Expr;
    fn code_list<I>(&mut self, es: I) -> Expr
    where
        I: Iterator<Item = Expr>;

    // TODO: add decoding methods (that would make it possible to pretty print using the coding literals)
}

pub struct StandardCoder {
    counter: RangeFrom<usize>,
    previous_symbols: HashMap<String, usize>,
}

impl StandardCoder {
    fn new() -> Self {
        Self {
            counter: 0..,
            previous_symbols: HashMap::default(),
        }
    }
}

impl Default for StandardCoder {
    fn default() -> Self {
        Self::new()
    }
}

impl Coder for StandardCoder {
    fn code_constructor(&mut self, c: Constructor) -> Expr {
        let n = if let Some(n) = self.previous_symbols.get(&c.0) {
            *n
        } else {
            let n = self.counter.next().unwrap();
            self.previous_symbols.insert(c.0, n);
            n
        };

        self.code_natural(n)
    }

    fn code_variable(&mut self, v: Variable) -> Expr {
        let n = if let Some(n) = self.previous_symbols.get(&v.0) {
            *n
        } else {
            let n = self.counter.next().unwrap();
            self.previous_symbols.insert(v.0, n);
            n
        };

        self.code_natural(n)
    }

    fn code_literal(&mut self, CodedLiteral::Expr(expr): CodedLiteral) -> Expr {
        match expr {
            MetaExpr::Apply(e1, e2) => Expr::Const(
                "Apply".into(),
                vec![
                    // TODO: It's a bit ugly that we wrap everything in coded literals each recursive call,
                    // might want to clean that up later
                    self.code_literal(CodedLiteral::Expr(*e1)),
                    self.code_literal(CodedLiteral::Expr(*e2)),
                ],
            ),
            MetaExpr::Lambda(x, e) => Expr::Const(
                "Lambda".into(),
                vec![
                    self.code_variable(x),
                    self.code_literal(CodedLiteral::Expr(*e)),
                ],
            ),
            MetaExpr::Case(e, branches) => {
                let branches: Vec<_> = branches
                    .into_iter()
                    .map(|Branch(c, vars, expr)| {
                        let vars: Vec<_> = vars
                            .into_iter()
                            .map(|var| self.code_variable(var))
                            .collect();
                        let vars = self.code_list(vars.into_iter());

                        Expr::Const(
                            "Branch".into(),
                            vec![
                                self.code_constructor(c),
                                vars,
                                self.code_literal(CodedLiteral::Expr(expr)),
                            ],
                        )
                    })
                    .collect();
                let branches = self.code_list(branches.into_iter());

                Expr::Const(
                    "Case".into(),
                    vec![self.code_literal(CodedLiteral::Expr(*e)), branches],
                )
            }
            MetaExpr::Rec(x, e) => Expr::Const(
                "Rec".into(),
                vec![
                    self.code_variable(x),
                    self.code_literal(CodedLiteral::Expr(*e)),
                ],
            ),
            MetaExpr::Var(x) => Expr::Const("Var".into(), vec![self.code_variable(x)]),
            MetaExpr::Const(c, es) => {
                let es = es
                    .into_iter()
                    .map(|e| self.code_literal(CodedLiteral::Expr(e)))
                    .collect::<Vec<_>>();
                Expr::Const(
                    "Const".into(),
                    vec![self.code_constructor(c), self.code_list(es.into_iter())],
                )
            }
            MetaExpr::Coded(_) => panic!("Nested coded literals are not supported"),
        }
    }

    fn code_natural(&mut self, n: usize) -> Expr {
        if n == 0 {
            Expr::Const("Zero".into(), vec![])
        } else {
            Expr::Const("Suc".into(), vec![self.code_natural(n - 1)])
        }
    }

    fn code_list<I>(&mut self, mut es: I) -> Expr
    where
        I: Iterator<Item = Expr>,
    {
        if let Some(expr) = es.next() {
            Expr::Const("Cons".into(), vec![expr, self.code_list(es)])
        } else {
            Expr::Const("Nil".into(), vec![])
        }
    }
}
