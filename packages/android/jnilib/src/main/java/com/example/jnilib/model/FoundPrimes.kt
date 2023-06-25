package com.example.jnilib.model

/**
 * Primes found by the JNI layer. Cannot be instantiated from the JVM
 */
class FoundPrimes private constructor() {

  val primeCount: Int get() = nativePrimeCount()
  val foundPrimes: List<Long> get() = nativeFoundPrimes().toList()

  @Suppress("unused") // used from the JNI side
  private var nativeRef: Long = 0

  external fun release();

  private external fun nativePrimeCount(): Int
  private external fun nativeFoundPrimes(): LongArray
}