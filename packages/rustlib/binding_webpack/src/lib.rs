use std::ops::Deref;

use js_sys::Array;
use js_sys::Function;
use js_sys::Object;
use js_sys::Reflect;
use wasm_bindgen::prelude::*;
use web_sys::console;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    log_str("Hello world from rust :)");

    Ok(())
}

// ====== JS Functions we can call from here

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

// ====== Functions we provide to JS

#[wasm_bindgen(js_name = fastFindPrimes)]
pub fn fast_find_primes(up_to: Option<i32>) -> JsValue {
    let expected_up_to =
        up_to.unwrap_or_else(|| panic!("An integer up to {} must be provided.", std::i32::MAX));
    let internal_result = internal::fast_sieve(expected_up_to as u64);

    let js_obj = Object::new();
    let ret = js_obj.deref();
    Reflect::set(
        ret,
        &JsValue::from_str("execTimeSecs"),
        &JsValue::from_f64(internal_result.exec_time.as_secs_f64()),
    )
    .unwrap();
    Reflect::set(
        ret,
        &JsValue::from_str("primeCount"),
        &JsValue::from_f64(internal_result.primes.len() as f64)
    )
    .unwrap();

    let result_arr = Array::new();
    for prime in internal_result.primes.iter() {
        result_arr.push(&JsValue::from_f64(*prime as f64));
    }
    Reflect::set(
        ret,
        &JsValue::from_str("foundPrimes"),
        &JsValue::from(result_arr),
    )
    .unwrap();

    ret.clone()
}

#[wasm_bindgen(js_name = simpleFindPrimesInout)]
pub fn simple_find_primes_inout(up_to: Option<i32>, result: &JsValue) {
    // validate input
    let expected_up_to =
        up_to.unwrap_or_else(|| panic!("An integer up to {} must be provided.", std::i32::MAX));
    let result_is_nullish = result.is_null() || result.is_undefined();
    if expected_up_to <= 0 {
        panic!("number up_to must be 1 or greater")
    }
    if result_is_nullish {
        panic!("a result object must be supplied")
    }

    let internal_result = internal::simple_sieve(expected_up_to as u64);

    // Set the primeCount
    Reflect::set(
        result,
        &JsValue::from_str("execTimeSecs"),
        &JsValue::from_f64(internal_result.exec_time.as_secs_f64()),
    )
    .unwrap();
    Reflect::set(
        result,
        &JsValue::from_str("primeCount"),
        &JsValue::from_f64(internal_result.primes.len() as f64)
    )
    .unwrap();
    // Fill the array
    let result_arr = Array::new();
    for num in internal_result.primes.iter() {
        let elem_v = JsValue::from_f64(*num as f64);
        result_arr.push(&elem_v);
    }
    let result_array_k = JsValue::from_str("foundPrimes");
    Reflect::set(result, &result_array_k, &JsValue::from(result_arr)).unwrap();

    // Done!!
}

#[wasm_bindgen(js_name = stringCallback)]
pub fn append_str_cb(initial: Option<String>, thiz: &JsValue, cb: JsValue) -> String {
    let expected_initial = initial.expect("The first argument must be a string");
    let cb_is_nullish = cb.is_null() || cb.is_undefined();
    if cb_is_nullish || !cb.is_function() {
        log("append_str_cb: Required: third argument is a function".to_string());
        expected_initial
    } else {
        internal::invoke_a_closure_cb(expected_initial, move |it| {
            let cb_as_func = Function::from(cb);
            let result = cb_as_func
                .call1(thiz, &JsValue::from_str(it.as_str()))
                .unwrap();
            if result.is_string() {
                result.as_string().unwrap()
            } else {
                String::from("undefined")
            }
        })
    }
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("I changed the file! {}", name));
}

fn log_str(str: &str) {
  console::log_1(&JsValue::from(str));
}

fn log(str: String) {
  console::log_1(&JsValue::from(str));
}

mod internal {
    pub use rustlib::examples::invoke_a_closure_cb;
    pub use rustlib::fast_sieve;
    pub use rustlib::simple_sieve;
}
