use std::{os::raw::c_char, ffi::CStr};
use objc::{class, sel, sel_impl, msg_send, runtime::Object};
use percent_encoding::percent_decode;
use cocoa::foundation::NSString;
use cocoa::base::nil;

#[link(name = "AppKit", kind = "framework")]
extern "C" {}

#[tauri::command]
pub fn get_frontmost_app_path() -> String {
  unsafe {
    let ns_workspace_class = class!(NSWorkspace);
    let shared_workspace: *mut Object = msg_send![ns_workspace_class, sharedWorkspace];
    let app: *mut Object = msg_send![shared_workspace, frontmostApplication];
    let bundle_url: *mut Object = msg_send![app, bundleURL];
    let description_string: *mut Object = msg_send![bundle_url, description];

    let description: *mut c_char = msg_send![description_string, UTF8String];
    let c_str: &CStr = CStr::from_ptr(description);
    let str_slice = c_str.to_string_lossy().into_owned();

    let iter = percent_decode(str_slice.as_bytes());
    let decoded = iter.decode_utf8().unwrap();
    decoded.to_string()
  }
}

#[tauri::command]
pub fn open_app(bundle_id: String) {
  unsafe {
    let workspace: *mut Object = msg_send![class!(NSWorkspace), sharedWorkspace];
    let bundle_id_nsstring = NSString::alloc(nil).init_str(&bundle_id);

    let app_url: *mut Object = msg_send![workspace, URLForApplicationWithBundleIdentifier:bundle_id_nsstring];

    if !app_url.is_null() {
      let _: () = msg_send![workspace, launchApplicationAtURL:app_url options:0 configuration:nil error:nil];
    } else {
      println!("Could not find application with bundle id: {}", bundle_id);
    }
  }
}