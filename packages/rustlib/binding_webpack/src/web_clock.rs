use wasm_bindgen::prelude::*;

use wasm_timer::{SystemTime as WasmSystemTime, UNIX_EPOCH};

use std::time::Duration;
use rustlib::Clock;


/// A source of monotonic millis that is available from the WebAssembly environment.
/// This one is based on the wasm_timer library, which emulates some standard time-and-date APIs 
/// in wasm
pub struct WasmClock;
impl WasmClock {
  pub fn new() -> WasmClock {
    WasmClock {}
  }
}
impl Clock for WasmClock {
    fn increasing_millis(&self) -> Duration {
      WasmSystemTime::now().duration_since(UNIX_EPOCH).unwrap()
    }
}