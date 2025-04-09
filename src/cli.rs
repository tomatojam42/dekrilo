use std::process::exit;
use std::{
    io::{stdin, stdout, Write},
    path::PathBuf,
};

use crate::DError;
use crate::{diff, noise::noise, timecalc::timecalc};

pub fn get_input(prompt: &str) -> String {
    let _ = stdout().write(prompt.as_bytes());
    let _ = stdout().flush();
    let mut input = String::new();
    match stdin().read_line(&mut input) {
        Ok(_n) => (),
        Err(e) => {
            println!("Error occured! {e}");
            exit(1)
        }
    }
    input.trim().to_string()
}

pub fn print_error(e: &str) {
    println!("* {e} *")
}

pub fn choose_anything<T, F, Ftest>(variants: &[T], how_to_display: F, test: Ftest) -> &T
where
    F: Fn(&T) -> String,
    Ftest: Fn(&T) -> Result<&T, &'static str>,
{
    for i in 1..=(variants).len() {
        let p = &variants[i - 1];
        let p_formatted = how_to_display(p);
        println!("  ( {} ) {}", i, p_formatted);
    }
    loop {
        let num = get_input("Введите число: ");
        println!();
        match num
            .trim()
            .parse::<usize>()
            .map_err(|_| "Не число!")
            .and_then(|n| match variants.get(n - 1) {
                Some(p) => Ok(p),
                None => Err("Нет такого!"),
            })
            .and_then(&test)
            .map_err(print_error)
        {
            Ok(i) => break i,
            Err(_) => continue,
        }
    }
}

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add noise to image <file>
    #[cfg(feature = "noise")]
    Noise { file: PathBuf },

    /// Calculate time
    #[cfg(feature = "timecalc")]
    Timacalc,

    /// Show where files differ as offset ranges
    #[cfg(feature = "diff")]
    Diff {
        file1: PathBuf,
        file2: PathBuf,
        #[arg(long, default_value_t=false)]
        func: bool
    },
}

pub fn parse() -> Result<(), DError> {
    let args = Cli::parse();
    match &args.command {
        Commands::Noise { file } => noise(file),
        Commands::Timacalc => timecalc(),
        Commands::Diff { file1, file2, func } => {
            let d = diff::DiffArgs {
                file1,
                file2,
                func: *func,
            };
            diff::diff(d)
        },
    }
}
