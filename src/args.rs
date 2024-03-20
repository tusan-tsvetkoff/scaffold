use std::{fmt, str::FromStr};

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FinishArgs {
    Done(&'static str),
    Exit(&'static str),
}

type MatchedArgs = (FinishArgs, String);

impl FinishArgs {
    pub fn message(&self) -> &'static str {
        match self {
            FinishArgs::Done(message) | FinishArgs::Exit(message) => message,
        }
    }

    pub fn is_done(&self) -> bool {
        matches!(self, FinishArgs::Done(_))
    }

    pub fn is_exit(&self) -> bool {
        matches!(self, FinishArgs::Exit(_))
    }

    pub fn is_valid_arg(s: &str) -> Result<MatchedArgs> {
        let s = s.trim().to_lowercase();
        let done = DONE.iter().any(|&x| s.ends_with(x));
        let exit = EXIT.iter().any(|&x| s.ends_with(x));
        match (done, exit) {
            (true, false) => Ok((FinishArgs::Done(DONE_MESSAGE), s)),
            (false, true) => Ok((FinishArgs::Exit(EXIT_MESSAGE), s)),
            _ => Err(InvalidArgumentError.into()),
        }
    }
}

const DONE: [&str; 3] = ["(done)", "(d)", "(finish)"];
const EXIT: [&str; 3] = ["(exit)", "(e)", "(abort)"];

const DONE_MESSAGE: &str = "Finished creating project template!";
const EXIT_MESSAGE: &str = "Aborted project template creation!";

impl FromStr for FinishArgs {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let (arg, _) = FinishArgs::is_valid_arg(s)?;
        Ok(arg)
    }
}

#[derive(Debug)]
struct InvalidArgumentError;

impl fmt::Display for InvalidArgumentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid argument provided")
    }
}

impl std::error::Error for InvalidArgumentError {}
