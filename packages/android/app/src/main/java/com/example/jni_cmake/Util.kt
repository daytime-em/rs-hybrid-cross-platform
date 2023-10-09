package com.example.jni_cmake

fun primeDensities(regions: Int, upTo: Long, primes: List<Long>): List<Int> {
  val regionSize = if (upTo < 5) {
    1
  } else {
    upTo / regions
  }

  // todo - here's hoping
  return primes.groupBy { it / regionSize }.map { entry -> entry.value.size }
}

