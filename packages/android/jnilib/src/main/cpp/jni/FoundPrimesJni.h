//
// Created by Emily Dixon on 8/23/23.
//

#ifndef RUST_ANDROID_PROJECT_FOUNDPRIMESJNI_H
#define RUST_ANDROID_PROJECT_FOUNDPRIMESJNI_H

#include <cstdint>
#include <vector>
#include <memory>
#include "rustlib.h"
#include "jni.h"
#include "GlobalRefContext.h"
#include "jni_helper.h"
#include <android/log.h>

class FoundPrimesJni {
public:
    /**
     * Create a FoundPrimesJni from the "rust direction", ie, instantiating the JVM object to be
     * returned
     */
    FoundPrimesJni(JNIEnv*, rustlib::FoundPrimesFfi);
    ~FoundPrimesJni();

    jobject asJavaObject();

    uint64_t getPrimeCount() {
        __android_log_print(
                ANDROID_LOG_DEBUG,
                "FoundPrimes",
                "prime count hot off the ffi %lu", this->foundPrimes.prime_count
                );
        return this->foundPrimes.prime_count;
    }

    uint64_t *getPrimes() {
        return this->foundPrimes.primes;
    }

    void release(JNIEnv*);
private:
    rustlib::FoundPrimesFfi foundPrimes;
    // sorta-raii for the jni. This object shares ownership of itself with the JVM until the JVM
    // calls release() to give it back. When/how the jvm code chooses to call release() is not the
    // concern of the jni layer.
    std::unique_ptr<GlobalRefContext> jvmRef;
};


#endif //RUST_ANDROID_PROJECT_FOUNDPRIMESJNI_H
