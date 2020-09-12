#![cfg(feature = "instructions")]

//! Special x86_64 instructions.

pub mod interrupts;
pub mod port;
pub mod random;
pub mod segmentation;
pub mod tables;
pub mod tlb;

/// Halts the CPU until the next interrupt arrives.
#[inline]
pub fn hlt() {
    #[cfg(feature = "inline_asm")]
    unsafe {
        llvm_asm!("hlt" :::: "volatile");
    }

    #[cfg(not(feature = "inline_asm"))]
    unsafe {
        crate::asm::x86_64_asm_hlt();
    }
}

/// Executes the `nop` instructions, which performs no operation (i.e. does nothing).
///
/// This operation is useful to work around the LLVM bug that endless loops are illegally
/// optimized away (see [the issue](https://github.com/rust-lang/rust/issues/28728)). By invoking this
/// instruction (which is marked as volatile), the compiler should no longer optimize the
/// endless loop away.
#[inline]
pub fn nop() {
    #[cfg(feature = "inline_asm")]
    unsafe {
        llvm_asm!("nop" :::: "volatile");
    }

    #[cfg(not(feature = "inline_asm"))]
    unsafe {
        crate::asm::x86_64_asm_nop();
    }
}

/// Emits a '[magic breakpoint](https://wiki.osdev.org/Bochs#Magic_Breakpoint)' instruction for the [Bochs](http://bochs.sourceforge.net/) CPU
/// emulator. Make sure to set `magic_break: enabled=1` in your `.bochsrc` file.
#[cfg(feature = "inline_asm")]
#[inline]
pub fn bochs_breakpoint() {
    unsafe {
        llvm_asm!("xchgw %bx, %bx" :::: "volatile");
    }
}

/// Gets the current instruction pointer. Note that this is only approximate as it requires a few
/// instructions to execute.
#[cfg(feature = "inline_asm")]
#[inline(always)]
pub fn read_rip() -> u64 {
    let rip: u64;
    unsafe {
        llvm_asm!(
            "lea (%rip), $0"
            : "=r"(rip) ::: "volatile"
        );
    }
    rip
}

/// Writes the FS segment base address
///
/// ## Safety
///
/// If `CR4.FSGSBASE` is not set, this instruction will throw an `#UD`.
///
/// The caller must ensure that this write operation has no unsafe side
/// effects, as the FS segment base address is often used for thread
/// local storage.
#[inline]
pub unsafe fn wrfsbase(val: u64) {
    #[cfg(feature = "inline_asm")]
    #[inline(always)]
    unsafe fn inner(val: u64) {
        llvm_asm!("wrfsbase $0" :: "r"(val) :: "volatile")
    }

    #[cfg(not(feature = "inline_asm"))]
    #[inline(always)]
    unsafe fn inner(val: u64) {
        crate::asm::x86_64_asm_wrfsbase(val)
    }

    inner(val)
}

/// Reads the FS segment base address
///
/// ## Safety
///
/// If `CR4.FSGSBASE` is not set, this instruction will throw an `#UD`.
#[inline]
pub unsafe fn rdfsbase() -> u64 {
    #[cfg(feature = "inline_asm")]
    #[inline(always)]
    unsafe fn inner() -> u64 {
        let val: u64;
        llvm_asm!("rdfsbase $0" : "=r" (val) ::: "volatile");
        val
    }

    #[cfg(not(feature = "inline_asm"))]
    #[inline(always)]
    unsafe fn inner() -> u64 {
        crate::asm::x86_64_asm_rdfsbase()
    }

    inner()
}

/// Writes the GS segment base address
///
/// ## Safety
///
/// If `CR4.FSGSBASE` is not set, this instruction will throw an `#UD`.
///
/// The caller must ensure that this write operation has no unsafe side
/// effects, as the GS segment base address might be in use.
#[inline]
pub unsafe fn wrgsbase(val: u64) {
    #[cfg(feature = "inline_asm")]
    #[inline(always)]
    unsafe fn inner(val: u64) {
        llvm_asm!("wrgsbase $0" :: "r"(val) :: "volatile")
    }

    #[cfg(not(feature = "inline_asm"))]
    #[inline(always)]
    unsafe fn inner(val: u64) {
        crate::asm::x86_64_asm_wrgsbase(val)
    }

    inner(val)
}

/// Reads the GS segment base address
///
/// ## Safety
///
/// If `CR4.FSGSBASE` is not set, this instruction will throw an `#UD`.
#[inline]
pub unsafe fn rdgsbase() -> u64 {
    #[cfg(feature = "inline_asm")]
    #[inline(always)]
    unsafe fn inner() -> u64 {
        let val: u64;
        llvm_asm!("rdgsbase $0" : "=r" (val) ::: "volatile");
        val
    }

    #[cfg(not(feature = "inline_asm"))]
    #[inline(always)]
    unsafe fn inner() -> u64 {
        crate::asm::x86_64_asm_rdgsbase()
    }

    inner()
}
