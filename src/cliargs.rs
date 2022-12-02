use tracing::field::Field;
use tracing::field::Visit;
use tracing::field::Value;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct AppArgs {
}

pub fn processArgs() -> AppArgs {
    return AppArgs {}
}
