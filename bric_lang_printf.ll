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





; STORE AN ARRAY





; ModuleID = 'example_module'
source_filename = "example_module"

@global_var = global i32 42
@pi = constant double 3.140000e+00
@hello = private unnamed_addr constant [12 x i8] c"Hello World\00", align 1
@0 = private unnamed_addr constant [21 x i8] c"Hello, world %d %d!\0A\00", align 1

declare i32 @puts(i8)

declare i32 @printf(i8, ...)

define i32 @main() {
entry:
  %local = alloca i32, align 4
  store i32 400, ptr %local, align 4
  %0 = load i32, ptr %local, align 4
  %allocaArray = alloca [3 x i32], align 4
  store [3 x i32] [i32 133, i32 32, i32 1000], ptr %allocaArray, align 4
  %1 = load i32, ptr getelementptr ([3 x i32], ptr %allocaArray, i32 1), align 4
  %t1 = call i32 @printf(ptr @0, i32 %0, i32 %1)
  %i = call i32 @puts(ptr @hello)
  ret i32 0
}











; CUSTOM FUNCTION CALL


; ModuleID = 'example_module'
source_filename = "example_module"

@global_var = global i32 42
@pi = constant double 3.140000e+00
@hello = private unnamed_addr constant [12 x i8] c"Hello World\00", align 1
@0 = private unnamed_addr constant [21 x i8] c"Hello, world %d %d!\0A\00", align 1

declare i32 @puts(i8)

declare i32 @printf(i8, ...)

define i32 @test() {
entry:
  %i = call i32 @puts(ptr @hello)
  ret i32 456
}

define i32 @main() {
entry:
  %local = alloca i32, align 4
  store i32 400, ptr %local, align 4
  %0 = load i32, ptr %local, align 4
  %allocaArray = alloca [3 x i32], align 4
  store [3 x i32] [i32 133, i32 32, i32 1000], ptr %allocaArray, align 4
  %allocaArray.0 = alloca [3 x i8], align 1
  store [3 x i8] [i8 23, i8 11, i8 2], ptr %allocaArray.0, align 1
  %elem = getelementptr i32, ptr %allocaArray, i32 0
  %elem.0 = getelementptr i32, ptr %allocaArray, i32 1
  %elem.1 = getelementptr i32, ptr %allocaArray, i32 2
  %elem.2 = getelementptr i8, ptr %allocaArray.0, i32 0
  %elem.3 = getelementptr i8, ptr %allocaArray.0, i32 1
  %elem.4 = getelementptr i8, ptr %allocaArray.0, i32 2
  %1 = load i32, ptr %elem, align 4
  %t1 = call i32 @printf(ptr @0, i32 %0, i32 %1)
  %t1.0 = call i32 @printf(ptr @0, i32 %0, ptr %elem)
  %t1.1 = call i32 @printf(ptr @0, i32 %0, ptr %elem.0)
  %t1.2 = call i32 @printf(ptr @0, i32 %0, ptr %elem.1)
  %t1.3 = call i32 @printf(ptr @0, i32 %0, ptr %allocaArray)
  %t1.4 = call i32 @printf(ptr @0, i32 %0, ptr %elem.2)
  %t1.5 = call i32 @printf(ptr @0, i32 %0, ptr %elem.3)
  %t1.6 = call i32 @printf(ptr @0, i32 %0, ptr %elem.4)
  %t1.7 = call i32 @printf(ptr @0, i32 %0, ptr %allocaArray.0)
  %i = call i32 @puts(ptr @hello)
  %i.0 = call i32 @test()
  %t1.8 = call i32 @printf(ptr @0, i32 %0, i32 %i.0)

  ret i32 0
}
