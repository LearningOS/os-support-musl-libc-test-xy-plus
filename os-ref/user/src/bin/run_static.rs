#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

// not in SUCC_TESTS & FAIL_TESTS
// count_lines, infloop, user_shell, usertests

// item of TESTS : app_name(argv_0), argv_1, argv_2, argv_3, exit_code
static SATIC_TESTS: &[&str] = &[
    ("argv.exe\0"),
    ("basename.exe\0"),
    ("clocale_mbfuncs.exe\0"),
    ("clock_gettime.exe\0"),
    ("crypt.exe\0"),
    ("dirname.exe\0"),
    ("env.exe\0"),
    ("fdopen.exe\0"),
    ("fnmatch.exe\0"),
    ("fscanf.exe\0"),
    ("fwscanf.exe\0"),
    ("iconv_open.exe\0"),
    ("inet_pton.exe\0"),
    ("mbc.exe\0"),
    ("memstream.exe\0"),
    ("pthread_cancel_points.exe\0"),
    ("pthread_cancel.exe\0"),
    ("pthread_cond.exe\0"),
    ("pthread_tsd.exe\0"),
    ("qsort.exe\0"),
    ("random.exe\0"),
    ("search_hsearch.exe\0"),
    ("search_insque.exe\0"),
    ("search_lsearch.exe\0"),
    ("search_tsearch.exe\0"),
    ("setjmp.exe\0"),
    ("snprintf.exe\0"),
    ("socket.exe\0"),
    ("sscanf.exe\0"),
    ("sscanf_long.exe\0"),
    ("stat.exe\0"),
    ("strftime.exe\0"),
    ("string.exe\0"),
    ("string_memcpy.exe\0"),
    ("string_memmem.exe\0"),
    ("string_memset.exe\0"),
    ("string_strchr.exe\0"),
    ("string_strcspn.exe\0"),
    ("string_strstr.exe\0"),
    ("strptime.exe\0"),
    ("strtod.exe\0"),
    ("strtod_simple.exe\0"),
    ("strtof.exe\0"),
    ("strtol.exe\0"),
    ("strtold.exe\0"),
    ("swprintf.exe\0"),
    ("tgmath.exe\0"),
    ("time.exe\0"),
    ("tls_align.exe\0"),
    ("udiv.exe\0"),
    ("ungetc.exe\0"),
    ("utime.exe\0"),
    ("wcsstr.exe\0"),
    ("wcstol.exe\0"),
    ("pleval.exe\0"),
    ("daemon_failure.exe\0"),
    ("dn_expand_empty.exe\0"),
    ("dn_expand_ptr_0.exe\0"),
    ("fflush_exit.exe\0"),
    ("fgets_eof.exe\0"),
    ("fgetwc_buffering.exe\0"),
    ("fpclassify_invalid_ld80.exe\0"),
    ("ftello_unflushed_append.exe\0"),
    ("getpwnam_r_crash.exe\0"),
    ("getpwnam_r_errno.exe\0"),
    ("iconv_roundtrips.exe\0"),
    ("inet_ntop_v4mapped.exe\0"),
    ("inet_pton_empty_last_field.exe\0"),
    ("iswspace_null.exe\0"),
    ("lrand48_signextend.exe\0"),
    ("lseek_large.exe\0"),
    ("malloc_0.exe\0"),
    ("mbsrtowcs_overflow.exe\0"),
    ("memmem_oob_read.exe\0"),
    ("memmem_oob.exe\0"),
    ("mkdtemp_failure.exe\0"),
    ("mkstemp_failure.exe\0"),
    ("printf_1e9_oob.exe\0"),
    ("printf_fmt_g_round.exe\0"),
    ("printf_fmt_g_zeros.exe\0"),
    ("printf_fmt_n.exe\0"),
    ("pthread_robust_detach.exe\0"),
    ("pthread_cancel_sem_wait.exe\0"),
    ("pthread_cond_smasher.exe\0"),
    ("pthread_condattr_setclock.exe\0"),
    ("pthread_exit_cancel.exe\0"),
    ("pthread_once_deadlock.exe\0"),
    ("pthread_rwlock_ebusy.exe\0"),
    ("putenv_doublefree.exe\0"),
    ("regex_backref_0.exe\0"),
    ("regex_bracket_icase.exe\0"),
    ("regex_ere_backref.exe\0"),
    ("regex_escaped_high_byte.exe\0"),
    ("regex_negated_range.exe\0"),
    ("regexec_nosub.exe\0"),
    ("rewind_clear_error.exe\0"),
    ("rlimit_open_files.exe\0"),
    ("scanf_bytes_consumed.exe\0"),
    ("scanf_match_literal_eof.exe\0"),
    ("scanf_nullbyte_char.exe\0"),
    ("setvbuf_unget.exe\0"),
    ("sigprocmask_internal.exe\0"),
    ("sscanf_eof.exe\0"),
    ("statvfs.exe\0"),
    ("strverscmp.exe\0"),
    ("syscall_sign_extend.exe\0"),
    ("uselocale_0.exe\0"),
    ("wcsncpy_read_overflow.exe\0"),
    ("wcsstr_false_negative.exe\0"),
];

use user_lib::{exec, fork, waitpid};

fn run_tests(tests: &[&str]) {
    let mut arr: [*const u8; 5] = [
        core::ptr::null::<u8>(),
        core::ptr::null::<u8>(),
        core::ptr::null::<u8>(),
        core::ptr::null::<u8>(),
        core::ptr::null::<u8>(),
    ];

    for test in tests {
        println!("Usertests: Running {}", test);
        arr[0] = "runtest.exe\0".as_ptr();
        arr[1] = "-w\0".as_ptr();
        arr[2] = "entry-static.exe\0".as_ptr();
        arr[3] = test.as_ptr();

        let pid = fork();
        if pid == 0 {
            exec("runtest.exe\0", &arr[..]);
            panic!("unreachable!");
        } else {
            let mut exit_code: i32 = Default::default();
            let wait_pid = waitpid(pid as usize, &mut exit_code);
            assert_eq!(pid, wait_pid);
        }
    }
}

#[no_mangle]
pub fn main() -> i32 {
    run_tests(SATIC_TESTS);
    return 0;
}
