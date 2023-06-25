//
// Created by Emily Dixon on 8/23/23.
//

#ifndef RUST_ANDROID_PROJECT_JNIHELPERS_H
#define RUST_ANDROID_PROJECT_JNIHELPERS_H

#include <jni.h>

class JniHelper {
public:
    static jmethodID findCtor(JNIEnv *env, jclass clazz, const char* signature);
    static jmethodID findMethod(JNIEnv *env, jclass clazz, char* name, char* signature);
    static jobject createObjectVCtor(JNIEnv *env, jclass clazz);
    static jobject allocateObject(JNIEnv *env, jclass clazz);
    static jfieldID getNativeRef(JNIEnv *env, jobject thiz);
    static void* nativeTwinOfObject(JNIEnv *env, jobject thiz);
};

#endif
