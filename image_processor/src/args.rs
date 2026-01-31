use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[clap(long)]
    pub input: String,
    #[clap(long)]
    pub output: String,
    #[clap(long)]
    pub plugin: String,
    #[clap(long)]
    pub params: String,
    #[clap(long, default_value_t = String::from("target/debug"))]
    pub plugin_path: String
}
