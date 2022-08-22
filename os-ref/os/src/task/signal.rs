use bitflags::*;

bitflags! {
    #[derive(Default)]
    pub struct SignalFlags: u64 {
        const SIGINT    = 1 << (2-1);
        const SIGQUIT   = 1 << (3-1);
        const SIGILL    = 1 << (4-1);
        const SIGABRT   = 1 << (6-1);
        const SIGBUS    = 1 << (7-1);
        const SIGFPE    = 1 << (8-1);
        const SIGKILL   = 1 << (9-1);
        const SIGSEGV   = 1 << (11-1);
        const SIGPIPE   = 1 << (13-1);
        const SIGTERM   = 1 << (15-1);
        const SIGSTKFLT = 1 << (16-1);
        const SIGCHLD   = 1 << (17-1);
        const SIGSTOP   = 1 << (19-1);
    }
}

impl SignalFlags {
    pub fn check_error(&self, mask: SignalFlags) -> Option<(i32, &'static str)> {
        // let sig = *self & !mask;
        let sig = *self;
        if sig.contains(Self::SIGINT) {
            Some((-2, "Killed, SIGINT=2"))
        } else if sig.contains(Self::SIGILL) {
            Some((-4, "Illegal Instruction, SIGILL=4"))
        } else if sig.contains(Self::SIGABRT) {
            Some((-6, "Aborted, SIGABRT=6"))
        } else if sig.contains(Self::SIGFPE) {
            Some((-8, "Erroneous Arithmetic Operation, SIGFPE=8"))
        } else if sig.contains(Self::SIGSEGV) {
            Some((-11, "Segmentation Fault, SIGSEGV=11"))
        } else {
            None
        }
    }
}

/// Linux struct sigaction
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct SignalAction {
    pub handler: usize, // this field may be an union
    pub flags: SignalActionFlags,
    pub restorer: usize,
    pub mask: SignalFlags,
}

bitflags! {
    #[derive(Default)]
    pub struct SignalActionFlags : usize {
        const NOCLDSTOP = 1;
        const NOCLDWAIT = 2;
        const SIGINFO = 4;
        const ONSTACK = 0x08000000;
        const RESTART = 0x10000000;
        const NODEFER = 0x40000000;
        const RESETHAND = 0x80000000;
        const RESTORER = 0x04000000;
    }
}
