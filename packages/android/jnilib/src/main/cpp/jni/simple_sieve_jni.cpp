//
// Created by Emily Dixon on 8/23/23.
//
#include <jni.h>
#include <string>
#include "FoundPrimesJni.h"
#include <android/log.h>

extern "C"
JNIEXPORT jobject JNICALL
Java_com_example_jnilib_PrimeSieve_evaluate(JNIEnv *env, jclass clazz, jint up_to) {
    rustlib::FoundPrimes rustPrimes = rustlib::simple_sieve(up_to);

    __android_log_print(
            ANDROID_LOG_DEBUG,
            "SimplePrimeSieve",
            "evaluate(): straight from the return value it's %lu", rustPrimes.prime_count
            );

    auto *jniPrimes = new FoundPrimesJni(env, rustPrimes);
    return jniPrimes->asJavaObject();
}