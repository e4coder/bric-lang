use crate::ast::block::Block;
use crate::ast::expression::Expr;
use crate::ast::value::Type;
use crate::ast::statement::Statement;

#[derive(PartialEq, Debug)]
pub enum FunctionBlock {
    Statements(Vec<Statement>),
}

impl FunctionBlock {
    pub fn new_function_block(statements: Vec<Statement>) -> Self {
        Self::Statements(statements)
    }
}

#[derive(PartialEq, Debug)]
pub enum FunctionParam {
    Param {
        identifier: String,
        param_type: Type,
    }
}

impl FunctionParam {
    pub fn new_param(identifier: String, param_type: Type) -> Self {
        Self::Param { identifier, param_type }
    }
}

#[derive(PartialEq, Debug)]
pub enum FunctionArg {
    Arg {
        expression: Box<Expr>,
    }
}

impl FunctionArg {
    pub fn new_arg(expression: Box<Expr>) -> Self {
        Self::Arg { expression }
    }
}