#![cfg_attr(
  all(not(debug_assertions), target_os = "macos"),
  windows_subsystem = "none"
)]

mod frontmost_app;  // 声明你的文件
use frontmost_app::{get_frontmost_app_path, get_focused_app_bundle_identifier, open_app, hide_frontmost_app, get_bundle_identifier};

use std::{collections::HashMap, sync::Mutex};
use tauri::State;
use serde_json::{json, to_string};


// here we use Mutex to achieve interior mutability
struct Storage {
  store: Mutex<HashMap<String, String>>,
}
impl Storage {
  fn get_entries_as_json(&self) -> Result<String, serde_json::Error> {
    let lock = self.store.lock().unwrap();
    let entries: Vec<_> = lock.iter().map(|(key, value)| json!({ "key": key, "value": value })).collect();
    to_string(&entries)
  }
  fn get_key_for_val_as_json(&self, key: String) -> Result<String, Box<dyn std::error::Error>> {
    let lock = self.store.lock().unwrap();

    match lock.get(&key) {
      Some(val) => Ok(val.clone()),
      None => Err(format!("Key '{}' not found.", key).into()),
    }
  }
}

#[tauri::command]
fn load_storage (storage: State<Storage>) -> String  {
  storage.get_entries_as_json().unwrap()
}

#[tauri::command]
fn load_specified_storage (storage: State<Storage>, key: String) -> Option<String>  {
  storage.get_key_for_val_as_json(key).ok()
}

#[tauri::command]
fn storage_insert(key: String, value: String, storage: State<Storage>) {
  // mutate the storage behind the Mutex
  storage.store.lock().unwrap().insert(key, value);
}

#[tauri::command]
fn storage_delete(key: String, storage: State<Storage>) {
  // mutate the storage behind the Mutex
  storage.store.lock().unwrap().remove(&key);
}

fn main() {
  tauri::Builder::default()
    .manage(Storage { store: Default::default() })
    .invoke_handler(tauri::generate_handler![load_storage, load_specified_storage, storage_insert, storage_delete, get_frontmost_app_path, open_app, hide_frontmost_app, get_bundle_identifier, get_focused_app_bundle_identifier])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}