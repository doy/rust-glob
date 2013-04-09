#[link(name = "glob",
       vers = "0.0.1",
       uuid = "9cb8312d-058c-482f-a0b0-27c48773c74b",
       url  = "https://github.com/doy/rust-glob")];

#[crate_type = "lib"];

use core::libc::{c_char,c_int,c_void,size_t};
use core::path::Path;
use core::unstable::finally::Finally;

pub fn glob (pattern: &str) -> ~[Path] {
    let g = glob_t {
        gl_pathc: 0, gl_pathv: ptr::null(), gl_offs: 0,
        gl_closedir: ptr::null(), gl_readdir: ptr::null(),
        gl_opendir: ptr::null(),
        gl_lstat: ptr::null(), gl_stat: ptr::null(),
    };
    do str::as_c_str(pattern) |c_pattern| {
        unsafe { c::glob(c_pattern, 0, ptr::null(), &g) }
    };
    do(|| {
        let paths = unsafe {
            vec::raw::from_buf_raw(g.gl_pathv, g.gl_pathc as uint)
        };
        do paths.map |&c_str| {
            Path(unsafe { str::raw::from_c_str(c_str) })
        }
    }).finally {
        unsafe { c::globfree(&g) };
    }
}

struct glob_t {
    gl_pathc: size_t,
    gl_pathv: **c_char,
    gl_offs:  size_t,

    // these are nonstandard
    gl_closedir: *c_void,
    gl_readdir:  *c_void,
    gl_opendir:  *c_void,
    gl_lstat:    *c_void,
    gl_stat:     *c_void,
}

extern mod c {
    fn glob(pattern: *c_char, flags: c_int,
            errfunc: *c_void, pglob: *glob_t) -> c_int;
    fn globfree(pglob: *glob_t);
}

#[test]
fn glob_test () {
    let paths = glob("*.rs");
    assert!(paths.len() == 1);
    assert!(paths[0].exists());
    assert!(paths[0].to_str() == ~"glob.rs");
}

#[test]
fn no_glob_test () {
    let paths = glob("*.nonexistent");
    assert!(paths.len() == 0);
}
