package com.example.jni_cmake

import kotlin.math.ceil

fun primeDensities(regions: Int, originalUpTo: Long, primes: List<Long>): List<Int> {
  val regionSize = if (originalUpTo <= regions) {
    1
  } else {
    ceil(originalUpTo / regions.toDouble()).toInt()
  }
  val realRegionCount = if (originalUpTo <= regions) {
    originalUpTo.toInt()
  } else {
    regions
  }

  val regionList = Array(realRegionCount) { 0 }
  for (prime in primes) {
    val proportion = prime / regionSize
    val idx = proportion.toInt()

    regionList[idx] = regionList[idx] + 1
  }
  return regionList.asList()
}

