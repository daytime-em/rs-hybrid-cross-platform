#include <jni.h>
#include <string>
#include "rustlib.h"

extern "C"
JNIEXPORT jstring JNICALL
Java_com_example_jnilib_NativeLib_stringFromJNI(
        JNIEnv *env,
        jobject /* this */) {
    std::string hello = "Hello from C++";
    return env->NewStringUTF(hello.c_str());
}

extern "C"
JNIEXPORT jstring JNICALL
Java_com_example_jnilib_NativeLib_printCallLocation(JNIEnv *env, jobject thiz, jstring to) {
    const char *inputStr = env->GetStringUTFChars(to, JNI_FALSE);
    std::string fromLocation = std::string(inputStr).append(" via the JNI,");

    char *rustLocation = rustlib::append_location_by_delegating(fromLocation.c_str());
    // Make a JVM string for the caller (JVM owns it)
    jstring ret = env->NewStringUTF(rustLocation);

    rustlib::free_rs_string(
            rustLocation); // Don't use free() or delete! `rustLocation` was allocated by rust
    env->ReleaseStringUTFChars(to, inputStr); // Release the cstr derived from `to`
    return ret;
}

extern "C"
struct CallbackContext {
    JNIEnv *env;
    jmethodID callbackMethod;
    jobject callbackObj;
    jstring callbackStr;
};
extern "C"
const char *callbackFunction(const char *in_str, const void *thunk) {
    auto *context = (CallbackContext *)thunk;
    jstring javaString = context->env->NewStringUTF(in_str);
    context->callbackStr = javaString;

    auto callerString = (jstring) context->env->CallObjectMethod(
            context->callbackObj,
            context->callbackMethod,
            javaString
            );
    if (context->env->ExceptionCheck()) {
        // Let java handle this
        return nullptr;
    }

    const char *retVal = context->env->GetStringUTFChars(callerString, JNI_FALSE);
    return retVal;
}
extern "C"
void disposeJniStr(const char *str, const void *thunk) {
    auto *context = (CallbackContext *)thunk;
    auto jstr = context->callbackStr;
    if (jstr == nullptr || str == nullptr) {
        context->env->FatalError("Expected: jstring and char string");
        return;
    }
    context->env->ReleaseStringUTFChars(jstr, str);
}

extern "C"
JNIEXPORT jstring JNICALL
Java_com_example_jnilib_NativeLib_callACallback(JNIEnv *env, jobject thiz, jstring initial_string,
                                                jobject callback) {
    // Convert our params
    const char *inputStr = env->GetStringUTFChars(initial_string, JNI_FALSE);
    jclass cbClass = env->GetObjectClass(callback);
    jmethodID cbMethod = env->GetMethodID(cbClass, "onString",
                                          "(Ljava/lang/String;)Ljava/lang/String;");
    if (env->ExceptionOccurred()) {
        env->FatalError("Exception caught while looking for callback method");
    }

    // you have to capture context this way because of extern C, even if you used a lambda instead
    // Also, note that using your JNIEnv in a callback is only well-defined if your callback is
    // going to be called synchronously from this thread. JNIEnv is only valid as long as the thread
    // it belongs to is alive, so either guarantee that somehow or use JavaVM->AttachCurrentThread
    // from your other native thread.
    auto *callbackContext = new CallbackContext();
    callbackContext->env = env;
    callbackContext->callbackMethod = cbMethod;
    callbackContext->callbackObj = callback;
    callbackContext->callbackStr = nullptr;

    auto *outputStr =
            rustlib::invoke_cb_on_string(inputStr,
                                         callbackFunction,
                                         disposeJniStr,
                                         callbackContext);

    jstring ret = env->NewStringUTF(outputStr);
    rustlib::free_rs_string(outputStr);
    env->ReleaseStringUTFChars(initial_string, inputStr);

    return ret;
}
