use chrono::NaiveTime;
use regex::{self, Regex};
use std::path::PathBuf;
use std::process::exit;
use std::process::{Command, Stdio};
use std::str::FromStr;
use std::time::Duration;

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
    // println!("{:#?}", args);

    let fl = PathBuf::from_str(&args.input).unwrap();
    get_duration(fl);

    println!("{:#?}", size)
}

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

fn parse_ffmpeg_time(ffmpeg_string: String) {
    let re = Regex::new(r"time=(\d+):(\d+):(\d+).(\d+)").unwrap();
    let (hour, min, sec, mil) = match re.is_match(&ffmpeg_string) {
        true => {
            let re_match = re.captures_iter(&ffmpeg_string).last().unwrap();
            (
                re_match.get(1).unwrap().as_str(),
                re_match.get(2).unwrap().as_str(),
                re_match.get(3).unwrap().as_str(),
                re_match.get(4).unwrap().as_str(),
            )
        }
        false => panic!("\nFailed to match regex for:\n{:#?}", ffmpeg_string),
    };
    dbg!(hour, min, sec, mil);
    //ffmpeg_string
}

/// Parses ffmpeg output string and returns duration in seconds and miliseconds
fn _parse_duration(_ffmpeg_string: String) {
    let duration = "00:00:11";
    let fmt_str = "%H:%M:%S.%09m";
    let no_timezone = NaiveTime::parse_from_str(&duration, fmt_str).unwrap();
    dbg!(no_timezone);
}

fn get_duration(file: PathBuf) {
    let mut cmd = Command::new("ffmpeg");
    cmd.args(&[
        "-hide_banner",
        "-i",
        file.as_os_str().to_owned().to_str().unwrap(),
        "-f",
        "null",
        "-",
    ]);

    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());

    let out = cmd.output();

    let output = match out {
        Ok(st) => match st.status.success() {
            true => String::from_utf8(st.stderr).unwrap(),
            false => panic!(
                "Failed to execute ffmpeg. \n{:#?}\n {:#?}",
                String::from_utf8(st.stderr),
                st.status.success()
            ),
        },
        Err(st) => panic!("\nFailed to execute ffmpeg. \n{:#?}", st),
    };

    let duration = parse_ffmpeg_time(output);
    println!("{:#?}", duration);
}

fn _segment() {}
