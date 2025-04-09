
pub mod timecalc;
pub mod noise;
pub mod cli;
pub mod diff;
pub mod my_strings;

pub enum DError {
    CantReadFile,
    CantWriteFile,
    CantParse,
}

impl std::fmt::Display for DError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DError::CantParse => write!(f, "Невозможно разобрать строку."),
            DError::CantReadFile => write!(f, "Невозможно прочитать файл."),
            DError::CantWriteFile => write!(f, "Невозможно записать в файл."),
            // _ => write!(f,"")
        }
    }
}
