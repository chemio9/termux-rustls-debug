extern crate rustls_platform_verifier;

use jni::objects::*;
use jni::{JNIEnv, JNIVersion};
use rustls_platform_verifier::android;
#[export_name = "Java_com_termux_app_TermuxInstaller_rustls"]
extern "C" fn java_init(env: JNIEnv, _class: JClass, context: JObject) {
    android::init_hosted(&env, context).unwrap();
}
