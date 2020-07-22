; ModuleID = 'llvm 7.0.0, -scal -ftz 0, v2.15.11'
source_filename = "llvm 7.0.0, -scal -ftz 0, v2.15.11"
target triple = "x86_64-apple-darwin19.5.0"

%struct.dspSoundfile = type <{ float**, [256 x i32], [256 x i32], [256 x i32], i32 }>
%struct.dspmydsp = type <{ float, [2 x float], i32 }>
%struct.UIGlue = type <{ i8*, void (i8*, i8*)*, void (i8*, i8*)*, void (i8*, i8*)*, void (i8*)*, void (i8*, i8*, float*)*, void (i8*, i8*, float*)*, void (i8*, i8*, float*, float, float, float, float)*, void (i8*, i8*, float*, float, float, float, float)*, void (i8*, i8*, float*, float, float, float, float)*, void (i8*, i8*, float*, float, float)*, void (i8*, i8*, float*, float, float)*, void (i8*, i8*, i8*, %struct.dspSoundfile**)*, void (i8*, float*, i8*, i8*)* }>
%struct.MetaGlue = type <{ i8*, void (i8*, i8*, i8*)* }>

@defaultsound = internal global %struct.dspSoundfile* null
@author = internal constant [7 x i8] c"author\00"
@Grame = internal constant [6 x i8] c"Grame\00"
@"basics.lib/name" = internal constant [16 x i8] c"basics.lib/name\00"
@"Faust Basic Element Library" = internal constant [28 x i8] c"Faust Basic Element Library\00"
@"basics.lib/version" = internal constant [19 x i8] c"basics.lib/version\00"
@"0.0" = internal constant [4 x i8] c"0.0\00"
@copyright = internal constant [10 x i8] c"copyright\00"
@"(c)GRAME 2006" = internal constant [14 x i8] c"(c)GRAME 2006\00"
@filename = internal constant [9 x i8] c"filename\00"
@gain = internal constant [5 x i8] c"gain\00"
@license = internal constant [8 x i8] c"license\00"
@BSD = internal constant [4 x i8] c"BSD\00"
@name = internal constant [5 x i8] c"name\00"
@volume = internal constant [7 x i8] c"volume\00"
@"signals.lib/name" = internal constant [17 x i8] c"signals.lib/name\00"
@"Faust Signal Routing Library" = internal constant [29 x i8] c"Faust Signal Routing Library\00"
@"signals.lib/version" = internal constant [20 x i8] c"signals.lib/version\00"
@version = internal constant [8 x i8] c"version\00"
@"1.0" = internal constant [4 x i8] c"1.0\00"
@"1" = internal constant [2 x i8] c"1\00"
@0 = internal constant [1 x i8] zeroinitializer
@"0x00" = internal constant [5 x i8] c"0x00\00"
@"{\22name\22: \22volume\22,\22filename\22: \22gain\22,\22version\22: \222.15.11\22,\22compile_options\22: \22-scal -ftz 0\22,\22library_list\22: [\22/usr/local/share/Faust-2.15.11/share/faust/stdfaust.lib\22,\22/usr/local/share/Faust-2.15.11/share/faust/basics.lib\22,\22/usr/local/share/Faust-2.15.11/share/faust/signals.lib\22],\22include_pathnames\22: [\22/usr/local/share/Faust-2.15.11/share/faust\22,\22/usr/local/share/faust\22,\22/usr/share/faust\22,\22.\22,\22/Users/br-gaster/dev/wasm/audioplugin_wasm/plugins/gain\22],\22size\22: \2216\22,\22inputs\22: \221\22,\22outputs\22: \221\22,\22meta\22: [ { \22author\22: \22Grame\22 },{ \22basics.lib/name\22: \22Faust Basic Element Library\22 },{ \22basics.lib/version\22: \220.0\22 },{ \22copyright\22: \22(c)GRAME 2006\22 },{ \22filename\22: \22gain\22 },{ \22license\22: \22BSD\22 },{ \22name\22: \22volume\22 },{ \22signals.lib/name\22: \22Faust Signal Routing Library\22 },{ \22signals.lib/version\22: \220.0\22 },{ \22version\22: \221.0\22 }],\22ui\22: [ {\22type\22: \22vgroup\22,\22label\22: \22volume\22,\22items\22: [ {\22type\22: \22vslider\22,\22label\22: \220x00\22,\22address\22: \22/volume/0x00\22,\22meta\22: [{ \221\22: \22\22 }],\22init\22: \220\22,\22min\22: \22-70\22,\22max\22: \224\22,\22step\22: \220.1\22}]}]}" = internal constant [1015 x i8] c"{\22name\22: \22volume\22,\22filename\22: \22gain\22,\22version\22: \222.15.11\22,\22compile_options\22: \22-scal -ftz 0\22,\22library_list\22: [\22/usr/local/share/Faust-2.15.11/share/faust/stdfaust.lib\22,\22/usr/local/share/Faust-2.15.11/share/faust/basics.lib\22,\22/usr/local/share/Faust-2.15.11/share/faust/signals.lib\22],\22include_pathnames\22: [\22/usr/local/share/Faust-2.15.11/share/faust\22,\22/usr/local/share/faust\22,\22/usr/share/faust\22,\22.\22,\22/Users/br-gaster/dev/wasm/audioplugin_wasm/plugins/gain\22],\22size\22: \2216\22,\22inputs\22: \221\22,\22outputs\22: \221\22,\22meta\22: [ { \22author\22: \22Grame\22 },{ \22basics.lib/name\22: \22Faust Basic Element Library\22 },{ \22basics.lib/version\22: \220.0\22 },{ \22copyright\22: \22(c)GRAME 2006\22 },{ \22filename\22: \22gain\22 },{ \22license\22: \22BSD\22 },{ \22name\22: \22volume\22 },{ \22signals.lib/name\22: \22Faust Signal Routing Library\22 },{ \22signals.lib/version\22: \220.0\22 },{ \22version\22: \221.0\22 }],\22ui\22: [ {\22type\22: \22vgroup\22,\22label\22: \22volume\22,\22items\22: [ {\22type\22: \22vslider\22,\22label\22: \220x00\22,\22address\22: \22/volume/0x00\22,\22meta\22: [{ \221\22: \22\22 }],\22init\22: \220\22,\22min\22: \22-70\22,\22max\22: \224\22,\22step\22: \220.1\22}]}]}\00"

declare void @free(i8*) align 2

define void @destroymydsp(%struct.dspmydsp* %dsp) align 2 {
entry_block:
  br label %return_block

return_block:                                     ; preds = %entry_block
  ret void
}

define void @deletemydsp(%struct.dspmydsp* %dsp) {
entry:
  %0 = bitcast %struct.dspmydsp* %dsp to i8*
  call void @destroymydsp(%struct.dspmydsp* %dsp)
  call void @free(i8* %0)
  ret void
}

declare i8* @calloc(i64, i64)

define void @allocatemydsp(%struct.dspmydsp* %dsp) align 2 {
entry_block:
  br label %return_block

return_block:                                     ; preds = %entry_block
  ret void
}

define %struct.dspmydsp* @newmydsp() {
entry:
  %0 = call i8* @calloc(i64 1, i64 16)
  %1 = bitcast i8* %0 to %struct.dspmydsp*
  call void @allocatemydsp(%struct.dspmydsp* %1)
  ret %struct.dspmydsp* %1
}

define void @buildUserInterfacemydsp(%struct.dspmydsp* %dsp, %struct.UIGlue* %interface) align 2 {
init:
  %0 = getelementptr inbounds %struct.UIGlue, %struct.UIGlue* %interface, i64 0, i32 0
  %1 = load i8*, i8** %0
  br label %code_block

code_block:                                       ; preds = %init
  %2 = getelementptr inbounds %struct.UIGlue, %struct.UIGlue* %interface, i64 0, i32 3
  %3 = load void (i8*, i8*)*, void (i8*, i8*)** %2
  call void %3(i8* %1, i8* getelementptr inbounds ([7 x i8], [7 x i8]* @volume, i32 0, i32 0))
  %4 = getelementptr inbounds %struct.UIGlue, %struct.UIGlue* %interface, i64 0, i32 13
  %5 = load void (i8*, float*, i8*, i8*)*, void (i8*, float*, i8*, i8*)** %4
  %6 = getelementptr inbounds %struct.dspmydsp, %struct.dspmydsp* %dsp, i32 0, i32 0
  call void %5(i8* %1, float* %6, i8* getelementptr inbounds ([2 x i8], [2 x i8]* @"1", i32 0, i32 0), i8* getelementptr inbounds ([1 x i8], [1 x i8]* @0, i32 0, i32 0))
  %7 = getelementptr inbounds %struct.UIGlue, %struct.UIGlue* %interface, i64 0, i32 7
  %8 = load void (i8*, i8*, float*, float, float, float, float)*, void (i8*, i8*, float*, float, float, float, float)** %7
  %9 = getelementptr inbounds %struct.dspmydsp, %struct.dspmydsp* %dsp, i32 0, i32 0
  call void %8(i8* %1, i8* getelementptr inbounds ([5 x i8], [5 x i8]* @"0x00", i32 0, i32 0), float* %9, float 0.000000e+00, float -7.000000e+01, float 4.000000e+00, float 0x3FB99999A0000000)
  %10 = getelementptr inbounds %struct.UIGlue, %struct.UIGlue* %interface, i64 0, i32 4
  %11 = load void (i8*)*, void (i8*)** %10
  call void %11(i8* %1)
  br label %return_block

return_block:                                     ; preds = %code_block
  ret void
}

define i32 @getSampleRatemydsp(%struct.dspmydsp* %dsp) {
entry_block:
  %0 = getelementptr inbounds %struct.dspmydsp, %struct.dspmydsp* %dsp, i32 0, i32 2
  %1 = load i32, i32* %0
  ret i32 %1
}

; Function Attrs: nounwind
declare float @__exp10f(float) #0

; Function Attrs: nounwind readnone
define float @exp10f(float %val0) #1 {
entry_block:
  br label %code_block

code_block:                                       ; preds = %entry_block
  %0 = call fast float @__exp10f(float %val0)
  ret float %0
}

; Function Attrs: nounwind
define i32 @getNumInputsmydsp(%struct.dspmydsp* %dsp) #0 {
entry_block:
  br label %code_block

code_block:                                       ; preds = %entry_block
  ret i32 1
}

; Function Attrs: nounwind
define i32 @getNumOutputsmydsp(%struct.dspmydsp* %dsp) #0 {
entry_block:
  br label %code_block

code_block:                                       ; preds = %entry_block
  ret i32 1
}

; Function Attrs: nounwind
define i32 @getInputRatemydsp(%struct.dspmydsp* %dsp, i32 %channel) #0 {
entry_block:
  %rate = alloca i32
  br label %code_block

code_block:                                       ; preds = %entry_block
  br label %init_block

init_block:                                       ; preds = %code_block
  switch i32 %channel, label %default [
    i32 0, label %case
  ]

exit_block:                                       ; preds = %code_block2, %code_block1
  %0 = load i32, i32* %rate
  ret i32 %0

default:                                          ; preds = %init_block
  br label %code_block2

case:                                             ; preds = %init_block
  br label %code_block1

code_block1:                                      ; preds = %case
  store i32 1, i32* %rate
  br label %exit_block

code_block2:                                      ; preds = %default
  store i32 -1, i32* %rate
  br label %exit_block
}

; Function Attrs: nounwind
define i32 @getOutputRatemydsp(%struct.dspmydsp* %dsp, i32 %channel) #0 {
entry_block:
  %rate = alloca i32
  br label %code_block

code_block:                                       ; preds = %entry_block
  br label %init_block

init_block:                                       ; preds = %code_block
  switch i32 %channel, label %default [
    i32 0, label %case
  ]

exit_block:                                       ; preds = %code_block2, %code_block1
  %0 = load i32, i32* %rate
  ret i32 %0

default:                                          ; preds = %init_block
  br label %code_block2

case:                                             ; preds = %init_block
  br label %code_block1

code_block1:                                      ; preds = %case
  store i32 1, i32* %rate
  br label %exit_block

code_block2:                                      ; preds = %default
  store i32 -1, i32* %rate
  br label %exit_block
}

; Function Attrs: nounwind readnone
declare float @powf(float, float) #1

define void @classInitmydsp(i32 %samplingFreq) {
entry_block:
  br label %return_block

return_block:                                     ; preds = %entry_block
  ret void
}

define void @instanceResetUserInterfacemydsp(%struct.dspmydsp* %dsp) {
entry_block:
  br label %code_block

code_block:                                       ; preds = %entry_block
  %0 = getelementptr inbounds %struct.dspmydsp, %struct.dspmydsp* %dsp, i32 0, i32 0
  store float 0.000000e+00, float* %0
  br label %return_block

return_block:                                     ; preds = %code_block
  ret void
}

define void @instanceClearmydsp(%struct.dspmydsp* %dsp) {
entry_block:
  br label %code_block

code_block:                                       ; preds = %entry_block
  br label %init_block

init_block:                                       ; preds = %code_block
  %l0 = alloca i32
  store i32 0, i32* %l0
  br label %test_block

test_block:                                       ; preds = %code_block2, %init_block
  %l01 = phi i32 [ 0, %init_block ], [ %next_index, %code_block2 ]
  %0 = load i32, i32* %l0
  %1 = icmp slt i32 %0, 2
  %2 = select i1 %1, i32 1, i32 0
  %3 = trunc i32 %2 to i1
  br i1 %3, label %loop_body_block, label %exit_block

loop_body_block:                                  ; preds = %test_block
  br label %code_block2

exit_block:                                       ; preds = %test_block
  br label %return_block

code_block2:                                      ; preds = %loop_body_block
  %4 = load i32, i32* %l0
  %5 = getelementptr inbounds %struct.dspmydsp, %struct.dspmydsp* %dsp, i64 0, i32 1
  %6 = getelementptr inbounds [2 x float], [2 x float]* %5, i64 0, i64 0
  %7 = getelementptr inbounds float, float* %6, i32 %4
  store float 0.000000e+00, float* %7
  %8 = load i32, i32* %l0
  %next_index = add i32 %8, 1
  store i32 %next_index, i32* %l0
  br label %test_block

return_block:                                     ; preds = %exit_block
  ret void
}

define void @instanceConstantsmydsp(%struct.dspmydsp* %dsp, i32 %samplingFreq) {
entry_block:
  br label %code_block

code_block:                                       ; preds = %entry_block
  %0 = getelementptr inbounds %struct.dspmydsp, %struct.dspmydsp* %dsp, i32 0, i32 2
  store i32 %samplingFreq, i32* %0
  br label %return_block

return_block:                                     ; preds = %code_block
  ret void
}

define void @metadatamydsp(%struct.MetaGlue* %m) {
entry_block:
  %0 = getelementptr %struct.MetaGlue, %struct.MetaGlue* %m, i64 0, i32 0
  %1 = load i8*, i8** %0
  %2 = getelementptr %struct.MetaGlue, %struct.MetaGlue* %m, i64 0, i32 1
  %3 = load void (i8*, i8*, i8*)*, void (i8*, i8*, i8*)** %2
  call void %3(i8* %1, i8* getelementptr inbounds ([7 x i8], [7 x i8]* @author, i32 0, i32 0), i8* getelementptr inbounds ([6 x i8], [6 x i8]* @Grame, i32 0, i32 0))
  call void %3(i8* %1, i8* getelementptr inbounds ([16 x i8], [16 x i8]* @"basics.lib/name", i32 0, i32 0), i8* getelementptr inbounds ([28 x i8], [28 x i8]* @"Faust Basic Element Library", i32 0, i32 0))
  call void %3(i8* %1, i8* getelementptr inbounds ([19 x i8], [19 x i8]* @"basics.lib/version", i32 0, i32 0), i8* getelementptr inbounds ([4 x i8], [4 x i8]* @"0.0", i32 0, i32 0))
  call void %3(i8* %1, i8* getelementptr inbounds ([10 x i8], [10 x i8]* @copyright, i32 0, i32 0), i8* getelementptr inbounds ([14 x i8], [14 x i8]* @"(c)GRAME 2006", i32 0, i32 0))
  call void %3(i8* %1, i8* getelementptr inbounds ([9 x i8], [9 x i8]* @filename, i32 0, i32 0), i8* getelementptr inbounds ([5 x i8], [5 x i8]* @gain, i32 0, i32 0))
  call void %3(i8* %1, i8* getelementptr inbounds ([8 x i8], [8 x i8]* @license, i32 0, i32 0), i8* getelementptr inbounds ([4 x i8], [4 x i8]* @BSD, i32 0, i32 0))
  call void %3(i8* %1, i8* getelementptr inbounds ([5 x i8], [5 x i8]* @name, i32 0, i32 0), i8* getelementptr inbounds ([7 x i8], [7 x i8]* @volume, i32 0, i32 0))
  call void %3(i8* %1, i8* getelementptr inbounds ([17 x i8], [17 x i8]* @"signals.lib/name", i32 0, i32 0), i8* getelementptr inbounds ([29 x i8], [29 x i8]* @"Faust Signal Routing Library", i32 0, i32 0))
  call void %3(i8* %1, i8* getelementptr inbounds ([20 x i8], [20 x i8]* @"signals.lib/version", i32 0, i32 0), i8* getelementptr inbounds ([4 x i8], [4 x i8]* @"0.0", i32 0, i32 0))
  call void %3(i8* %1, i8* getelementptr inbounds ([8 x i8], [8 x i8]* @version, i32 0, i32 0), i8* getelementptr inbounds ([4 x i8], [4 x i8]* @"1.0", i32 0, i32 0))
  br label %return_block

return_block:                                     ; preds = %entry_block
  ret void
}

define i8* @getJSONmydsp() {
return_block:
  ret i8* getelementptr inbounds ([1015 x i8], [1015 x i8]* @"{\22name\22: \22volume\22,\22filename\22: \22gain\22,\22version\22: \222.15.11\22,\22compile_options\22: \22-scal -ftz 0\22,\22library_list\22: [\22/usr/local/share/Faust-2.15.11/share/faust/stdfaust.lib\22,\22/usr/local/share/Faust-2.15.11/share/faust/basics.lib\22,\22/usr/local/share/Faust-2.15.11/share/faust/signals.lib\22],\22include_pathnames\22: [\22/usr/local/share/Faust-2.15.11/share/faust\22,\22/usr/local/share/faust\22,\22/usr/share/faust\22,\22.\22,\22/Users/br-gaster/dev/wasm/audioplugin_wasm/plugins/gain\22],\22size\22: \2216\22,\22inputs\22: \221\22,\22outputs\22: \221\22,\22meta\22: [ { \22author\22: \22Grame\22 },{ \22basics.lib/name\22: \22Faust Basic Element Library\22 },{ \22basics.lib/version\22: \220.0\22 },{ \22copyright\22: \22(c)GRAME 2006\22 },{ \22filename\22: \22gain\22 },{ \22license\22: \22BSD\22 },{ \22name\22: \22volume\22 },{ \22signals.lib/name\22: \22Faust Signal Routing Library\22 },{ \22signals.lib/version\22: \220.0\22 },{ \22version\22: \221.0\22 }],\22ui\22: [ {\22type\22: \22vgroup\22,\22label\22: \22volume\22,\22items\22: [ {\22type\22: \22vslider\22,\22label\22: \220x00\22,\22address\22: \22/volume/0x00\22,\22meta\22: [{ \221\22: \22\22 }],\22init\22: \220\22,\22min\22: \22-70\22,\22max\22: \224\22,\22step\22: \220.1\22}]}]}", i32 0, i32 0)
}

define void @computemydsp(%struct.dspmydsp* %dsp, i32 %count, float** %inputs, float** %outputs) {
entry_block:
  %i = alloca i32
  %fSlow0 = alloca float
  %output0 = alloca float*
  %input0 = alloca float*
  br label %code_block

code_block:                                       ; preds = %entry_block
  %0 = getelementptr inbounds float*, float** %inputs, i32 0
  %1 = load float*, float** %0
  store float* %1, float** %input0
  %2 = getelementptr inbounds float*, float** %outputs, i32 0
  %3 = load float*, float** %2
  store float* %3, float** %output0
  %4 = getelementptr inbounds %struct.dspmydsp, %struct.dspmydsp* %dsp, i32 0, i32 0
  %5 = load float, float* %4
  %6 = fmul fast float 0x3FA99999A0000000, %5
  %7 = call fast float @powf(float 1.000000e+01, float %6)
  %8 = fmul fast float 0x3F50624DE0000000, %7
  store float %8, float* %fSlow0
  br label %init_block

init_block:                                       ; preds = %code_block
  store i32 0, i32* %i
  br label %test_block

test_block:                                       ; preds = %code_block2, %init_block
  %i1 = phi i32 [ 0, %init_block ], [ %next_index, %code_block2 ]
  %9 = load i32, i32* %i
  %10 = icmp slt i32 %9, %count
  %11 = select i1 %10, i32 1, i32 0
  %12 = trunc i32 %11 to i1
  br i1 %12, label %loop_body_block, label %exit_block

loop_body_block:                                  ; preds = %test_block
  br label %code_block2

exit_block:                                       ; preds = %test_block
  br label %return_block

code_block2:                                      ; preds = %loop_body_block
  %13 = getelementptr inbounds %struct.dspmydsp, %struct.dspmydsp* %dsp, i64 0, i32 1
  %14 = getelementptr inbounds [2 x float], [2 x float]* %13, i64 0, i64 0
  %15 = getelementptr inbounds float, float* %14, i32 0
  %16 = load float, float* %fSlow0
  %17 = getelementptr inbounds %struct.dspmydsp, %struct.dspmydsp* %dsp, i64 0, i32 1
  %18 = getelementptr inbounds [2 x float], [2 x float]* %17, i64 0, i64 0
  %19 = getelementptr inbounds float, float* %18, i32 1
  %20 = load float, float* %19
  %21 = fmul fast float 0x3FEFF7CEE0000000, %20
  %22 = fadd fast float %16, %21
  store float %22, float* %15
  %23 = load i32, i32* %i
  %24 = load float*, float** %output0
  %25 = getelementptr inbounds float, float* %24, i32 %23
  %26 = load i32, i32* %i
  %27 = load float*, float** %input0
  %28 = getelementptr inbounds float, float* %27, i32 %26
  %29 = load float, float* %28
  %30 = getelementptr inbounds %struct.dspmydsp, %struct.dspmydsp* %dsp, i64 0, i32 1
  %31 = getelementptr inbounds [2 x float], [2 x float]* %30, i64 0, i64 0
  %32 = getelementptr inbounds float, float* %31, i32 0
  %33 = load float, float* %32
  %34 = fmul fast float %29, %33
  store float %34, float* %25
  %35 = getelementptr inbounds %struct.dspmydsp, %struct.dspmydsp* %dsp, i64 0, i32 1
  %36 = getelementptr inbounds [2 x float], [2 x float]* %35, i64 0, i64 0
  %37 = getelementptr inbounds float, float* %36, i32 1
  %38 = getelementptr inbounds %struct.dspmydsp, %struct.dspmydsp* %dsp, i64 0, i32 1
  %39 = getelementptr inbounds [2 x float], [2 x float]* %38, i64 0, i64 0
  %40 = getelementptr inbounds float, float* %39, i32 0
  %41 = load float, float* %40
  store float %41, float* %37
  %42 = load i32, i32* %i
  %next_index = add i32 %42, 1
  store i32 %next_index, i32* %i
  br label %test_block

return_block:                                     ; preds = %exit_block
  ret void
}

define void @instanceInitmydsp(%struct.dspmydsp* %dsp, i32 %samplingFreq) {
entry_block:
  call void @instanceConstantsmydsp(%struct.dspmydsp* %dsp, i32 %samplingFreq)
  call void @instanceResetUserInterfacemydsp(%struct.dspmydsp* %dsp)
  call void @instanceClearmydsp(%struct.dspmydsp* %dsp)
  ret void
}

define void @initmydsp(%struct.dspmydsp* %dsp, i32 %samplingFreq) {
entry_block:
  call void @classInitmydsp(i32 %samplingFreq)
  call void @instanceInitmydsp(%struct.dspmydsp* %dsp, i32 %samplingFreq)
  ret void
}

define void @setDefaultSoundmydsp(%struct.dspSoundfile* %default_sound) {
entry_block:
  store %struct.dspSoundfile* %default_sound, %struct.dspSoundfile** @defaultsound
  ret void
}

attributes #0 = { nounwind }
attributes #1 = { nounwind readnone }
