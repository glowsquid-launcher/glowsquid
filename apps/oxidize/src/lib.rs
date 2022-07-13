use tauri::async_runtime::Mutex;

pub mod auth;
pub mod commands;
pub mod error;
pub mod internal_errors;
pub mod playerdb;
pub mod prisma;

pub const CLIENT_ID: &str = "2aa32806-92e3-4242-babc-392ac0f0fd30";
pub struct GlowState {
  pub db: prisma::PrismaClient,
  pub is_dev: Mutex<bool>,
}
