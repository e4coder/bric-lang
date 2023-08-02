extern crate llvm_sys as llvm;
// #[macro_use]
// extern crate lalrpop_util;

use std::{fs, f32::consts, task::Context};

// mod ast;

fn main() {
    
    // let source_result = fs::read_to_string("test.bric");
    // let source = match source_result {
    //    Ok(s) => s,
    //    Err(_e) => r#"let x = "Could not load from test source";"#.to_string(), 
    // };

    // let program = ast::bric_lang::ProgramParser::new()
    //     .parse(&source)
    //     .expect("Unable to parse the program file");

    // for statement in program.items {
    //     println!("{:?}",statement)
    // }

    unsafe {
        let context = llvm::core::LLVMContextCreate();
        let module = llvm::core::LLVMModuleCreateWithName(b"example_module\0".as_ptr() as *const _);
        let builder = llvm::core::LLVMCreateBuilderInContext(context);

        let int_8_type = llvm::core::LLVMInt8TypeInContext(context);
        let int_8_type_ptr = llvm::core::LLVMPointerType(int_8_type, 0);
        let int_32_type = llvm::core::LLVMInt32TypeInContext(context);

        let gval = GlobalVariableDeclaration(context, module, builder);
        GlobalConstandDefinition(context, module, builder);
        
        let puts = PutsFunctionDeclaration(context, module, builder);

        MainFunctionDefinition(context, module, builder, puts.function_type, puts.function, gval);
        
        llvm::core::LLVMPrintModuleToFile(module, b"bric_lang.ll\0".as_ptr() as *const _, std::ptr::null_mut());

        llvm::core::LLVMDisposeBuilder(builder);
        llvm::core::LLVMDisposeModule(module);
        llvm::core::LLVMContextDispose(context);

    }

}


unsafe fn MainFunctionDefinition (
    context: *mut llvm::LLVMContext,
    module: *mut llvm::LLVMModule,
    builder: *mut llvm::LLVMBuilder,
    puts_function_type: *mut llvm::LLVMType,
    puts_function: *mut llvm::LLVMValue,
    gval: *mut llvm::LLVMValue,
) {
    let int_8_type = llvm::core::LLVMInt8TypeInContext(context);
    let int_8_type_ptr = llvm::core::LLVMPointerType(int_8_type, 0);
    let int_32_type = llvm::core::LLVMInt32TypeInContext(context);

    let main_function_type = llvm::core::LLVMFunctionType(int_32_type, std::ptr::null_mut(), 0, 0);
    let main_function = llvm::core::LLVMAddFunction(module, b"main\0".as_ptr() as *const _, main_function_type);

    let entry = llvm::core::LLVMAppendBasicBlockInContext(context, main_function, b"entry\0".as_ptr() as *const _);
    llvm::core::LLVMPositionBuilderAtEnd(builder, entry);



    let mut puts_function_args = llvm::core::LLVMBuildPointerCast(
        builder,
        llvm::core::LLVMBuildGlobalStringPtr(builder, b"Hello World\0".as_ptr() as *const _, b"hello\0".as_ptr() as *const _),
        int_8_type_ptr,
        b"0\0".as_ptr() as *const _
    );

    llvm::core::LLVMBuildCall2(builder, puts_function_type, puts_function, &mut puts_function_args, 1, b"i\0".as_ptr() as *const _);
    ReassignGlobalVariableDeclaration(context, module, builder, gval);
    llvm::core::LLVMBuildRet(builder, llvm::core::LLVMConstInt(int_32_type, 0, 0));

}

struct FunctionDeclarationReturn {
    function_type: *mut llvm::LLVMType,
    function: *mut llvm::LLVMValue,
}

unsafe fn PutsFunctionDeclaration  (
    context: *mut llvm::LLVMContext,
    module: *mut llvm::LLVMModule,
    builder: *mut llvm::LLVMBuilder
) -> FunctionDeclarationReturn {
    let int_8_type = llvm::core::LLVMInt8TypeInContext(context);
    let int_8_type_ptr = llvm::core::LLVMPointerType(int_8_type, 0);
    let int_32_type = llvm::core::LLVMInt32TypeInContext(context);

    let puts_function_type = llvm::core::LLVMFunctionType(int_32_type, &mut int_8_type.clone(), 1, 0);

    let puts_function = llvm::core::LLVMAddFunction(module, b"puts\0".as_ptr() as *const _, puts_function_type);

    FunctionDeclarationReturn { function_type: puts_function_type, function: puts_function }
}


unsafe fn GlobalVariableDeclaration (
    context: *mut llvm::LLVMContext,
    module: *mut llvm::LLVMModule,
    builder: *mut llvm::LLVMBuilder,
) -> *mut llvm::LLVMValue {
    let int_32_type = llvm::core::LLVMInt32TypeInContext(context);
    let gval = llvm::core::LLVMAddGlobal(module, int_32_type, b"global_var\0".as_ptr() as *const _);

    let val = llvm::core::LLVMConstInt(int_32_type, 42, 0);
    llvm::core::LLVMSetInitializer(gval, val);

    gval
}

unsafe fn GlobalConstandDefinition (
    context: *mut llvm::LLVMContext,
    module: *mut llvm::LLVMModule,
    builder: *mut llvm::LLVMBuilder,
) {
    let double_type = llvm::core::LLVMDoubleTypeInContext(context);
    let gconst = llvm::core::LLVMAddGlobal(module, double_type, b"pi\0".as_ptr() as *const _);
    let val = llvm::core::LLVMConstReal(double_type, 3.14);
    llvm::core::LLVMSetInitializer(gconst, val);
    llvm::core::LLVMSetGlobalConstant(gconst, 1);
}


unsafe fn ReassignGlobalVariableDeclaration (
    context: *mut llvm::LLVMContext,
    module: *mut llvm::LLVMModule,
    builder: *mut llvm::LLVMBuilder,
    gval: *mut llvm::LLVMValue,
) {
    let int_32_type = llvm::core::LLVMInt32TypeInContext(context);

    let newval = llvm::core::LLVMConstInt(int_32_type, 100, 0);
    llvm::core::LLVMSetInitializer(gval, newval);
}