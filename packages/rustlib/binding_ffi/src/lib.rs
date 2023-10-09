mod outputs;

use std::ffi::{CStr, c_char, c_void, CString};
use rustlib::{self, FromPrimesResult};

pub use outputs::FoundPrimesFfi;
pub use outputs::free_found_primes;

#[no_mangle]
pub extern "C" fn simple_sieve(up_to: u64) -> FoundPrimesFfi {
    let result = rustlib::fast_sieve(up_to);
    FoundPrimesFfi::from_primes_result(&result)
}

// --- Just playing around below here

/// # Safety
/// This function does not know how/if the calling language will free the value returned
/// from the callback, the `dispose_str` param must point to a function that can free it
/// after it's been copied into a rust value
#[no_mangle]
pub unsafe extern "C" fn invoke_cb_on_string(
  str_in: *const c_char,
  cb: unsafe extern "C" fn(*const c_char, *const c_void) -> *const c_char,
  dispose_str: unsafe extern "C" fn(*const c_char, *const c_void),
  thunk: *const c_void
) -> *mut c_char {
    let input =  String::from(CStr::from_ptr(str_in).to_str().unwrap());

    let result = rustlib::examples::invoke_a_closure_cb(
      input,
      |str| {
        // invoke the callback with a c str
        let cb_chars : *const c_char = cb(CString::new(str).unwrap().into_raw(), thunk);
        let ret = String::from(CStr::from_ptr(cb_chars).to_str().unwrap()); // Copy their chars
        dispose_str(cb_chars, thunk); // caller knows how to free their returned chars
        ret
      }
    );

    CString::new(result).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn just_print() {
  rustlib::examples::just_print();
}

/// # Safety
/// The returned string must be freed with `free_rs_string`
#[no_mangle]
pub unsafe extern "C" fn append_location_by_delegating(from: *const c_char) -> *mut c_char {
  // tbh, probably don't panic all the time
  if from.is_null() {
    panic!("null pointer while append_location_by_delegating");
  }
  let c_str = unsafe { CStr::from_ptr(from) };
  let input = c_str.to_str().unwrap();
  let mut appended = input.to_owned();
  appended.push_str(" via ffi, ");

  let lib_output = rustlib::append_location_by_delegating(appended.as_str());

  CString::new(lib_output).unwrap().into_raw()
}

/// # Safety
/// The returned string must be freed with `free_rs_string`
#[no_mangle]
pub unsafe extern "C" fn append_location(from: *const c_char) -> *mut c_char {
  // tbh, probably don't panic all the time
  if from.is_null() {
    panic!("null pointer while append_location_by_delegating");
  }
  let c_str = unsafe { CStr::from_ptr(from) };
  let input = c_str.to_str().unwrap();
  let mut appended = input.to_owned();
  appended.push_str(" via ffi, ");

  let lib_output = rustlib::examples::append_location(appended.as_str());

  CString::new(lib_output).unwrap().into_raw()
}

/// # Safety
/// c_char must be a pointer to a valid CString 
#[no_mangle]
pub unsafe extern "C" fn free_rs_string(at: *mut c_char) {
  if !at.is_null() { 
    drop(CString::from_raw(at));
  } 
}
