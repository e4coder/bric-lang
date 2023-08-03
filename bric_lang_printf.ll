; ModuleID = 'example_module'
source_filename = "example_module"

@global_var = global i32 42
@pi = constant double 3.140000e+00
@hello = private unnamed_addr constant [12 x i8] c"Hello World\00", align 1
@.str = private unnamed_addr constant [3 x i8] c"%d\00", align 1

declare i32 @puts(i8)
declare i32 @printf(i8*, ...)

define i32 @main() {
entry:
  %global_var = load i32, ptr @global_var, align 4
  %global_var1 = add i32 %global_var, %global_var
  store i32 %global_var1, ptr @global_var, align 4
  %0 = load i32, i32* @global_var
  %1 = call i32 (i8*, ...) @printf(i8* noundef getelementptr inbounds ([3 x i8], [3 x i8]* @.str, i64 0, i64 0), i32 noundef %0)
  ret i32 0
}

define i32 @_start() {
  entry:
    %0 = call i32 @main() 
    ret i32 0
}