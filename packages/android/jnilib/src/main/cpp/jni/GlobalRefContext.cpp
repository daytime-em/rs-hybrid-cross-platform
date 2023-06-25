//
// Created by Emily Dixon on 8/23/23.
//

#include "GlobalRefContext.h"

GlobalRefContext::GlobalRefContext(JNIEnv *env, jobject object) {
    this->javaRef = env->NewGlobalRef(object);
}

void GlobalRefContext::deleteRef(JNIEnv *env) {
    if (this->isRefValid()) {
        // Delete the global ref
        env->DeleteGlobalRef(this->javaRef);
        this->javaRef = nullptr;
    }
}
