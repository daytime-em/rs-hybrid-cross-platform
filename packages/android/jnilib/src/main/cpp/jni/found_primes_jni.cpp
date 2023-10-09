
//
// Created by Emily Dixon on 8/23/23.
//

#include "FoundPrimesJni.h"
#include <android/log.h>

extern "C"
JNIEXPORT void JNICALL
Java_com_example_jnilib_model_FoundPrimes_release(JNIEnv *env, jobject thiz) {
    auto *nativeTwin = (FoundPrimesJni *) JniHelper::nativeTwinOfObject(env, thiz);
    __android_log_print(
            ANDROID_LOG_VERBOSE,
            "FoundPrimes",
            "Cleaning up rust mem"
    );
    if (nativeTwin) {
        // Release the refs we're holding to the java object so it can be gc'd, then delete the
        //  native object.
        nativeTwin->release(env);
        delete nativeTwin;
    }
}
extern "C"
JNIEXPORT jint JNICALL
Java_com_example_jnilib_model_FoundPrimes_nativePrimeCount(JNIEnv *env, jobject thiz) {
    auto *nativeTwin = (FoundPrimesJni *) JniHelper::nativeTwinOfObject(env, thiz);
    if (nativeTwin) {
        return static_cast<jint>(nativeTwin->getPrimeCount());
    } else {
        return -1;
    }
}
extern "C"
JNIEXPORT jlongArray JNICALL
Java_com_example_jnilib_model_FoundPrimes_nativeFoundPrimes(JNIEnv *env, jobject thiz) {
    auto *nativeTwin = (FoundPrimesJni *) JniHelper::nativeTwinOfObject(env, thiz);
    if (nativeTwin) {
        uint64_t *arrayPtr = nativeTwin->getPrimes();
        uint64_t primeCount = nativeTwin->getPrimeCount();
        jlongArray out = env->NewLongArray((jsize)primeCount);

        __android_log_print(
                ANDROID_LOG_DEBUG,
                "FoundPrimes",
                "YAY! Found primeCount %lu", primeCount
                );

        env->SetLongArrayRegion(
                out,
                0,
                (jsize) primeCount,
                (jlong *) arrayPtr
        );

        return out;
    } else {
        return nullptr;
    }
}

extern "C"
JNIEXPORT jint JNICALL
Java_com_example_jnilib_model_FoundPrimes_nativeUpTo(JNIEnv *env, jobject thiz) {
    auto *nativeTwin = (FoundPrimesJni *) JniHelper::nativeTwinOfObject(env, thiz);
    if (nativeTwin) {
        return static_cast<jint>(nativeTwin->getUpTo());
    } else {
        return -1;
    }
}