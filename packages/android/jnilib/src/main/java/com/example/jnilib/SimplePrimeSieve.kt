package com.example.jnilib

import com.example.jnilib.model.FoundPrimes

/**
 * Finds primes using a basic prime sieve algorithm. Supports values up to [Int.MAX_VALUE]
 */
object SimplePrimeSieve {

  @JvmStatic
  external fun evaluate(upTo: Int): FoundPrimes;
}
