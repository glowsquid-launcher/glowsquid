use tauri::{
  api::{path::app_dir, shell::open},
  command, AppHandle, Config, Manager,
};

#[command]
pub fn get_app_path() -> String {
  app_dir(&Config::default())
    .expect("Could not get app path")
    .join("glowsquid")
    .into_os_string()
    .into_string()
    .expect("Could not convert app path to string")
}

#[command]
// TODO: proper error handling
pub async fn add_new_account(app_handle: AppHandle, dev: bool) -> Result<(), ()> {
  let port = tauri_plugin_oauth::start(None, |url| {
    println!("{}", url);
  })
  .unwrap();

  open(
    &app_handle.shell_scope(),
    if dev {
      format!("localhost:4000/api/auth/start/?port={}", port)
    } else {
        panic!("no production URL in place yet");
    },
    None,
  )
  .unwrap();

  Ok(())
}
