use crate::commands::*;

pub fn week6_category() {
    let command_handler = CommandHandler::new("command", [
        Command::new("panic", panic),
        Command::new("catch_unwind", catch_unwind),
        Command::new("error", error),
        Command::new("error propagation", error_propagation),
        Command::new("different error", different_error),
        Command::new("thiserror", thiserror),
        Command::new("dynamic error", dynamic_error),
        Command::new("anyhow", anyhow),
        Command::new("raw pointer dereferencing", raw_pointer_dereferencing),
        Command::new("static mut", static_mut),
        Command::new("union", _union),
        Command::new("unsafe function call", unsafe_function_call),
        Command::new("unsafe function", unsafe_function),
        Command::new("extern code call", extern_code_call),
        Command::new("ffi", ffi),
    ].into_iter());

    command_handler.handle();
}

fn panic() {
    let v = vec![10, 20, 30];
    println!("v[100]: {}", v[100]);
}

use std::panic;

fn catch_unwind() {
    let result = panic::catch_unwind(|| {
        println!("hello!");
    });
    assert!(result.is_ok());
    
    // let result = panic::catch_unwind(|| {
        panic!("oh no!");
    // });
    assert!(result.is_err());

    println!("but living");
}

use std::fs;
use std::io::Read;

fn error() {
    let file = fs::File::open("diary.txt");
    match file {
        Ok(mut file) => {
            let mut contents = String::new();
            file.read_to_string(&mut contents);
            println!("Dear diary: {contents}");
        },
        Err(err) => {
            println!("The diary could not be opened: {err}");
        }
    }
}

use std::{io};

fn read_username(path: &str) -> Result<String, io::Error> {
    let username_file_result = fs::File::open(path);
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(err) => return Err(err),
    };

    let mut username = String::new();
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(err) => Err(err),
    }
}

fn error_propagation() {
    //fs::write("config.dat", "alice").unwrap();
    let username = read_username("config.dat");
    println!("username or error: {username:?}");
}

use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::fs::{File};

#[derive(Debug)]
enum ReadUsernameError {
    IoError(io::Error),
    EmptyUsername(String),
}

impl Error for ReadUsernameError {}

impl Display for ReadUsernameError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::IoError(e) => write!(f, "IO error: {e}"),
            Self::EmptyUsername(filename) => write!(f, "Found no username in {filename}"),
        }
    }
}

impl From<io::Error> for ReadUsernameError {
    fn from(err: io::Error) -> ReadUsernameError {
        ReadUsernameError::IoError(err)
    }
}

fn read_username_different(path: &str) -> Result<String, ReadUsernameError> {
    let mut username = String::with_capacity(100);
    File::open(path)?.read_to_string(&mut username)?;
    if username.is_empty() {
        return Err(ReadUsernameError::EmptyUsername(String::from(path)));
    }
    Ok(username)
}

fn different_error() {
    //fs::write("config.dat", "").unwrap();
    let username = read_username_different("config.dat");
    println!("username or error: {username:?}");
}

use thiserror::Error;

#[derive(Debug, Error)]
enum ThisErrorReadUsernameError {
    #[error("Could not read: {0}")]
    IoError(#[from] io::Error),
    #[error("Found no username in {0}")]
    EmptyUsername(String),
}

fn read_username_thiserror(path: &str) -> Result<String, ThisErrorReadUsernameError> {
    let mut username = String::new();
    fs::File::open(path)?.read_to_string(&mut username)?;
    if username.is_empty() {
        return Err(ThisErrorReadUsernameError::EmptyUsername(String::from(path)));
    }
    Ok(username)
}

fn thiserror() {
    //fs::write("config.dat", "").unwrap();
    match read_username_thiserror("config.dat") {
        Ok(username) => println!("Username: {username}"),
        Err(err)     => println!("Error: {err}"),
    }
}

#[derive(Clone, Debug, Eq, Error, PartialEq)]
#[error("Found no username in {0}")]
struct EmptyUsernameError(String);

fn read_username_dyn(path: &str) -> Result<String, Box<dyn Error>> {
    let mut username = String::new();
    fs::File::open(path)?.read_to_string(&mut username)?;
    if username.is_empty() {
        return Err(EmptyUsernameError(String::from(path)).into());
    }
    Ok(username)
}

fn dynamic_error() {
    //fs::write("config.dat", "").unwrap();
    match read_username_dyn("config.dat") {
        Ok(username) => println!("Username: {username}"),
        Err(err)     => println!("Error: {err}"),
    }
}

use anyhow::{Context, Result, bail};

fn read_username_anyhow(path: &str) -> Result<String> {
    let mut username = String::with_capacity(100);
    fs::File::open(path)
        .with_context(|| format!("Failed to open {path}"))?
        .read_to_string(&mut username)
        .context("Failed to read")?;
    if username.is_empty() {
        bail!("Found no username in {path}");
    }
    Ok(username)
}

fn anyhow() {
    //fs::write("config.dat", "").unwrap();
    match read_username_anyhow("config.dat") {
        Ok(username) => println!("Username: {username}"),
        Err(err)     => println!("Error: {err:?}"),
    }
}

fn first_word(text: &str) -> &str {
    match text.find(' ') {
        Some(idx) => &text[..idx],
        None => &text,
    }
}

#[test]
fn test_empty() {
    assert_eq!(first_word(""), "");
}

#[test]
fn test_single_word() {
    assert_eq!(first_word("Hello"), "Hello");
}

#[test]
fn test_multiple_words() {
    assert_eq!(first_word("Hello World"), "Hello");
}

fn helper(a: &str, b: &str) -> String {
    format!("{a} {b}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_helper() {
        assert_eq!(helper("foo", "bar"), "foo bar");
    }
}

/// Shortens a string to the given length.
///
/// ```
/// use playground::shorten_string;
/// assert_eq!(shorten_string("Hello World", 5), "Hello");
/// assert_eq!(shorten_string("Hello World", 20), "Hello World");
/// ```
pub fn shorten_string(s: &str, length: usize) -> &str {
    &s[..std::cmp::min(length, s.len())]
}

fn raw_pointer_dereferencing() {
    let mut num = 5;

    let r1 = &mut num as *mut i32;
    let r2 = r1 as *const i32;

    // Safe because r1 and r2 were obtained from references and so are
    // guaranteed to be non-null and properly aligned, the objects underlying
    // the references from which they were obtained are live throughout the
    // whole unsafe block, and they are not accessed either through the
    // references or concurrently through any other pointers.
    unsafe {
        println!("r1 is: {}", *r1);
        *r1 = 10;
        println!("r2 is: {}", *r2);
    }
}

static mut COUNTER: u32 = 0;

fn add_to_counter(inc: u32) {
    unsafe { COUNTER += inc; }  // 잠재적 데이터 경합!
}

fn static_mut() {
    add_to_counter(42);

    unsafe { println!("COUNTER: {COUNTER}"); }  // 잠재적 데이터 경합!
}

#[repr(C)]
union MyUnion {
    i: u8,
    b: bool,
}

fn _union() {
    let u = MyUnion { i: 42 };
    println!("int: {}", unsafe { u.i });
    println!("bool: {}", unsafe { u.b });  // Undefined behavior!
}

fn unsafe_function_call() {
    let emojis = "🗻∈🌏";

    // Safe because the indices are in the correct order, within the bounds of
    // the string slice, and lie on UTF-8 sequence boundaries.
    unsafe {
        println!("emoji: {}", emojis.get_unchecked(0..4));
        println!("emoji: {}", emojis.get_unchecked(4..7));
        println!("emoji: {}", emojis.get_unchecked(7..11));
    }

    println!("char count: {}", count_chars(unsafe { emojis.get_unchecked(0..7) }));

    // Not upholding the UTF-8 encoding requirement breaks memory safety!
    // println!("emoji: {}", unsafe { emojis.get_unchecked(0..3) });
    // println!("char count: {}", count_chars(unsafe { emojis.get_unchecked(0..3) }));
}

fn count_chars(s: &str) -> usize {
    s.chars().map(|_| 1).sum()
}

/// Swaps the values pointed to by the given pointers.
///
/// # Safety
///
/// The pointers must be valid and properly aligned.
unsafe fn swap(a: *mut u8, b: *mut u8) {
    let temp = *a;
    *a = *b;
    *b = temp;
}

fn unsafe_function() {
    let mut a = 42;
    let mut b = 66;

    // Safe because ...
    unsafe {
        swap(&mut a, &mut b);
    }

    println!("a = {}, b = {}", a, b);
}

extern "C" {
    fn abs(input: i32) -> i32;
}

fn extern_code_call() {
    unsafe {
        // Undefined behavior if abs misbehaves.
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}

use std::mem::size_of_val;
use std::slice;

/// ...
/// # Safety
/// The type must have a defined representation and no padding.
pub unsafe trait AsBytes {
    fn as_bytes(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(self as *const Self as *const u8, size_of_val(self))
        }
    }
}

// Safe because u32 has a defined representation and no padding.
unsafe impl AsBytes for u32 {}

// TODO: remove this when you're done with your implementation.

mod ffi {
    use std::os::raw::{c_char, c_int};
    #[cfg(not(target_os = "macos"))]
    use std::os::raw::{c_long, c_ulong, c_ushort, c_uchar};

    // Opaque type. See https://doc.rust-lang.org/nomicon/ffi.html.
    #[repr(C)]
    pub struct DIR {
        _data: [u8; 0],
        _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
    }

    // Layout according to the Linux man page for readdir(3), where ino_t and
    // off_t are resolved according to the definitions in
    // /usr/include/x86_64-linux-gnu/{sys/types.h, bits/typesizes.h}.
    #[cfg(not(target_os = "macos"))]
    #[repr(C)]
    pub struct dirent {
        pub d_ino: c_ulong,
        pub d_off: c_long,
        pub d_reclen: c_ushort,
        pub d_type: c_uchar,
        pub d_name: [c_char; 256],
    }

    // Layout according to the macOS man page for dir(5).
    #[cfg(all(target_os = "macos"))]
    #[repr(C)]
    pub struct dirent {
        pub d_fileno: u64,
        pub d_seekoff: u64,
        pub d_reclen: u16,
        pub d_namlen: u16,
        pub d_type: u8,
        pub d_name: [c_char; 1024],
    }

    extern "C" {
        pub fn opendir(s: *const c_char) -> *mut DIR;

        #[cfg(not(all(target_os = "macos", target_arch = "x86_64")))]
        pub fn readdir(s: *mut DIR) -> *const dirent;

        // See https://github.com/rust-lang/libc/issues/414 and the section on
        // _DARWIN_FEATURE_64_BIT_INODE in the macOS man page for stat(2).
        //
        // "Platforms that existed before these updates were available" refers
        // to macOS (as opposed to iOS / wearOS / etc.) on Intel and PowerPC.
        #[cfg(all(target_os = "macos", target_arch = "x86_64"))]
        #[link_name = "readdir$INODE64"]
        pub fn readdir(s: *mut DIR) -> *const dirent;

        pub fn closedir(s: *mut DIR) -> c_int;
    }
}

use std::ffi::{CStr, CString, OsStr, OsString};
use std::os::unix::ffi::OsStrExt;

use self::ffi::opendir;

#[derive(Debug)]
struct DirectoryIterator {
    path: CString,
    dir: *mut ffi::DIR,
}

impl DirectoryIterator {
    fn new(path: &str) -> Result<DirectoryIterator, String> {
        // Call opendir and return a Ok value if that worked,
        // otherwise return Err with a message.
        let path = CString::new(path).map_err(|error| format!("invalid path: {error}"))?;
        let dir = unsafe { ffi::opendir(path.as_ptr()) };
        if dir.is_null() {
            Err(format!("Could not open {:?}", path))
        } else {
            Ok(DirectoryIterator { path, dir })
        }
    }
}

impl Iterator for DirectoryIterator {
    type Item = OsString;
    fn next(&mut self) -> Option<OsString> {
        // Keep calling readdir until we get a NULL pointer back.
        let entry = unsafe { ffi::readdir(self.dir) };
        if entry.is_null() {
            return None;
        }

        let d_name = unsafe { CStr::from_ptr((*entry).d_name.as_ptr()) };
        let os_str = OsStr::from_bytes(d_name.to_bytes());
        
        Some(os_str.to_owned())
    }
}

impl Drop for DirectoryIterator {
    fn drop(&mut self) {
        // Call closedir as needed.
        if self.dir.is_null() {
            return;
        }

        unsafe { ffi::closedir(self.dir) };
    }
}

fn ffi() {
    let iter = DirectoryIterator::new(".").unwrap();
    println!("files: {:#?}", iter.collect::<Vec<_>>());
}