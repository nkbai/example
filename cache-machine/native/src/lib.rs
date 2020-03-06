#[macro_use]
extern crate lazy_static;

extern crate neon;

extern crate shared_memory;

use neon::prelude::*;
use neon::register_module;
use shared_memory::*;
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::sync::Mutex;

#[derive(SharedMemCast)]
struct ShmemStructCache {
  num_slaves: u32,
  message: [u8; 256],
}

static GLOBAL_LOCK_ID: usize = 0;

fn create_open_mem(operator: f64, object: String) -> Result<String, SharedMemError> {
  let shmem = match SharedMem::create_linked("shared_mem.link", LockType::Mutex, 4096) {
    Ok(v) => v,
    Err(SharedMemError::LinkExists) => SharedMem::open_linked("shared_mem.link")?,
    Err(e) => return Err(e),
  };

  if shmem.num_locks() != 1 {
    return Ok(String::from("Expected to only have 1 lock in shared mapping !"));
  }

  if operator == 0 as f64 {
    set_cache(shmem, object)
  } else {
    get_cache(shmem)
  }
}

fn set_cache(mut shmem: SharedMem, object: String) -> Result<String, SharedMemError> {
  {
    let mut shared_state = shmem.wlock::<ShmemStructCache>(GLOBAL_LOCK_ID)?;
    let set_string: CString = CString::new(object.as_str()).unwrap();
    shared_state.message[0..set_string.to_bytes_with_nul().len()]
      .copy_from_slice(set_string.to_bytes_with_nul());
  }
  Ok("".to_owned())
}

fn get_cache(mut shmem: SharedMem) -> Result<String, SharedMemError> {
  let mut result = "";
  {
    let shared_state = shmem.rlock::<ShmemStructCache>(GLOBAL_LOCK_ID)?;
    let shmem_str: &CStr = unsafe { CStr::from_ptr(shared_state.message.as_ptr() as *mut i8) };
    result = shmem_str.to_str().unwrap();
  }

  Ok(result.to_owned())
}


fn cache_opeator(mut cx: FunctionContext) -> JsResult<JsString> {
  let operator = cx.argument::<JsNumber>(0)?.value();
  let set_value = cx.argument::<JsString>(1)?.value();
  match create_open_mem(operator, set_value) {
    Ok(v) => Ok(cx.string(v)),
    Err(e) => Ok(cx.string("error")),
  }
}

register_module!(mut m, {
  m.export_function("opeator", cache_opeator)?;
  Ok(())
});
