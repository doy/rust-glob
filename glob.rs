#[link(name = "glob",
       vers = "0.0.2",
       uuid = "9cb8312d-058c-482f-a0b0-27c48773c74b",
       url  = "https://github.com/doy/rust-glob")];

#[crate_type = "lib"];

use std::{ str, ptr, vec };
use std::libc::types::os::common::posix01::glob_t;
use std::libc::funcs::posix01::glob;
use std::path::Path;
use std::unstable::finally::Finally;

pub fn glob (pattern: &str) -> ~[Path] {
    let mut g = glob_t {
        gl_pathc: 0, gl_pathv: ptr::null(), gl_offs: 0,
        __unused1: ptr::null(), __unused2: ptr::null(), __unused3: ptr::null(), __unused4: ptr::null(), __unused5: ptr::null()
    };
    do str::as_c_str(pattern) |c_pattern| {
        unsafe { glob::glob(c_pattern, 0, ptr::null(), &mut g) }
    };
    do(|| {
        let paths = unsafe {
            vec::raw::from_buf_raw(g.gl_pathv, g.gl_pathc as uint)
        };
        do paths.map |&c_str| {
            Path(unsafe { str::raw::from_c_str(c_str) })
        }
    }).finally {
        unsafe { glob::globfree(&mut g) };
    }
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
