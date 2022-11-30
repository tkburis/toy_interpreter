use crate::expr::Expr;
use crate::token::Token;

pub enum Stmt {
    Block {
        statements: Vec<Stmt>,
    },
    Expression {
        expression: Expr,
    },
    Print {
        expression: Expr,
    },
    Var {
        name: Token,
        initializer: Option<Expr>,
    },
}

pub trait StmtVisitor<T, E> {
    fn accept_stmt(&mut self, stmt: &Stmt) -> Result<T, E> {
        match stmt {
            Stmt::Block { statements } => {
                self.visit_block_stmt(statements)
            },
            Stmt::Expression { expression } => {
                self.visit_expression_stmt(expression)
            },
            Stmt::Print { expression } => {
                self.visit_print_stmt(expression)
            },
            Stmt::Var { name, initializer } => {
                self.visit_var_stmt(name, initializer.as_ref())
            },
        }
    }

    fn visit_block_stmt(&mut self, statements: &[Stmt]) -> Result<T, E>;
    fn visit_expression_stmt(&mut self, expression: &Expr) -> Result<T, E>;
    fn visit_print_stmt(&mut self, expression: &Expr) -> Result<T, E>;
    fn visit_var_stmt(&mut self, name: &Token, initializer: Option<&Expr>) -> Result<T, E>;
}
