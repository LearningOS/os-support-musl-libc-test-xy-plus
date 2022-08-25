use crate::mm::{translated_ref, translated_refmut};
use crate::task::{block_current_and_run_next, current_process, current_task, current_user_token};
use crate::task::{SignalAction, SignalFlags};
use crate::timer::{add_timer, get_time_ms, TimeVal};
use numeric_enum_macro::numeric_enum;

pub fn sys_rt_sigaction(signum: usize, act: usize, oldact: usize, sigsetsize: usize) -> isize {
    // println!("[kernel] warn: unimplement syscall rt_sigaction, will have no use");

    if let Some(signal) = SignalFlags::from_bits(1 << signum as u64) {
        if sigsetsize != core::mem::size_of::<SignalFlags>()
            || signal == SignalFlags::SIGKILL
            || signal == SignalFlags::SIGSTOP
        {
            return -1;
        }
        let process = current_process();
        let token = current_user_token();
        let mut inner = process.inner_exclusive_access();
        *translated_refmut(token, oldact as *mut SignalAction) = inner.signal_action(signum);
        inner.set_signal_action(signum, *translated_refmut(token, act as *mut SignalAction));
        return 0;
    }
    return -1;
}

pub fn sys_rt_sigprocmask(how: usize, set: usize, oldset: usize, sigsetsize: usize) -> isize {
    // println!("[kernel] warn: unimplement syscall rt_sigprocmask, will have no use");
    numeric_enum! {
        #[repr(usize)]
        #[derive(Debug)]
        enum How {
            Block = 0,
            Unblock = 1,
            SetMask = 2,
        }
    }
    if let Ok(how) = How::try_from(how) {
        if sigsetsize != core::mem::size_of::<SignalFlags>() {
            return -1;
        }
        let process = current_process();
        let token = current_user_token();
        let mut inner = process.inner_exclusive_access();

        if oldset != 0 {
            *translated_refmut(token, oldset as *mut SignalFlags) = inner.signal_mask;
        }
        let set = *translated_ref(token, set as *mut SignalFlags);
        match how {
            How::Block => inner.signal_mask |= set,
            How::Unblock => inner.signal_mask -= set,
            How::SetMask => inner.signal_mask = set,
        }
        return 0;
    }
    return -1;
}

pub fn sys_rt_sigtimedwait(set: usize, info: usize, timeout: usize) -> isize {
    // println!("sys rt_sigtimedwait isn't fully implemented, only sigchld won't be ingnored");
    let token = current_user_token();
    let set = *translated_ref(token, set as *mut SignalFlags);
    if set.contains(SignalFlags::SIGCHLD) {
        let timeout = translated_ref(token, timeout as *mut TimeVal);
        let expire_ms = get_time_ms() + timeout.sec * 1000;
        let task = current_task().unwrap();
        add_timer(expire_ms, task);
        block_current_and_run_next();
        let process = current_process();
        let mut inner = process.inner_exclusive_access();
        let pair = inner
            .children
            .iter()
            .enumerate()
            .find(|(_, p)| p.inner_exclusive_access().is_zombie);
        if let Some((idx, _)) = pair {
            return 0;
        }
        return -1;
    } else {
        println!("sigchld not in set, return 0 immediately");
        return 0;
    }
}
