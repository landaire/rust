//@ add-minicore
//@ compile-flags: --target={{src-base}}/xbox360-abi.json -Zunstable-options -Zmerge-functions=disabled
//@ compile-flags: -Ctarget-feature=+altivec
//@ needs-llvm-components: powerpc
// This target uses LLVM's existing PPC64 ILP32 triple so the frontend classifier
// can be tested before the Xbox 360 triple reaches Rust's bundled LLVM.

#![crate_type = "lib"]
#![feature(no_core, lang_items, repr_simd)]
#![no_core]
#![allow(improper_ctypes_definitions)]

extern crate minicore;

#[repr(C)]
pub struct S1 {
    value: u8,
}

#[repr(C)]
pub struct S2 {
    value: u16,
}

#[repr(C)]
pub struct S3 {
    value: [u8; 3],
}

#[repr(C)]
pub struct S4 {
    value: u32,
}

#[repr(C)]
pub struct F4 {
    value: f32,
}

#[repr(C)]
pub struct S5 {
    value: [u8; 5],
}

#[repr(C)]
pub struct S8 {
    value: [u8; 8],
}

#[repr(C)]
pub struct D8 {
    value: f64,
}

#[repr(C)]
pub struct S12 {
    value: [u32; 3],
}

#[repr(simd)]
pub struct F32x4([f32; 4]);

#[repr(simd)]
pub struct I32x4([i32; 4]);

#[repr(simd)]
pub struct U8x16([u8; 16]);

#[repr(simd)]
pub struct F32x2([f32; 2]);

#[repr(simd)]
pub struct F64x2([f64; 2]);

#[repr(simd)]
pub struct I64x2([i64; 2]);

#[repr(simd)]
pub struct F32x8([f32; 8]);

#[repr(transparent)]
pub struct TransparentF32x4(F32x4);

#[repr(C)]
pub struct WrapF32x4 {
    value: F32x4,
}

// CHECK-LABEL: define{{.*}} i8 @return_s1()
#[no_mangle]
pub extern "C" fn return_s1() -> S1 {
    S1 { value: 1 }
}

// CHECK-LABEL: define{{.*}} i16 @return_s2()
#[no_mangle]
pub extern "C" fn return_s2() -> S2 {
    S2 { value: 1 }
}

// CHECK-LABEL: define{{.*}} void @return_s3(
// CHECK-SAME: ptr{{.*}}sret([3 x i8])
#[no_mangle]
pub extern "C" fn return_s3() -> S3 {
    S3 { value: [1; 3] }
}

// CHECK-LABEL: define{{.*}} i32 @return_s4()
#[no_mangle]
pub extern "C" fn return_s4() -> S4 {
    S4 { value: 1 }
}

// CHECK-LABEL: define{{.*}} i32 @return_f4()
#[no_mangle]
pub extern "C" fn return_f4() -> F4 {
    F4 { value: 1.0 }
}

// CHECK-LABEL: define{{.*}} void @return_s5(
// CHECK-SAME: ptr{{.*}}sret([5 x i8])
#[no_mangle]
pub extern "C" fn return_s5() -> S5 {
    S5 { value: [1; 5] }
}

// CHECK-LABEL: define{{.*}} void @identity_s5(
// CHECK-SAME: ptr{{.*}}sret([5 x i8]){{.*}}, i64
#[no_mangle]
pub extern "C" fn identity_s5(value: S5) -> S5 {
    value
}

// CHECK-LABEL: define{{.*}} i64 @return_s8()
#[no_mangle]
pub extern "C" fn return_s8() -> S8 {
    S8 { value: [1; 8] }
}

// CHECK-LABEL: define{{.*}} i64 @return_d8()
#[no_mangle]
pub extern "C" fn return_d8() -> D8 {
    D8 { value: 1.0 }
}

// CHECK-LABEL: define{{.*}} void @return_s12(
// CHECK-SAME: ptr{{.*}}sret([12 x i8])
#[no_mangle]
pub extern "C" fn return_s12() -> S12 {
    S12 { value: [1; 3] }
}

// CHECK-LABEL: define{{.*}} void @take_s1(i8
#[no_mangle]
pub extern "C" fn take_s1(value: S1) {
    let _ = value;
}

// CHECK-LABEL: define{{.*}} void @take_s2(i16
#[no_mangle]
pub extern "C" fn take_s2(value: S2) {
    let _ = value;
}

// CHECK-LABEL: define{{.*}} void @take_s3(i64
#[no_mangle]
pub extern "C" fn take_s3(value: S3) {
    let _ = value;
}

// CHECK-LABEL: define{{.*}} void @take_s4(i32
#[no_mangle]
pub extern "C" fn take_s4(value: S4) {
    let _ = value;
}

// CHECK-LABEL: define{{.*}} void @take_f4(i32
#[no_mangle]
pub extern "C" fn take_f4(value: F4) {
    let _ = value;
}

// CHECK-LABEL: define{{.*}} void @take_s5(i64
#[no_mangle]
pub extern "C" fn take_s5(value: S5) {
    let _ = value;
}

// CHECK-LABEL: define{{.*}} void @take_s8(i64
#[no_mangle]
pub extern "C" fn take_s8(value: S8) {
    let _ = value;
}

// CHECK-LABEL: define{{.*}} void @take_d8(i64
#[no_mangle]
pub extern "C" fn take_d8(value: D8) {
    let _ = value;
}

// CHECK-LABEL: define{{.*}} void @take_s12(
// CHECK-SAME: ptr{{.*}}byval([12 x i8])
#[no_mangle]
pub extern "C" fn take_s12(value: S12) {
    let _ = value;
}

// CHECK-LABEL: define{{.*}} <4 x float> @identity_f32x4(<4 x float>
#[no_mangle]
pub extern "C" fn identity_f32x4(value: F32x4) -> F32x4 {
    value
}

// CHECK-LABEL: define{{.*}} <4 x i32> @identity_i32x4(<4 x i32>
#[no_mangle]
pub extern "C" fn identity_i32x4(value: I32x4) -> I32x4 {
    value
}

// CHECK-LABEL: define{{.*}} <16 x i8> @identity_u8x16(<16 x i8>
#[no_mangle]
pub extern "C" fn identity_u8x16(value: U8x16) -> U8x16 {
    value
}

// CHECK-LABEL: define{{.*}} <4 x float> @identity_transparent_f32x4(<4 x float>
#[no_mangle]
pub extern "C" fn identity_transparent_f32x4(value: TransparentF32x4) -> TransparentF32x4 {
    value
}

// CHECK-LABEL: define{{.*}} void @identity_wrap_f32x4(
// CHECK-SAME: ptr{{.*}}sret([16 x i8]){{.*}}, ptr{{.*}}byval([16 x i8])
#[no_mangle]
pub extern "C" fn identity_wrap_f32x4(value: WrapF32x4) -> WrapF32x4 {
    value
}

// CHECK-LABEL: define{{.*}} i64 @identity_f32x2(i64
#[no_mangle]
pub extern "C" fn identity_f32x2(value: F32x2) -> F32x2 {
    value
}

// CHECK-LABEL: define{{.*}} void @identity_f64x2(
// CHECK-SAME: ptr{{.*}}sret([16 x i8]){{.*}}, ptr{{.*}}byval([16 x i8])
#[no_mangle]
pub extern "C" fn identity_f64x2(value: F64x2) -> F64x2 {
    value
}

// CHECK-LABEL: define{{.*}} void @identity_i64x2(
// CHECK-SAME: ptr{{.*}}sret([16 x i8]){{.*}}, ptr{{.*}}byval([16 x i8])
#[no_mangle]
pub extern "C" fn identity_i64x2(value: I64x2) -> I64x2 {
    value
}

// CHECK-LABEL: define{{.*}} void @identity_f32x8(
// CHECK-SAME: ptr{{.*}}sret([32 x i8]){{.*}}, ptr{{.*}}byval([32 x i8])
#[no_mangle]
pub extern "C" fn identity_f32x8(value: F32x8) -> F32x8 {
    value
}

// CHECK: define noundef i8 @narrow_i8(i8 noundef{{( returned)?}} %value)
#[no_mangle]
pub extern "C" fn narrow_i8(value: i8) -> i8 {
    value
}

// CHECK: define noundef i8 @narrow_u8(i8 noundef{{( returned)?}} %value)
#[no_mangle]
pub extern "C" fn narrow_u8(value: u8) -> u8 {
    value
}

// CHECK: define noundef i16 @narrow_i16(i16 noundef{{( returned)?}} %value)
#[no_mangle]
pub extern "C" fn narrow_i16(value: i16) -> i16 {
    value
}

// CHECK: define noundef i16 @narrow_u16(i16 noundef{{( returned)?}} %value)
#[no_mangle]
pub extern "C" fn narrow_u16(value: u16) -> u16 {
    value
}

// CHECK: define noundef zeroext i1 @narrow_bool(i1 noundef{{( returned)?}} zeroext %value)
#[no_mangle]
pub extern "C" fn narrow_bool(value: bool) -> bool {
    value
}
