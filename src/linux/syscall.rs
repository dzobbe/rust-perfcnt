
macro_rules! syscall {
    ($arg0:expr)
        => ( syscall::syscall0($arg0 as u64) );

    ($arg0:expr, $arg1:expr)
        => ( syscall::syscall1($arg0 as u64, $arg1 as u64) );

    ($arg0:expr, $arg1:expr, $arg2:expr)
        => ( syscall::syscall2($arg0 as u64, $arg1 as u64, $arg2 as u64) );

    ($arg0:expr, $arg1:expr, $arg2:expr, $arg3:expr)
        => ( syscall::syscall3($arg0 as u64, $arg1 as u64, $arg2 as u64, $arg3 as u64) );

    ($arg0:expr, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr)
        => ( syscall::syscall4($arg0 as u64, $arg1 as u64, $arg2 as u64, $arg3 as u64, $arg4 as u64) );

    ($arg0:expr, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr, $arg5:expr)
        => ( syscall::syscall5($arg0 as u64, $arg1 as u64, $arg2 as u64, $arg3 as u64, $arg4 as u64, $arg5 as u64) );

    ($arg0:expr, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr, $arg5:expr, $arg6:expr)
        => ( syscall::syscall6($arg0 as u64, $arg1 as u64, $arg2 as u64, $arg3 as u64, $arg4 as u64, $arg5 as u64, $arg6 as u64) );

    ($arg0:expr, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr, $arg5:expr, $arg6:expr, $arg7:expr)
        => ( syscall::syscall7($arg0 as u64, $arg1 as u64, $arg2 as u64, $arg3 as u64, $arg4 as u64, $arg5 as u64, $arg6 as u64, $arg7 as u64) );
}

#[inline(always)]
#[allow(unused_mut)]
pub unsafe fn syscall0(arg0: u64) -> u64 {
    let mut ret: u64;
    asm!("syscall" : "={rax}" (ret) : "{rax}" (arg0) : "rcx", "r11", "memory" : "volatile");
    ret
}

#[inline(always)]
#[allow(unused_mut)]
pub unsafe fn syscall1(arg0: u64, arg1: u64) -> u64 {
    let mut ret: u64;
    asm!("syscall" : "={rax}" (ret) : "{rax}" (arg0), "{rdi}" (arg1)
                   : "rcx", "r11", "memory" : "volatile");
    ret
}

#[inline(always)]
#[allow(unused_mut)]
pub unsafe fn syscall2(arg0: u64, arg1: u64, arg2: u64) -> u64 {
    let mut ret: u64;
    asm!("syscall" : "={rax}" (ret) : "{rax}" (arg0), "{rdi}" (arg1), "{rsi}" (arg2)
                   : "rcx", "r11", "memory" : "volatile");
    ret
}

#[inline(always)]
#[allow(unused_mut)]
pub unsafe fn syscall3(arg0: u64, arg1: u64, arg2: u64, arg3: u64) -> u64 {
    let mut ret: u64;
    asm!("syscall" : "={rax}" (ret) : "{rax}" (arg0), "{rdi}" (arg1), "{rsi}" (arg2), "{rdx}" (arg3)
                   : "rcx", "r11", "memory" : "volatile");
    ret
}

#[inline(always)]
#[allow(unused_mut)]
pub unsafe fn syscall4(arg0: u64, arg1: u64, arg2: u64, arg3: u64, arg4: u64) -> u64 {
    let mut ret: u64;
    asm!("syscall" : "={rax}" (ret)
                   : "{rax}"  (arg0), "{rdi}"  (arg1), "{rsi}"  (arg2), "{rdx}"  (arg3), "{r10}"  (arg4)
                   : "rcx", "r11", "memory" : "volatile");
    ret
}

#[inline(always)]
#[allow(unused_mut)]
pub unsafe fn syscall5(arg0: u64, arg1: u64, arg2: u64, arg3: u64, arg4: u64, arg5: u64) -> u64 {
    let mut ret: u64;
    asm!("syscall" : "={rax}" (ret)
                   : "{rax}" (arg0), "{rdi}" (arg1), "{rsi}" (arg2), "{rdx}" (arg3), "{r10}" (arg4), "{r8}" (arg5)
                   : "rcx", "r11", "memory"
                   : "volatile");
    ret
}

#[inline(always)]
#[allow(unused_mut)]
pub unsafe fn syscall6(arg0: u64,
                       arg1: u64,
                       arg2: u64,
                       arg3: u64,
                       arg4: u64,
                       arg5: u64,
                       arg6: u64)
                       -> u64 {
    let mut ret: u64;
    asm!("syscall" : "={rax}" (ret)
                   : "{rax}" (arg0), "{rdi}" (arg1), "{rsi}" (arg2), "{rdx}" (arg3),
                     "{r10}" (arg4), "{r8}" (arg5), "{r9}" (arg6)
                   : "rcx", "r11", "memory"
                   : "volatile");
    ret
}
