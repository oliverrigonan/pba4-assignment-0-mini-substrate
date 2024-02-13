; ModuleID = 'probe5.4c37a2cf7e50b537-cgu.0'
source_filename = "probe5.4c37a2cf7e50b537-cgu.0"
target datalayout = "e-m:o-i64:64-i128:128-n32:64-S128"
target triple = "arm64-apple-macosx11.0.0"

@alloc_2825f8db424f8dfbfa629cc5dfc1d997 = private unnamed_addr constant <{ [75 x i8] }> <{ [75 x i8] c"/rustc/8ca44ef9caa4049d584fbbce218c219cdca33a2f/library/core/src/num/mod.rs" }>, align 1
@alloc_3f98cc44fa1d45dfd4a23045bdcececa = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_2825f8db424f8dfbfa629cc5dfc1d997, [16 x i8] c"K\00\00\00\00\00\00\00w\04\00\00\05\00\00\00" }>, align 8
@str.0 = internal constant [25 x i8] c"attempt to divide by zero"

; probe5::probe
; Function Attrs: uwtable
define void @_ZN6probe55probe17hfb3c64d84355f715E() unnamed_addr #0 {
start:
  %0 = call i1 @llvm.expect.i1(i1 false, i1 false)
  br i1 %0, label %panic.i, label %"_ZN4core3num21_$LT$impl$u20$u32$GT$10div_euclid17h192b2ca97541e0f7E.exit"

panic.i:                                          ; preds = %start
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17h0cc851e9bd47495cE(ptr align 1 @str.0, i64 25, ptr align 8 @alloc_3f98cc44fa1d45dfd4a23045bdcececa) #3
  unreachable

"_ZN4core3num21_$LT$impl$u20$u32$GT$10div_euclid17h192b2ca97541e0f7E.exit": ; preds = %start
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(none)
declare i1 @llvm.expect.i1(i1, i1) #1

; core::panicking::panic
; Function Attrs: cold noinline noreturn uwtable
declare void @_ZN4core9panicking5panic17h0cc851e9bd47495cE(ptr align 1, i64, ptr align 8) unnamed_addr #2

attributes #0 = { uwtable "frame-pointer"="non-leaf" "target-cpu"="apple-m1" }
attributes #1 = { nocallback nofree nosync nounwind willreturn memory(none) }
attributes #2 = { cold noinline noreturn uwtable "frame-pointer"="non-leaf" "target-cpu"="apple-m1" }
attributes #3 = { noreturn }

!llvm.module.flags = !{!0}

!0 = !{i32 8, !"PIC Level", i32 2}
