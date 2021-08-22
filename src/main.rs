use std::process::exit;
use std::process::Command;

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

    let size: u64 = parse_to_bytes(&args.size);
    println!("{:#?}", args);

    println!("{:#?}", size)
}

fn segment() {}

/// Takes string of size provided by user and returns u64 of bytes
fn parse_to_bytes(size: &String) -> u64 {
    match bytefmt::parse_to(size, bytefmt::Unit::KIB) {
        Ok(size) => size as u64,
        Err(e) => {
            println!(
            "Can't parse byte format, valid sizes: [KB,KiB,MB,MiB,GB,GiB], \nProvided: {:#?} Example: 5 MiB\nerror: {:#?}",
            size,
            e
        );
            exit(1);
        }
    }
}
