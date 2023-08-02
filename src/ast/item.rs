use crate::ast::block::Block;
use crate::ast::expression::Expr;
use crate::ast::value::Type;
use crate::ast::functions::{FunctionBlock, FunctionParam};

#[derive(PartialEq, Debug)]
pub enum Item {
    Struct {
        identifier: String,
        struct_def: String,
    },
    Function {
        identifier: String,
        params: Option<Vec<FunctionParam>>,
        function_ret_type: Option<Type>,
        function_block: FunctionBlock,
    },
}


impl Item {
    pub fn new_struct(identifier: String, struct_def: String) -> Self {
        Self::Struct {
            identifier,
            struct_def,
        }
    }
    pub fn new_function(identifier: String, params: Option<Vec<FunctionParam>>, function_ret_type: Option<Type>, function_block: FunctionBlock) -> Self {
        Self::Function {
            identifier,
            params,
            function_ret_type,
            function_block,
        }
    }
}