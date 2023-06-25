//
// Created by Emily Dixon on 8/23/23.
//

#include "FoundPrimesJni.h"
#include "GlobalRefContext.h"
#include "jni_helper.h"

FoundPrimesJni::FoundPrimesJni(JNIEnv *env, rustlib::FoundPrimes rustPrimes) {
    // Create the JVM object that will interact with us
    jclass jvmFoundPrimes = env->FindClass("com/example/jnilib/model/FoundPrimes");
    jobject javaTwin = JniHelper::allocateObject(env, jvmFoundPrimes);

    jfieldID nativeRefField = JniHelper::getNativeRef(env, javaTwin);
    auto address = (jlong) this;
    env->SetLongField(javaTwin, nativeRefField, address);

    this->jvmRef = std::make_unique<GlobalRefContext>(GlobalRefContext(env, javaTwin));
    this->foundPrimes = rustPrimes;
}

void FoundPrimesJni::release(JNIEnv *env) {
    if (this->jvmRef) {
        // give up our java resources
        this->jvmRef->deleteRef(env);
        this->jvmRef = nullptr;
    }
}

jobject FoundPrimesJni::asJavaObject() {
    if (this->jvmRef == nullptr) {
        return nullptr;
    }
    return this->jvmRef->getJavaObject();
}

FoundPrimesJni::~FoundPrimesJni() {
    // Rust gave us ownership from its allocator, we have to give this back
    rustlib::free_found_primes(this->foundPrimes);
}
