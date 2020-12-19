#![feature(bool_to_option)]
use anyhow::{anyhow, ensure, Context, Result};
use clap::{App, AppSettings, Arg};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

// TODO: combine these
// TODO: dynamically generate palette file name so I can execute multiple concurrently
const PALETTE_DIR: &'static str = "/tmp/gifer/";
const PALETTE_FILE: &'static str = "/tmp/gifer/palette.png";

struct RunOptions {
    input: PathBuf,
    output: PathBuf,
}

fn main() -> Result<()> {
    let app = build_cli();
    let opts = get_options(app)?;

    // Make GIF Palette
    // ==============================

    fs::create_dir_all(PALETTE_DIR)
        .context("Failed to create temporary directory for palette output")?;

    let mut palette_gen = Command::new("ffmpeg");
    palette_gen
        .arg("-y")// no prompt
        .arg("-i")
        .arg(opts.input.as_os_str())
        .arg("-vf")
        .arg("fps=30,scale=320:-1:flags=lanczos,palettegen=stats_mode=diff")
        .arg(PALETTE_FILE);

    run_command(&mut palette_gen).context("Error using FFmpeg")?;

    // Make GIF
    // ==============================

    let mut palette_gen = Command::new("ffmpeg");
    palette_gen
        .arg("-y") // no prompt
        .arg("-i")
        .arg(opts.input.as_os_str())
        .arg("-i")
        .arg(PALETTE_FILE)
        .arg("-lavfi")
        .arg("fps=30,scale=320:-1:flags=lanczos [x]; [x][1:v] paletteuse=dither=bayer:bayer_scale=5:diff_mode=rectangle")
        .arg(opts.output.as_os_str());

    run_command(&mut palette_gen).context("Error using FFmpeg")?;

    Ok(())
}

fn get_options(app: App) -> Result<RunOptions> {
    let matches = app.get_matches();

    let input = matches
        .value_of("input")
        .map(Path::new)
        .ok_or(anyhow!("Failed to parse input"))?;
    ensure!(input.is_file(), "Input is not a valid file");

    let output = matches
        .value_of("output")
        .map(Path::new)
        .ok_or(anyhow!("Failed to parse output"))?;

    Ok(RunOptions {
        input: input.to_owned(),
        output: output.to_owned(),
    })
}

fn build_cli() -> App<'static, 'static> {
    return App::new("gifer")
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
        );
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
