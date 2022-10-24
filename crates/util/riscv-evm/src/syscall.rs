//! riscv32 system calls.
// calling convention: https://man7.org/linux/man-pages/man2/syscall.2.html
// input: a7 system number, a0:arg0, ... a5:arg5
// ouput: ret0:a0, ret1:a1

#![allow(dead_code)]

#[cfg(target_arch = "x86_64")]
pub use x64::*;

#[cfg(target_arch = "x86_64")]
mod x64 {
    use core::arch::asm;
    //x86_64 calling convention: sysnum: rax, input: rdi rsi rdx r10 r8 r9, ouput: rax rdx

    #[inline]
    #[must_use]
    pub unsafe fn syscall0_readonly(nr: usize) -> usize {
        let r0: usize;
        asm!(
        "syscall",
        inlateout("rax") nr => r0,
        options(nostack, preserves_flags, readonly)
        );
        r0
    }

    #[inline]
    pub unsafe fn syscall1(nr: usize, a0: usize) -> usize {
        let r0;
        asm!(
        "syscall",
        inlateout("rax") nr => r0,
        in("rdi") a0,
        options(nostack, preserves_flags)
        );
        r0
    }

    #[inline]
    #[must_use]
    pub unsafe fn syscall1_readonly(nr: usize, a0: usize) -> usize {
        let r0;
        asm!(
        "syscall",
        inlateout("rax") nr => r0,
        in("rdi") a0,
        options(nostack, preserves_flags, readonly)
        );
        r0
    }

    #[inline]
    #[must_use]
    pub unsafe fn syscall1_noreturn(nr: usize, a0: usize) -> ! {
        asm!( "syscall", in("rax") nr, in("rdi") a0, options(noreturn))
    }

    #[inline]
    pub unsafe fn syscall2(nr: usize, a0: usize, a1: usize) -> usize {
        let r0;
        asm!(
        "syscall",
        inlateout("rax") nr => r0,
        in("rdi") a0,
        in("rsi") a1,
        options(nostack, preserves_flags)
        );
        r0
    }

    #[inline]
    #[must_use]
    pub unsafe fn syscall2_noreturn(nr: usize, a0: usize, a1: usize) -> ! {
        asm!( "syscall", in("rax") nr, in("rdi") a0, in("rsi") a1, options(noreturn))
    }

    #[inline]
    pub unsafe fn syscall2_readonly(nr: usize, a0: usize, a1: usize) -> usize {
        let r0;
        asm!(
        "syscall",
        inlateout("rax") nr => r0,
        in("rdi") a0,
        in("rsi") a1,
        options(nostack, preserves_flags, readonly)
        );
        r0
    }

    #[inline]
    #[must_use]
    pub unsafe fn syscall3(nr: usize, a0: usize, a1: usize, a2: usize) -> usize {
        let r0;
        asm!(
        "syscall",
        inlateout("rax") nr => r0,
        in("rdi") a0,
        in("rsi") a1,
        in("rdx") a2,
        options(nostack, preserves_flags)
        );
        r0
    }

    #[inline]
    #[must_use]
    pub unsafe fn syscall3_readonly(nr: usize, a0: usize, a1: usize, a2: usize) -> usize {
        let r0;
        asm!(
        "syscall",
        inlateout("rax") nr => r0,
        in("rdi") a0,
        in("rsi") a1,
        in("rdx") a2,
        options(nostack, preserves_flags, readonly)
        );
        r0
    }

    #[inline]
    #[must_use]
    pub unsafe fn syscall4(nr: usize, a0: usize, a1: usize, a2: usize, a3: usize) -> usize {
        let r0;
        asm!(
        "syscall",
        inlateout("rax") nr => r0,
        in("rdi") a0,
        in("rsi") a1,
        in("rdx") a2,
        in("r10") a3,
        options(nostack, preserves_flags)
        );
        r0
    }

    #[inline]
    #[must_use]
    pub unsafe fn syscall4_readonly(
        nr: usize, a0: usize, a1: usize, a2: usize, a3: usize,
    ) -> usize {
        let r0;
        asm!(
        "syscall",
        inlateout("rax") nr => r0,
        in("rdi") a0,
        in("rsi") a1,
        in("rdx") a2,
        in("r10") a3,
        options(nostack, preserves_flags, readonly)
        );
        r0
    }

    #[inline]
    pub unsafe fn syscall5(
        nr: usize, a0: usize, a1: usize, a2: usize, a3: usize, a4: usize,
    ) -> usize {
        let r0;
        asm!(
        "syscall",
        inlateout("rax") nr => r0,
        in("rdi") a0,
        in("rsi") a1,
        in("rdx") a2,
        in("r10") a3,
        in("r8") a4,
        options(nostack, preserves_flags)
        );
        r0
    }

    #[inline]
    #[must_use]
    pub unsafe fn syscall5_readonly(
        nr: usize, a0: usize, a1: usize, a2: usize, a3: usize, a4: usize,
    ) -> usize {
        let r0;
        asm!(
        "syscall",
        inlateout("rax") nr => r0,
        in("rdi") a0,
        in("rsi") a1,
        in("rdx") a2,
        in("r10") a3,
        in("r8") a4,
        options(nostack, preserves_flags, readonly)
        );
        r0
    }

    #[inline]
    #[must_use]
    pub unsafe fn syscall6(
        nr: usize, a0: usize, a1: usize, a2: usize, a3: usize, a4: usize, a5: usize,
    ) -> usize {
        let r0;
        asm!(
        "syscall",
        inlateout("rax") nr => r0,
        in("rdi") a0,
        in("rsi") a1,
        in("rdx") a2,
        in("r10") a3,
        in("r8") a4,
        in("r9") a5,
        options(nostack, preserves_flags)
        );
        r0
    }

    #[inline]
    #[must_use]
    pub unsafe fn syscall6_readonly(
        nr: usize, a0: usize, a1: usize, a2: usize, a3: usize, a4: usize, a5: usize,
    ) -> usize {
        let r0;
        asm!(
        "syscall",
        inlateout("rax") nr => r0,
        in("rdi") a0,
        in("rsi") a1,
        in("rdx") a2,
        in("r10") a3,
        in("r8") a4,
        in("r9") a5,
        options(nostack, preserves_flags, readonly)
        );
        r0
    }
}

#[cfg(target_arch = "riscv32")]
pub use riscv::*;

#[cfg(target_arch = "riscv32")]
mod riscv {
    use core::arch::asm;

    #[inline]
    #[must_use]
    pub unsafe fn syscall0_readonly(nr: usize) -> usize {
        let r0: usize;
        asm!( "ecall", in("a7") nr, lateout("a0") r0,
        options(nostack, preserves_flags, readonly)
        );
        r0
    }

    #[inline]
    pub unsafe fn syscall1(nr: usize, a0: usize) -> usize {
        let r0;
        asm!(
        "ecall",
        in("a7") nr,
        inlateout("a0") a0 => r0,
        options(nostack, preserves_flags)
        );
        r0
    }

    #[inline]
    #[must_use]
    pub unsafe fn syscall1_readonly(nr: usize, a0: usize) -> usize {
        let r0;
        asm!(
        "ecall",
        in("a7") nr,
        inlateout("a0") a0 => r0,
        options(nostack, preserves_flags, readonly)
        );
        r0
    }

    #[inline]
    #[must_use]
    pub unsafe fn syscall1_noreturn(nr: usize, a0: usize) -> ! {
        asm!( "ecall", in("a7") nr, in("a0") a0, options(noreturn))
    }

    #[inline]
    pub unsafe fn syscall2(nr: usize, a0: usize, a1: usize) -> usize {
        let r0;
        asm!(
        "ecall",
        in("a7") nr,
        inlateout("a0") a0 => r0,
        in("a1") a1,
        options(nostack, preserves_flags)
        );
        r0
    }

    #[inline]
    #[must_use]
    pub unsafe fn syscall2_noreturn(nr: usize, a0: usize, a1: usize) -> ! {
        asm!( "ecall", in("a7") nr, in("a0") a0, in("a1") a1, options(noreturn))
    }

    #[inline]
    pub unsafe fn syscall2_readonly(nr: usize, a0: usize, a1: usize) -> usize {
        let r0;
        asm!(
        "ecall",
        in("a7") nr,
        inlateout("a0") a0 => r0,
        in("a1") a1,
        options(nostack, preserves_flags, readonly)
        );
        r0
    }

    #[inline]
    #[must_use]
    pub unsafe fn syscall3(nr: usize, a0: usize, a1: usize, a2: usize) -> usize {
        let r0;
        asm!(
        "ecall",
        in("a7") nr,
        inlateout("a0") a0 => r0,
        in("a1") a1,
        in("a2") a2,
        options(nostack, preserves_flags)
        );
        r0
    }

    #[inline]
    #[must_use]
    pub unsafe fn syscall3_readonly(nr: usize, a0: usize, a1: usize, a2: usize) -> usize {
        let r0;
        asm!(
        "ecall",
        in("a7") nr,
        inlateout("a0") a0 => r0,
        in("a1") a1,
        in("a2") a2,
        options(nostack, preserves_flags, readonly)
        );
        r0
    }

    #[inline]
    #[must_use]
    pub unsafe fn syscall4(nr: usize, a0: usize, a1: usize, a2: usize, a3: usize) -> usize {
        let r0;
        asm!(
        "ecall",
        in("a7") nr,
        inlateout("a0") a0 => r0,
        in("a1") a1,
        in("a2") a2,
        in("a3") a3,
        options(nostack, preserves_flags)
        );
        r0
    }

    #[inline]
    #[must_use]
    pub unsafe fn syscall4_readonly(
        nr: usize, a0: usize, a1: usize, a2: usize, a3: usize,
    ) -> usize {
        let r0;
        asm!(
        "ecall",
        in("a7") nr,
        inlateout("a0") a0 => r0,
        in("a1") a1,
        in("a2") a2,
        in("a3") a3,
        options(nostack, preserves_flags, readonly)
        );
        r0
    }

    #[inline]
    pub unsafe fn syscall5(
        nr: usize, a0: usize, a1: usize, a2: usize, a3: usize, a4: usize,
    ) -> usize {
        let r0;
        asm!(
        "ecall",
        in("a7") nr,
        inlateout("a0") a0 => r0,
        in("a1") a1,
        in("a2") a2,
        in("a3") a3,
        in("a4") a4,
        options(nostack, preserves_flags)
        );
        r0
    }

    #[inline]
    #[must_use]
    pub unsafe fn syscall5_readonly(
        nr: usize, a0: usize, a1: usize, a2: usize, a3: usize, a4: usize,
    ) -> usize {
        let r0;
        asm!(
        "ecall",
        in("a7") nr,
        inlateout("a0") a0 => r0,
        in("a1") a1,
        in("a2") a2,
        in("a3") a3,
        in("a4") a4,
        options(nostack, preserves_flags, readonly)
        );
        r0
    }

    #[inline]
    #[must_use]
    pub unsafe fn syscall6(
        nr: usize, a0: usize, a1: usize, a2: usize, a3: usize, a4: usize, a5: usize,
    ) -> usize {
        let r0;
        asm!(
        "ecall",
        in("a7") nr,
        inlateout("a0") a0 => r0,
        in("a1") a1,
        in("a2") a2,
        in("a3") a3,
        in("a4") a4,
        in("a5") a5,
        options(nostack, preserves_flags)
        );
        r0
    }

    #[inline]
    #[must_use]
    pub unsafe fn syscall6_readonly(
        nr: usize, a0: usize, a1: usize, a2: usize, a3: usize, a4: usize, a5: usize,
    ) -> usize {
        let r0;
        asm!(
        "ecall",
        in("a7") nr,
        inlateout("a0") a0 => r0,
        in("a1") a1,
        in("a2") a2,
        in("a3") a3,
        in("a4") a4,
        in("a5") a5,
        options(nostack, preserves_flags, readonly)
        );
        r0
    }
}
