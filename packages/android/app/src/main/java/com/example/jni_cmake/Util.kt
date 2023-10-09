package com.example.jni_cmake

import kotlin.math.ceil

fun primeDensities(regions: Int, upTo: Long, primes: List<Long>): List<Int> {
  val regionSize = if (upTo < 5) {
    1
  } else {
    ceil(upTo / regions.toDouble()).toInt()
  }

  return primes.groupBy { it / regionSize }.map { entry -> entry.value.size }
}

