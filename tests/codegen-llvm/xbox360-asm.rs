//@ add-minicore
//@ compile-flags: --target={{src-base}}/xbox360-abi.json -Zunstable-options -Zmerge-functions=disabled
//@ revisions: vmx classic
//@[vmx] compile-flags: -Ctarget-feature=+vmx128
//@[classic] compile-flags: -Ctarget-feature=-vmx128,+altivec
//@ needs-llvm-components: powerpc

#![crate_type = "lib"]
#![feature(no_core, asm_experimental_arch)]
#![no_core]

extern crate minicore;
use minicore::*;

// CHECK-LABEL: @call_clobbers
// vmx: "{{[^"]*}}={v20},{{[^"]*}}={v31},={v32},{{[^"]*}}={v63},~{cr0}
// classic: "{{[^"]*}}={v20},{{[^"]*}}={v31},~{v32},{{[^"]*}}~{v63},~{cr0}
// CHECK-NOT: ={v64}
// CHECK: ret void
#[no_mangle]
pub unsafe fn call_clobbers() {
    asm!("", clobber_abi("C"));
}

// CHECK-LABEL: @high_vector_clobber
// vmx: "=&{v127}"
// classic: "~{v127}"
// CHECK: ret void
#[no_mangle]
pub unsafe fn high_vector_clobber() {
    asm!("", out("v127") _, options(nostack, nomem, preserves_flags));
}
