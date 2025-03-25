use std::{io::{stdin, stdout, Write}, path::PathBuf};

use crate::{diff::{self, Diffargs}, noise::noise, timecalc::timecalc};
use core::fmt;

pub fn get_input(prompt: &str) -> String{
    let _ = stdout().write(format!("{}",prompt).as_bytes());
    let _ = stdout().flush();
    let mut input = String::new();
    match stdin().read_line(&mut input) {
        Ok(_) => {},
        Err(_) => {},
    }
    input.trim().to_string()
}

pub fn print_error(e: &str) {
    println!("* {e} *")
}

pub fn choose_anything <T, F, Ftest>
    (variants: &Vec<T>, how_to_display: F, test: Ftest) -> &T 
    where
    F: Fn(&T) -> String,
    Ftest: Fn(&T) -> Result<&T, &'static str>
{
    for i in 1..=(&variants).len() {
        let p = &variants[i-1];
        let p_formatted = how_to_display(p);
        println!("  ( {} ) {}", i, p_formatted);
    }
    loop {
        let num = get_input("Введите число: ");
        println!();
        match num.trim().parse::<usize>()
        .map_err(|_| "Не число!")
        .and_then(|n| match variants.get(n-1) {
                Some(p) => Ok(p),
                None => Err("Нет такого!")
            })
        .and_then(|p| test(p))
        .map_err( |e| print_error(e))
        {
            Ok(i) => break i,
            Err(_) => continue
        }
    }
}

/* pub fn choose_script() -> Scripts {
    let variants = Scripts::variants();
    choose_anything(&variants, |i|i.to_string(), |i| Ok(i), ).to_owned()
}

#[derive(Clone)]
pub enum Scripts {
    #[cfg(feature = "noise")]
    Noise,
    #[cfg(feature = "timecalc")]
    Timacalc,
    #[cfg(feature = "diffrange")]
    Diff,
} */

// "noise", "timecalc", "diffrange"

/* impl Scripts {
    pub fn variants() -> Vec<Scripts> {
        vec![
        #[cfg(feature = "noise")]
        Scripts::Noise,
        #[cfg(feature = "timecalc")]
        Scripts::Timacalc,
        #[cfg(feature = "diffrange")]
        Scripts::Diff,
        ]
    }
    pub fn run(self) -> Result<(),()> {
        match self {
            #[cfg(feature = "noise")]
            Scripts::Noise => noise(),
            #[cfg(feature = "timecalc")]
            Scripts::Timacalc => timecalc(),
            #[cfg(feature = "diffrange")]
            Scripts::Diff => diff::diffrange(),
        }
    }
}

impl fmt::Display for Scripts {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            #[cfg(feature = "noise")]
            Self::Noise => write!(f, "Восстановить при проблемах с памятью"),
            #[cfg(feature = "timecalc")]
            Self::Timacalc => write!(f, "Сбросить пароль админа"),
            #[cfg(feature = "diffrange")]
            Self::Diffrange => write!(f, "Узнать версию чипа"),
        }
    }
}
 */

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
    Noise{ file: PathBuf },

    /// Calculate time
    #[cfg(feature = "timecalc")]
    Timacalc,

    /// Show where files differ as offset ranges
    #[cfg(feature = "diff")]
    Diff{ file1: PathBuf, file2: PathBuf },
}

pub fn parse() -> Result<(), ()> {
    let args = Cli::parse();
    match &args.command {
        Commands::Noise { file } => noise(file),
        Commands::Timacalc => timecalc(),
        Commands::Diff { file1, file2 } => {
            let d = Diffargs::default();
            diff::diff(file1,file2,d)
        }
    }
}