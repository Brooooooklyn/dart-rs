use std::marker::PhantomData;
use std::mem;

use dart_sys::*;

pub use dart_sys as sys;

#[derive(Clone, Copy, Debug)]
pub struct Args(Dart_NativeArguments);

impl Args {
  pub fn from_raw(d: Dart_NativeArguments) -> Self {
    Args(d)
  }
}

#[derive(Clone, Copy, Debug)]
pub struct DartNull;

#[derive(Clone, Copy, Debug)]
pub struct Value<T> {
  raw: Dart_Handle,
  _marker: PhantomData<T>,
}

impl <T> Value<T> {
  pub fn to_handle(self) -> Dart_Handle {
    self.raw
  }
}

impl Value<DartNull> {
  pub fn create_null() -> Value<DartNull> {
    let raw = unsafe { Dart_Null() };
    mem::forget(raw);
    Value { raw, _marker: PhantomData }
  }
}

#[macro_export]
macro_rules! register_module {
  ($module_name:ident, $( $x:ident ),*) => {
    use $crate::sys::*;

    #[no_mangle]
    pub unsafe extern "C" fn $module_name(parent_library: Dart_Handle) -> Dart_Handle {
      if Dart_IsError(parent_library) {
        return parent_library;
      }
      let result_code = Dart_SetNativeResolver(parent_library, Some(__dart__bindings__resolve__name__), None);
      if Dart_IsError(result_code) {
        return result_code;
      };

      return Dart_Null();
    }

    extern "C" fn __dart__bindings__resolve__name__(h: $crate::sys::Dart_Handle, _argc: i32, _auto_setup_scope: *mut bool) -> $crate::sys::Dart_NativeFunction {
      use $crate::sys::*;
      use std::ptr;
      use std::ffi::CString;
      use std::mem;

      unsafe {
        if !Dart_IsString(h) {
          return None;
        }
        let mut chars = ptr::null();
        let result = Dart_StringToCString(h, &mut chars);
        if Dart_IsError(result) {
          Dart_PropagateError(result);
          return None;
        }
        let name = CString::from_raw(chars as *mut i8).into_string().expect("Get name string fail");

        match name.as_str() {
          $(
            stringify!($x) => {
              unsafe extern "C" fn __bind__to__dart__ (args: Dart_NativeArguments) {
                let call_result = $x(Args::from_raw(args)).to_handle();
                Dart_SetReturnValue(args, call_result);
              }
              mem::forget(name);
              Some(__bind__to__dart__)
            },
            _ => {
              println!("{} not match", name);
              mem::forget(name);
              None 
            },
          )*
        }
      }
    }
  };
}
