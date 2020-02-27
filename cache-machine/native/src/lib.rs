#[macro_use]
extern crate lazy_static;

extern crate neon;

use neon::prelude::*;
use neon::register_module;
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
  static ref HASHMAP: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

fn hashmap_put(key: String, value: String) {
  HASHMAP.lock().unwrap().insert(key, value);
}

fn hashmap_remove(key: String) {
  HASHMAP.lock().unwrap().remove(&key);
}

fn hashmap_get(key: String) -> String {
  match HASHMAP.lock().unwrap().get(&key) {
    Some(value) => value.to_owned(),
    None => "".to_owned(),
  }
}

fn put(mut cx: FunctionContext) -> JsResult<JsUndefined> {
  let key = cx.argument::<JsString>(0)?.value();
  let value = cx.argument::<JsString>(1)?.value();
  hashmap_put(key, value);
  Ok(cx.undefined())
}

fn remove(mut cx: FunctionContext) -> JsResult<JsUndefined> {
  let key = cx.argument::<JsString>(0)?.value();
  hashmap_remove(key);
  Ok(cx.undefined())
}

fn get(mut cx: FunctionContext) -> JsResult<JsString> {
  let key = cx.argument::<JsString>(0)?.value();
  Ok(cx.string(hashmap_get(key)))
}

register_module!(mut m, {
  m.export_function("put", put)?;
  m.export_function("remove", remove)?;
  m.export_function("get", get)?;
  Ok(())
});
