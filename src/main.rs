fn main() {

    deno_core::JsRuntime::new(deno_core::RuntimeOptions::default());

    unsafe {
        v8::V8::dispose();
    }

    v8::V8::dispose_platform();
}
