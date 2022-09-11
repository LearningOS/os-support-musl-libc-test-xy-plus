use numeric_enum_macro::numeric_enum;
numeric_enum! {
    #[repr(usize)]
    #[derive(Debug)]
    enum SYSCALL {
        DUP = 24,
        OPEN = 56,
        CLOSE = 57,
        PIPE = 59,
        READ = 63,
        WRITE = 64,
        EXIT = 93,
        EXIT_GROUP = 94,
        SLEEP = 101,
        CLOCK_GETTIME = 113,
        YIELD = 124,
        KILL = 129,
        RT_SIGACTION = 134,
        RT_SIGPROCMASK = 135,
        RT_SIGTIMEDWAIT = 137,
        GET_TIME = 169,
        GETPID = 172,
        GETTID = 178,
        BRK = 214,
        FORK = 220,
        EXEC = 221,
        WAITPID = 260,
        PRLIMIT64 = 261,
        THREAD_CREATE = 1000,
        WAITTID = 1002,
        MUTEX_CREATE = 1010,
        MUTEX_LOCK = 1011,
        MUTEX_UNLOCK = 1012,
        SEMAPHORE_CREATE = 1020,
        SEMAPHORE_UP = 1021,
        SEMAPHORE_DOWN = 1022,
        CONDVAR_CREATE = 1030,
        CONDVAR_SIGNAL = 1031,
        CONDVAR_WAIT = 1032,
        CREATE_DESKTOP = 2000,
    }
}

mod fs;
#[cfg(feature = "board_qemu")]
mod gui;
mod process;
mod signal;
mod sync;
mod thread;
#[cfg(feature = "board_qemu")]
pub use self::gui::create_desktop;
use fs::*;

#[cfg(feature = "board_qemu")]
pub use gui::PAD;
use process::*;
use signal::*;
use sync::*;
use thread::*;

#[cfg(feature = "board_qemu")]
pub fn syscall(syscall_id: usize, args: [usize; 4]) -> isize {
    // if syscall_id != SYSCALL::YIELD && syscall_id != SYSCALL::WAITPID {
    //     println!(
    //         "{} in {} : {}, {}, {}, {}",
    //         syscall_id,
    //         sys_getpid(),
    //         args[0],
    //         args[1],
    //         args[2],
    //         args[3]
    //     );
    // }
    if let Ok(syscall_id) = SYSCALL::try_from(syscall_id) {
        match syscall_id {
            SYSCALL::DUP => sys_dup(args[0]),
            SYSCALL::OPEN => sys_open(args[0] as *const u8, args[1] as u32),
            SYSCALL::CLOSE => sys_close(args[0]),
            SYSCALL::PIPE => sys_pipe(args[0] as *mut usize),
            SYSCALL::READ => sys_read(args[0], args[1] as *const u8, args[2]),
            SYSCALL::WRITE => sys_write(args[0], args[1] as *const u8, args[2]),
            SYSCALL::EXIT => sys_exit(args[0] as i32),
            SYSCALL::EXIT_GROUP => sys_exit(args[0] as i32),
            SYSCALL::SLEEP => sys_sleep(args[0]),
            SYSCALL::YIELD => sys_yield(),
            SYSCALL::KILL => sys_kill(args[0], args[1] as u64),
            SYSCALL::RT_SIGACTION => sys_rt_sigaction(args[0], args[1], args[2], args[3]),
            SYSCALL::RT_SIGPROCMASK => sys_rt_sigprocmask(args[0], args[1], args[2], args[3]),
            SYSCALL::RT_SIGTIMEDWAIT => sys_rt_sigtimedwait(args[0], args[1], args[2]),
            SYSCALL::GET_TIME => sys_get_time(),
            SYSCALL::CLOCK_GETTIME => sys_clock_get_time(args[0], args[1]),
            SYSCALL::GETPID => sys_getpid(),
            SYSCALL::BRK => {
                // println!("[kernel] warn: skip unimplemented sys brk");
                -1
            }
            SYSCALL::FORK => sys_fork(),
            SYSCALL::EXEC => sys_exec(args[0] as *const u8, args[1] as *const usize),
            SYSCALL::WAITPID => sys_waitpid(args[0] as isize, args[1] as *mut i32),
            SYSCALL::THREAD_CREATE => sys_thread_create(args[0], args[1]),
            SYSCALL::GETTID => sys_gettid(),
            SYSCALL::WAITTID => sys_waittid(args[0]) as isize,
            SYSCALL::PRLIMIT64 => {
                // println!("[kernel] warn: skip unimplemented sys prlimit64");
                0
            }
            SYSCALL::MUTEX_CREATE => sys_mutex_create(args[0] == 1),
            SYSCALL::MUTEX_LOCK => sys_mutex_lock(args[0]),
            SYSCALL::MUTEX_UNLOCK => sys_mutex_unlock(args[0]),
            SYSCALL::SEMAPHORE_CREATE => sys_semaphore_create(args[0]),
            SYSCALL::SEMAPHORE_UP => sys_semaphore_up(args[0]),
            SYSCALL::SEMAPHORE_DOWN => sys_semaphore_down(args[0]),
            SYSCALL::CONDVAR_CREATE => sys_condvar_create(args[0]),
            SYSCALL::CONDVAR_SIGNAL => sys_condvar_signal(args[0]),
            SYSCALL::CONDVAR_WAIT => sys_condvar_wait(args[0], args[1]),
            SYSCALL::CREATE_DESKTOP => create_desktop(),
        }
    } else {
        panic!("Unsupported syscall_id: {}", syscall_id)
    }
}

#[cfg(feature = "board_k210")]
pub fn syscall(syscall_id: usize, args: [usize; 3]) -> isize {
    match syscall_id {
        SYSCALL::DUP => sys_dup(args[0]),
        SYSCALL::OPEN => sys_open(args[0] as *const u8, args[1] as u32),
        SYSCALL::CLOSE => sys_close(args[0]),
        SYSCALL::PIPE => sys_pipe(args[0] as *mut usize),
        SYSCALL::READ => sys_read(args[0], args[1] as *const u8, args[2]),
        SYSCALL::WRITE => sys_write(args[0], args[1] as *const u8, args[2]),
        SYSCALL::EXIT => sys_exit(args[0] as i32),
        SYSCALL::SLEEP => sys_sleep(args[0]),
        SYSCALL::YIELD => sys_yield(),
        SYSCALL::KILL => sys_kill(args[0], args[1] as u32),
        SYSCALL::GET_TIME => sys_get_time(),
        SYSCALL::GETPID => sys_getpid(),
        SYSCALL::FORK => sys_fork(),
        SYSCALL::EXEC => sys_exec(args[0] as *const u8, args[1] as *const usize),
        SYSCALL::WAITPID => sys_waitpid(args[0] as isize, args[1] as *mut i32),
        SYSCALL::THREAD_CREATE => sys_thread_create(args[0], args[1]),
        SYSCALL::GETTID => sys_gettid(),
        SYSCALL::WAITTID => sys_waittid(args[0]) as isize,
        SYSCALL::MUTEX_CREATE => sys_mutex_create(args[0] == 1),
        SYSCALL::MUTEX_LOCK => sys_mutex_lock(args[0]),
        SYSCALL::MUTEX_UNLOCK => sys_mutex_unlock(args[0]),
        SYSCALL::SEMAPHORE_CREATE => sys_semaphore_create(args[0]),
        SYSCALL::SEMAPHORE_UP => sys_semaphore_up(args[0]),
        SYSCALL::SEMAPHORE_DOWN => sys_semaphore_down(args[0]),
        SYSCALL::CONDVAR_CREATE => sys_condvar_create(args[0]),
        SYSCALL::CONDVAR_SIGNAL => sys_condvar_signal(args[0]),
        SYSCALL::CONDVAR_WAIT => sys_condvar_wait(args[0], args[1]),
        _ => panic!("Unsupported syscall_id: {}", syscall_id),
    }
}
