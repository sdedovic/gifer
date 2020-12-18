#![feature(bool_to_option)]
use anyhow::{anyhow, Context, Error, Result};
use clap::{App, AppSettings, Arg};
use std::fs;
use std::process::{Command, Stdio};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

// TODO: combine these
// TODO: dynamically generate palette file name so I can execute multiple concurrently
const PALETTE_DIR: &'static str = "/tmp/gifer/";
const PALETTE_FILE: &'static str = "/tmp/gifer/palette.png";

fn main() -> Result<()> {
    // Build Command Line Interface
    // ============================

    let matches = App::new("gifer")
        .version(VERSION)
        .about("Utility for creating web-friendly gifs.")
        .author("Stevan Dedovic <stevan@dedovic.com>")
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::AllowMissingPositional)
        .arg(
            Arg::with_name("input")
                .help("Sets the input file to use")
                .long("input")
                .short("i")
                .value_name("INFILE")
                .required(true),
        )
        .arg(
            Arg::with_name("output")
                .help("Sets the output file to write")
                .value_name("OUTFILE")
                .required(true),
        )
        .get_matches();

    // Validate "--input" Argument
    // =========================

    let input = matches
        .value_of("input")
        .ok_or(anyhow!("Failed to parse input"))?;

    let metadata =
        fs::metadata(input).with_context(|| format!("Failed to read input {}", input))?;

    metadata
        .is_file()
        .then_some(())
        .ok_or(anyhow!("Value is not a file"))
        .with_context(|| format!("Failed to read input {}", input))?;

    // Validate OUTPUT Argument
    // =========================

    let output = matches
        .value_of("output")
        .ok_or(anyhow!("Failed to parse output"))?;

    // Make GIF Palette
    // ==============================

    fs::create_dir_all(PALETTE_DIR)
        .context("Failed to create temporary directory for palette output")?;

    let mut palette_gen = Command::new("ffmpeg");
    palette_gen
        .arg("-y") // no prompt
        .arg("-i")
        .arg(input)
        .arg("-filter_complex")
        .arg("[0:v] palettegen")
        .arg(PALETTE_FILE);

    run_command(&mut palette_gen).context("Error using FFmpeg")?;

    // Make GIF
    // ==============================

    let mut palette_gen = Command::new("ffmpeg");
    palette_gen
        .arg("-y") // no prompt
        .arg("-i")
        .arg(input)
        .arg("-i")
        .arg(PALETTE_FILE)
        .arg("-lavfi")
        .arg("paletteuse=alpha_threshold=128")
        .arg("-gifflags")
        .arg("-offsetting")
        .arg(output);

    run_command(&mut palette_gen).context("Error using FFmpeg")?;

    Ok(())
}

fn run_command(command: &mut Command) -> Result<()> {
    let child = command
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()?;

    let output = child.wait_with_output()?;

    let stderr = String::from_utf8_lossy(&output.stderr);

    let status = output
        .status
        .code()
        .ok_or(anyhow!("Unable to interpret status code"))?;

    (status == 0)
        .then_some(())
        .ok_or(anyhow!("Bad status code of {:}\n{:}", status, stderr))
}
