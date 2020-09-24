use rusty_v8::{
    self as v8, Context, ContextScope, Function, FunctionCallback, FunctionCallbackArguments,
    HandleScope, Integer, Isolate, Local, MapFnTo, Object, ReturnValue, Script, Value,
};

// js から呼ぶ関数
fn foo(scope: &mut HandleScope, args: FunctionCallbackArguments, mut rv: ReturnValue) {
    let x = args.get(0).to_int32(scope).unwrap().value() as i32;
    let y = args.get(1).to_int32(scope).unwrap().value() as i32;
    let result = Integer::new(scope, x + y);
    rv.set(result.into());
}

// js から呼び出す関数のセットアップ
fn setup_func(
    scope: &mut v8::ContextScope<HandleScope<Context>>,
    context: Local<Context>,
    name: &str,
    callback: impl MapFnTo<FunctionCallback>,
) {
    let func = Function::new(scope, callback).unwrap();
    let global = context.global(scope);
    let key = v8::String::new(scope, name).unwrap();
    global
        .create_data_property(scope, key.into(), func.into())
        .unwrap();
}

fn main() {
    let platform = v8::new_default_platform().unwrap();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();

    // 環境
    let isolate = &mut v8::Isolate::new(Default::default());

    let scope = &mut v8::HandleScope::new(isolate);
    let context = v8::Context::new(scope);
    let scope = &mut v8::ContextScope::new(scope, context);

    // js から関数呼び出せるように
    setup_func(scope, context, "foo", foo);

    let code = r#"
        let hoge = 4 + 5;
        console.log("hoge", hoge);
        const add = (a, b) => a + b;
        add(5, 6);
        foo(11, 7);
    "#;
    let code = v8::String::new(scope, code).unwrap();
    println!("javascript code: {}", code.to_rust_string_lossy(scope));

    let script = v8::Script::compile(scope, code, None).unwrap();
    let result = script.run(scope).unwrap();
    let result = result.to_string(scope).unwrap();
    println!("result: {}", result.to_rust_string_lossy(scope));
}
