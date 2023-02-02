use clap::Parser;

pub const CONFIG_FILE: &'static str = "cfg.toml";

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub(crate) struct Args {
    #[clap(short = 'c', long, default_value = CONFIG_FILE)]
    pub(crate) config_path: String,
}
