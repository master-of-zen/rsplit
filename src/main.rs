use bytefmt;
use clap::{AppSettings, Clap};
use regex::{self, Regex};
use std::fs;
use std::path::PathBuf;
use std::process::exit;
use std::process::{Command, Stdio};
use std::str::FromStr;

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

    let split_size: u64 = parse_to_bytes(&args.size);
    let fl = PathBuf::from_str(&args.input).unwrap();
    let input_duration = get_duration(&fl);
    let input_size = fs::metadata(&fl).unwrap().len();

    fl.canonicalize().unwrap();

    println!("Input length: {}", format_time(input_duration));
    println!("Input size: {}B", input_size);
    println!("Split size: {}B", split_size);

    segmenting(fl, input_duration, split_size)
}

/// Takes string of size provided by user and returns u64 of bytes
fn parse_to_bytes(size: &String) -> u64 {
    match bytefmt::parse_to(size, bytefmt::Unit::B) {
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

/// Parses ffmpeg output string and returns duration in miliseconds
fn parse_ffmpeg_time(ffmpeg_string: String) -> u64 {
    let re = Regex::new(r"time=(\d+):(\d+):(\d+).(\d+)").unwrap();
    let (hour, min, sec, mil) = match re.is_match(&ffmpeg_string) {
        true => {
            let re_match = re.captures_iter(&ffmpeg_string).last().unwrap();
            (
                re_match.get(1).unwrap().as_str().parse::<u64>().unwrap(),
                re_match.get(2).unwrap().as_str().parse::<u64>().unwrap(),
                re_match.get(3).unwrap().as_str().parse::<u64>().unwrap(),
                re_match.get(4).unwrap().as_str().parse::<u64>().unwrap(),
            )
        }
        false => panic!("\nFailed to match regex for:\n{:#?}", ffmpeg_string),
    };

    hour * 60 * 60 * 1000 + min * 60 * 1000 + sec * 1000 + mil
}
/// Gets duration of file in miliseconds
fn get_duration(file: &PathBuf) -> u64 {
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

    parse_ffmpeg_time(output)
}

/// Segmeting input file in while loop until it's exhausted
fn segmenting(input_file: PathBuf, input_duration: u64, split_size: u64) {
    let mut done_length = 0u64;
    let end_length = input_duration;

    let mut segments = 0u64;
    while done_length < end_length {
        segments += 1;

        let output = &input_file.clone().with_file_name(format!(
            "{}_{}.{}",
            input_file.file_stem().unwrap().to_str().unwrap(),
            segments,
            input_file.extension().unwrap().to_str().unwrap(),
        ));

        segment(&input_file, &output, done_length, split_size);

        let chunk_length = get_duration(&output);
        let chunk_size = fs::metadata(output).unwrap().len();
        println!(
            "Chunk: {} Size: {}B, Length: {}.{}s ",
            segments,
            chunk_size,
            chunk_length / 1000,
            chunk_length % 1000
        );

        done_length += chunk_length;
    }
}

fn segment(input_file: &PathBuf, output: &PathBuf, start: u64, split_size: u64) {
    let ffmpeg_ss = format_time(start);

    let mut cmd = Command::new("ffmpeg");

    cmd.args([
        "-hide_banner",
        "-y",
        "-ss",
        &ffmpeg_ss,
        "-i",
        input_file.as_os_str().to_str().unwrap(),
        "-fs",
        &split_size.to_string(),
        "-c",
        "copy",
        output.as_os_str().to_str().unwrap(),
    ]);

    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());

    let out = cmd.output();

    match out {
        Ok(st) => match st.status.success() {
            true => return,
            false => panic!(
                "Failed to execute ffmpeg. \n{:#?}\n {:#?}",
                String::from_utf8(st.stderr),
                st.status.success()
            ),
        },
        Err(st) => panic!("\nFailed to execute ffmpeg. \n{:#?}", st),
    };
}

/// Formats time to HH:MM:SS.m
pub fn format_time(input_duration: u64) -> String {
    let hours = input_duration / 60 / 60 / 1000;
    let minutes = input_duration % 3600000 / 60 / 1000;
    let seconds = input_duration % 60000 / 1000;
    let miliseconds = input_duration % 1000;
    format!(
        "{:.0}:{:.0}:{:.0}.{:.0}",
        hours, minutes, seconds, miliseconds
    )
}
