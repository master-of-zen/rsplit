use bytefmt;
use clap::{AppSettings, Clap};

#[derive(Clap, Debug)]
#[clap(version = "0.1", author = "Zen <master_of_zen@protonmail.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Args {
    #[clap(short, long)]
    input: String,

    #[clap(short, long)]
    size: String,
}

fn main() {
    let args = Args::parse();

    let size: u64 = bytefmt::parse_to(&args.size, bytefmt::Unit::KIB).unwrap() as u64;

    println!("{:#?}", args);
}
