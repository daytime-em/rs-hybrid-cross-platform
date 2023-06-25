package com.example.jnilib

class NativeLib {

  /**
   * A native method that is implemented by the 'jnilib' native library,
   * which is packaged with this application.
   */
  external fun stringFromJNI(): String

  external fun printCallLocation(to: String): String

  external fun callACallback(initialString: String, callback: Callback): String

  fun interface Callback {
    fun onString(str: String): String
  }
}
