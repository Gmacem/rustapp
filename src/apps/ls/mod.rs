pub mod app;

use clap::Args;

#[derive(Args)]
pub struct LsArgs{
  pub path: String,
}
