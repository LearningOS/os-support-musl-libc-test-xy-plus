#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

// not in SUCC_TESTS & FAIL_TESTS
// count_lines, infloop, user_shell, usertests

// item of TESTS : app_name(argv_0), argv_1, argv_2, argv_3, exit_code
static SATIC_TESTS: &[&str] = &[
    ("argv\0"),
    ("basename\0"),
    // ("clocale_mbfuncs\0"),
    ("clock_gettime\0"),
    ("crypt\0"),
    ("dirname\0"),
    // ("env\0"),
    // ("fdopen\0"),
    ("fnmatch\0"),
    ("fscanf\0"),
    ("fwscanf\0"),
    ("iconv_open\0"),
    ("inet_pton\0"),
    ("mbc\0"),
    ("memstream\0"),
    ("pthread_cancel_points\0"),
    ("pthread_cancel\0"),
    ("pthread_cond\0"),
    ("pthread_tsd\0"),
    ("qsort\0"),
    ("random\0"),
    ("search_hsearch\0"),
    ("search_insque\0"),
    ("search_lsearch\0"),
    ("search_tsearch\0"),
    ("setjmp\0"),
    ("snprintf\0"),
    ("socket\0"),
    ("sscanf\0"),
    ("sscanf_long\0"),
    ("stat\0"),
    ("strftime\0"),
    ("string\0"),
    ("string_memcpy\0"),
    ("string_memmem\0"),
    ("string_memset\0"),
    ("string_strchr\0"),
    ("string_strcspn\0"),
    ("string_strstr\0"),
    ("strptime\0"),
    ("strtod\0"),
    ("strtod_simple\0"),
    ("strtof\0"),
    ("strtol\0"),
    ("strtold\0"),
    ("swprintf\0"),
    ("tgmath\0"),
    ("time\0"),
    ("tls_align\0"),
    ("udiv\0"),
    ("ungetc\0"),
    ("utime\0"),
    ("wcsstr\0"),
    ("wcstol\0"),
    ("pleval\0"),
    ("daemon_failure\0"),
    ("dn_expand_empty\0"),
    ("dn_expand_ptr_0\0"),
    ("fflush_exit\0"),
    ("fgets_eof\0"),
    ("fgetwc_buffering\0"),
    ("fpclassify_invalid_ld80\0"),
    ("ftello_unflushed_append\0"),
    ("getpwnam_r_crash\0"),
    ("getpwnam_r_errno\0"),
    ("iconv_roundtrips\0"),
    ("inet_ntop_v4mapped\0"),
    ("inet_pton_empty_last_field\0"),
    ("iswspace_null\0"),
    ("lrand48_signextend\0"),
    ("lseek_large\0"),
    ("malloc_0\0"),
    ("mbsrtowcs_overflow\0"),
    ("memmem_oob_read\0"),
    ("memmem_oob\0"),
    ("mkdtemp_failure\0"),
    ("mkstemp_failure\0"),
    ("printf_1e9_oob\0"),
    ("printf_fmt_g_round\0"),
    ("printf_fmt_g_zeros\0"),
    ("printf_fmt_n\0"),
    ("pthread_robust_detach\0"),
    ("pthread_cancel_sem_wait\0"),
    ("pthread_cond_smasher\0"),
    ("pthread_condattr_setclock\0"),
    ("pthread_exit_cancel\0"),
    ("pthread_once_deadlock\0"),
    ("pthread_rwlock_ebusy\0"),
    ("putenv_doublefree\0"),
    ("regex_backref_0\0"),
    ("regex_bracket_icase\0"),
    ("regex_ere_backref\0"),
    ("regex_escaped_high_byte\0"),
    ("regex_negated_range\0"),
    ("regexec_nosub\0"),
    ("rewind_clear_error\0"),
    ("rlimit_open_files\0"),
    ("scanf_bytes_consumed\0"),
    ("scanf_match_literal_eof\0"),
    ("scanf_nullbyte_char\0"),
    ("setvbuf_unget\0"),
    ("sigprocmask_internal\0"),
    ("sscanf_eof\0"),
    ("statvfs\0"),
    ("strverscmp\0"),
    ("syscall_sign_extend\0"),
    ("uselocale_0\0"),
    ("wcsncpy_read_overflow\0"),
    ("wcsstr_false_negative\0"),
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
