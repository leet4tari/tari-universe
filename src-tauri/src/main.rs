// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
  let _ = fix_path_env::fix(); // fixes the app from crashing on macOS, see https://github.com/tauri-apps/tauri/issues/7063

  // https://docs.rs/diesel_migrations/latest/diesel_migrations/macro.embed_migrations.html#automatic-rebuilds
  println!("cargo:rerun-if-changed=./migrations");
  wallet_daemon_test_lib::run()
}
