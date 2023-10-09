use rustlib::examples::invoke_a_closure_cb;
use rustlib::examples::just_print as print_something;
use rustlib::PrimesResult;
use swift_ffi::StringAppendCallback;

#[swift_bridge::bridge]
mod swift_ffi {
    extern "Rust" {
        type FoundPrimes;

        #[swift_bridge(swift_name = "primeCount")]
        fn get_count(&self) -> usize;

        #[swift_bridge(swift_name = "upToNumber")]
        fn get_up_to(&self) -> u64;

        #[swift_bridge(swift_name = "primes")]
        fn get_primes(&self) -> Vec<u64>;
    }

    extern "Rust" {
        type SimplePrimeFinder;

        #[swift_bridge(init)]
        fn new() -> SimplePrimeFinder;

        #[swift_bridge(swift_name = "findPrimesWithSimpleSieve")]
        fn find_primes_slow(&self, #[swift_bridge(label = "upTo")] up_to: u64) -> FoundPrimes;

        #[swift_bridge(swift_name = "findPrimesWithFastSieve")]
        fn find_primes_fast(&self, #[swift_bridge(label = "upTo")] up_to: u64) -> FoundPrimes;
    }

    extern "Rust" {
        // Just playing around
        #[swift_bridge(swift_name = "printViaPrivateDelegate")]
        fn rustlib_just_print();
        #[swift_bridge(swift_name = "printViaRedeclareMethod")]
        fn print_something(); // symbols in modules must be declared with `use as`
        #[swift_bridge(swift_name = "didIAppear")]
        fn try_regenerating_bindings(); // You can't create bindings for functions with implementations
        #[swift_bridge(swift_name = "doACallback")]
        fn append_by_cb(
            #[swift_bridge(label = "startingWith")] start: String,
            callback: StringAppendCallback, // Sending Closures to Rust this way is not supported
        ) -> String;
    }

    extern "Swift" {
        // Closures can't currently be sent from Swift, so you need to define a swift type
        //  and bind a callback function like this to call callbacks from Swift
        type StringAppendCallback;

        // Note that this name must match the *label* of the swift param, not its name
        fn invoke(self, from: String) -> String;
    }
}

// We can make an opaque Swift type from new Rust structs
pub struct SimplePrimeFinder;
impl SimplePrimeFinder {
    fn new() -> Self {
        SimplePrimeFinder {}
    }

    fn find_primes_fast(&self, up_to: u64) -> FoundPrimes {
        FoundPrimes {
            internal_result: rustlib::fast_sieve(up_to),
        }
    }

    fn find_primes_slow(&self, up_to: u64) -> FoundPrimes {
        FoundPrimes {
            internal_result: rustlib::simple_sieve(up_to),
        }
    }
}

impl FoundPrimes {
    fn get_count(&self) -> usize {
        self.internal_result.primes_count() 
    }

    fn get_up_to(&self) -> u64 {
      self.internal_result.up_to
    }

    fn get_primes(&self) -> Vec<u64> {
        let mut prime_list: Vec<u64> = Vec::new();
        for prime in self.internal_result.primes.iter() {
            prime_list.push(*prime);
        }
        prime_list
    }
}

pub struct FoundPrimes {
    internal_result: PrimesResult,
}

fn append_by_cb(from: String, cb: StringAppendCallback) -> String {
    invoke_a_closure_cb(from, move |it: String| cb.invoke(it))
}

pub fn just_panic() {
    panic!("ok this should do something");
}

fn try_regenerating_bindings() {}

fn rustlib_just_print() {
    rustlib::examples::just_print();
}
