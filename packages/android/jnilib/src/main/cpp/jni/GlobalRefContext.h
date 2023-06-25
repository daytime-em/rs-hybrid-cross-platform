//
// Created by Emily Dixon on 8/23/23.
//

#ifndef RUST_ANDROID_PROJECT_GLOBALREFCONTEXT_H
#define RUST_ANDROID_PROJECT_GLOBALREFCONTEXT_H

#import "jni.h"

/**
 * Creates and holds a Global Ref on the supplied jobject. You can access the jobject that's referenced
 */
class GlobalRefContext {
public:
    GlobalRefContext(JNIEnv *, jobject);

    /**
     * Get a jobject representing a global ref to the object being wrapped
     * @return
     */
    jobject getJavaObject() {
        return this->javaRef;
    }

    /**
     * True if the ref to the jvm object is valid
     * @return
     */
    bool isRefValid() {
        return this->javaRef;
    }

    /**
     * Deletes the underlying jvm data. This requires a valid JNIEnv from a calling thread. That's
     * assumed to be a JVM thread calling `external fun release()` from the JVM.
     */
    void deleteRef(JNIEnv *);
private:
   jobject javaRef = nullptr;
};


#endif //RUST_ANDROID_PROJECT_GLOBALREFCONTEXT_H
