package com.example.jni_cmake

import androidx.appcompat.app.AppCompatActivity
import android.os.Bundle
import android.util.Log
import com.example.jni_cmake.databinding.ActivityMainBinding
import com.example.jnilib.NativeLib
import com.example.jnilib.SimplePrimeSieve
import com.example.jnilib.model.FoundPrimes

class MainActivity : AppCompatActivity() {

  lateinit var viewBinding: ActivityMainBinding

  override fun onCreate(savedInstanceState: Bundle?) {
    super.onCreate(savedInstanceState)
    viewBinding = ActivityMainBinding.inflate(layoutInflater)
    setContentView(viewBinding.root)

    var foundPrimes: FoundPrimes? = null
    val primeGreeting: CharSequence
    try {
      foundPrimes = SimplePrimeSieve.evaluate(100)
      primeGreeting = "${foundPrimes.primeCount} primes found"
      val primeList = foundPrimes.foundPrimes;
      Log.d("MainActivity", "primes: $primeList")
    } finally {
      foundPrimes?.release()
    }

    viewBinding.greeting.text = primeGreeting
//    viewBinding.greeting.text = callSite("MainActivity.onCreate()")
  }

  override fun onResume() {
    super.onResume()

    val str = NativeLib().callACallback("My String!") { "$it Is this" }
    Log.i("MainActivity", "Called with callback: $str")
  }

  private fun callSite(subject: String): String {
    val nativeLib = NativeLib()
    return nativeLib.printCallLocation(subject)
  }
}