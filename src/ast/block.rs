use crate::ast::expression::Expr;
use crate::ast::statement::Statement;

#[derive(PartialEq, Debug)]
pub enum Block {
    StatementsBlock(Vec<Statement>),
    Condition {
        expression: Box<Expr>,
        then_block: Vec<Statement>,
        else_block: Option<Vec<Statement>>,
    },
    LoopBlock(Vec<Statement>),
    WhileBlock {
        expression: Box<Expr>,
        statements: Vec<Statement>,
    }
}

impl Block {
    pub fn new_statements(statements: Vec<Statement>) -> Self {
        Self::StatementsBlock(statements)
    }

    pub fn new_condition(
        expression: Box<Expr>,
        then_block: Vec<Statement>,
        else_block: Option<Vec<Statement>>,
    ) -> Self {
        Self::Condition {
            expression,
            then_block,
            else_block,
        }
    }

    pub fn new_loop(
        statements: Vec<Statement>,
    ) -> Self {
        Self::LoopBlock(statements)
    }

    pub fn new_while(
        expression: Box<Expr>,
        statements: Vec<Statement>
    ) -> Self {
        Self::WhileBlock {
            expression,
            statements,
        }
    }
}