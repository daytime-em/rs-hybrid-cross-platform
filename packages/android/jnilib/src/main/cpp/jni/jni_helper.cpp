//
// Created by Emily Dixon on 8/23/23.
//
#include "jni_helper.h"

jmethodID JniHelper::findCtor(JNIEnv *env, jclass clazz, const char* signature) {
    jmethodID id = env->GetMethodID(clazz, "<init>", signature);
    if(env->ExceptionCheck()) {
        env->ExceptionDescribe();
        env->ExceptionClear();
        return nullptr;
    } else {
        return id;
    }
}

jmethodID JniHelper::findMethod(JNIEnv *env, jclass clazz, char* name, char* signature) {
    jmethodID id = env->GetMethodID(clazz, name, signature);
    if(env->ExceptionCheck()) {
        env->ExceptionDescribe();
        env->ExceptionClear();
        return nullptr;
    } else {
        return id;
    }
}

jobject JniHelper::createObjectVCtor(JNIEnv *env, jclass clazz) {
    jmethodID ctor = findCtor(env, clazz, "()V");
    jobject obj = env->NewObject(clazz, ctor);
    if (env->ExceptionCheck()) {
        // Since we called a function written by an end dev, let the exception propagate to them
        return nullptr;
    }
    return obj;
}

jfieldID JniHelper::getNativeRef(JNIEnv *env, jobject thiz) {
    jclass objectClass = env->GetObjectClass(thiz);
    if (env->ExceptionCheck()) {
        env->ExceptionDescribe();
        env->ExceptionClear();
        return nullptr;
    }
    jfieldID nativeRefField = env->GetFieldID(objectClass, "nativeRef", "J");
    if (env->ExceptionCheck()) {
        env->ExceptionDescribe();
        env->ExceptionClear();
        return nullptr;
    }
    return nativeRefField;
}

void* JniHelper::nativeTwinOfObject(JNIEnv *env, jobject thiz) {
    jfieldID nativeRefField = getNativeRef(env, thiz);
    if (nativeRefField == nullptr) {
        return nullptr;
    }

    jlong nativeRefValue = env->GetLongField(thiz, nativeRefField);
    if (env->ExceptionCheck()) {
        env->ExceptionDescribe();
        env->ExceptionClear();
        return nullptr;
    }

    return (void *)nativeRefValue;
}

jobject JniHelper::allocateObject(JNIEnv *env, jclass clazz) {
    return env->AllocObject(clazz);
}
