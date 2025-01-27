//@ run-pass
//@ compile-flags: -Cstrip=none -Cdebuginfo=line-tables-only
//@ needs-unwind

#![allow(improper_ctypes)]
#![allow(improper_ctypes_definitions)]
#![feature(backtrace_frames)]

use std::{backtrace::Backtrace, sync::atomic::{AtomicBool, Ordering}};

#[link(name = "rust_test_helpers", kind = "static")]
extern "C" {
    fn cpp_trampoline(func: extern "C" fn()) -> ();
}

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");
    static RAN_ASSERTS: AtomicBool = AtomicBool::new(false);

    extern "C" fn assert_cpp_frames() {
        let trace = Backtrace::capture();

        let actual = format!("{:?}", trace.frames());
        let expected = [
            "templated_trampoline",
            "cpp_trampoline",
        ];
        for e in expected {
            assert!(actual.contains(e));
        }

        RAN_ASSERTS.store(true, Ordering::SeqCst);
    }

    assert!(!RAN_ASSERTS.load(Ordering::SeqCst));
    unsafe {
        cpp_trampoline(assert_cpp_frames);
    }
    assert!(RAN_ASSERTS.load(Ordering::SeqCst));
}
