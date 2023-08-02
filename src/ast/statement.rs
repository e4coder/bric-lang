use crate::ast::block::Block;
use crate::ast::expression::Expr;
use crate::ast::value::Type;
use crate::ast::functions::{FunctionBlock, FunctionParam};
use crate::ast::functions::FunctionArg;

#[derive(PartialEq, Debug)]
pub enum Statement {
    Assignment {
        identifier: String,
        expression: Box<Expr>,
    },
    Definition {
        identifier: String,
        expression: Option<Box<Expr>>,
        value_type: Type,
    },
    Block(Block),
    Function {
        identifier: String,
        params: Option<Vec<FunctionParam>>,
        function_ret_type: Option<Type>,
        function_block: FunctionBlock,
    },
    FunctionCall {
        identifier: String,
        args: Option<Vec<FunctionArg>>,
    },
    ReturnStatement {
        expression: Option<Box<Expr>>,
    }
}

impl Statement {
    pub fn new_assignment(identifier: String, expression: Box<Expr>) -> Self {
        Self::Assignment {
            identifier,
            expression,
        }
    }

    pub fn new_definition(identifier: String, value_type: Type, expression: Option<Box<Expr>>) -> Self {
        Self::Definition {
            identifier,
            value_type,
            expression,
        }
    }

    pub fn new_block(block: Block) -> Self {
        Self::Block(block)
    }

    pub fn new_function(identifier: String, params: Option<Vec<FunctionParam>>, function_ret_type: Option<Type>, function_block: FunctionBlock) -> Self {
        Self::Function {
            identifier,
            params,
            function_ret_type,
            function_block,
        }
    }

    pub fn new_return_statement(expression: Option<Box<Expr>>) -> Self {
        Self::ReturnStatement { expression }
    }

    pub fn new_function_call (identifier: String, args: Option<Vec<FunctionArg>>) -> Self {
        Self::FunctionCall { identifier, args }
    }
}