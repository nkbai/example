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

static mut shmem_golbal: Option<shared_memory::SharedMem> = None;

fn create_open_mem() -> Result<shared_memory::SharedMem, SharedMemError> {
  let shmem = match SharedMem::create_linked("shared_mem.link", LockType::Mutex, 4096) {
    Ok(v) => v,
    Err(SharedMemError::LinkExists) => SharedMem::open_linked("shared_mem.link")?,
    Err(e) => return Err(e),
  };

  if shmem.num_locks() != 1 {
    return Err(SharedMemError::InvalidHeader);
  }
  Ok(shmem)
}

fn set_cache(set_cache: String) -> Result<String, SharedMemError> {
  {
    let shared_state =  shmem_golbal.unwrap().wlock::<ShmemStructCache>(GLOBAL_LOCK_ID)?;
    let set_string: CString = CString::new(set_cache.as_str()).unwrap();
    shared_state.message[0..set_string.to_bytes_with_nul().len()]
      .copy_from_slice(set_string.to_bytes_with_nul());
  }
  Ok("".to_owned())
}

fn get_cache() -> Result<String, SharedMemError> {
  // let key = cx.argument::<JsString>(0)?.value();
  let mut result = "";
  {
    let shmem = shmem_golbal.unwrap();
    let shared_state = shmem.rlock::<ShmemStructCache>(GLOBAL_LOCK_ID)?;
    let shmem_str: &CStr = unsafe { CStr::from_ptr(shared_state.message.as_ptr() as *mut i8) };
    result = shmem_str.to_str().unwrap();
  }

  Ok(result.to_owned())
}

fn get(mut cx: FunctionContext) -> JsResult<JsString> {
  match get_cache() {
    Ok(v) => Ok(cx.string(v)),
    Err(e) => Ok(cx.string("error")),
  }
}

fn set(mut cx: FunctionContext) -> JsResult<JsString> {
  let value = cx.argument::<JsString>(0)?.value();
  match set_cache(value) {
    Ok(v) => Ok(cx.string(v)),
    Err(e) => Ok(cx.string("error")),
  }
}

register_module!(mut m, {
  unsafe {
    shmem_golbal = match create_open_mem() {
      Ok(v) => Some(v),
      _ => None,
    };
  }

  m.export_function("get", get)?;
  m.export_function("set", set)?;
  Ok(())
});
