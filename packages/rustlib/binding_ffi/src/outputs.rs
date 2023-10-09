use std::mem::ManuallyDrop;

use rustlib::FromPrimesResult;

#[repr(C)]
pub struct FoundPrimesFfi {
    pub exec_time_millis: u64,
    pub primes: *mut u64,
    pub up_to: u64,
    prime_count: usize,
}

#[no_mangle]
pub extern "C" fn free_found_primes(found_primes: FoundPrimesFfi) {
    // free the vector of primes.
    unsafe {
        let vec = Vec::from_raw_parts(
            found_primes.primes,
            found_primes.prime_count,
            found_primes.prime_count,
        );
        drop(vec);
    }
}

impl FromPrimesResult for FoundPrimesFfi {
    fn from_primes_result(result: &rustlib::PrimesResult) -> Self {
        let /*mut*/ primes_vec = result.primes.clone();
        //primes_vec.shrink_to_fit(); // When we free this, we want len & capacity to ==
        let mut vec_box = ManuallyDrop::new(primes_vec.into_boxed_slice());
        let exec_time_millis = result.exec_time.subsec_millis() as u64 + result.exec_time.as_secs();
        FoundPrimesFfi {
            prime_count: result.primes_count(),
            exec_time_millis,
            primes: vec_box.as_mut_ptr(),
            up_to: result.up_to,
        }
    }
}
