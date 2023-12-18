use clap::Args;

#[derive(Args)]
pub struct FindAppArgs{
  pub name: String,

  // Sort files by name in ascending order
  #[arg(short, long, default_value_t=false)]
  pub sort: bool,

  /// Print to file
  #[arg(short, long, default_value=None)]
  pub filename: Option<String>,

  /// Find files which contains content
  #[arg(short, long, default_value=None)]
  pub in_file: Option<String>,
}
