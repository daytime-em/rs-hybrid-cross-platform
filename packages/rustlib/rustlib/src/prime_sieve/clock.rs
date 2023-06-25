use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Generically reports the time for performance measurements. The included SystemTimeClock returns epoch mllis,
/// but any millisecond-resolution time source can be used.
/// note: WebAssembly users must supply a different Clock, since the wasm environment doesn't support SystemTime
pub trait Clock {
  /// returns some kind of millisecond-resolution time measurement. Could be epoch time, uptime, or whatever
  fn increasing_millis(&self) -> Duration;
}

pub struct SystemTimeClock;
impl SystemTimeClock {
  pub fn new() -> SystemTimeClock {
    SystemTimeClock {}
  }
}
impl Clock for SystemTimeClock {
  fn increasing_millis(&self) -> Duration {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap()
  }
}
