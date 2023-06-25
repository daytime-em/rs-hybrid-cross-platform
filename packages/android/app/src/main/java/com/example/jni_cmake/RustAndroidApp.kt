package com.example.jni_cmake

import android.app.Application

class RustAndroidApp : Application() {

  companion object {
    init {
      System.loadLibrary("jnilib")
    }
  }
}