use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct NoNotesInListError;

impl Error for NoNotesInListError {
    fn description(&self) -> &str {
        "There are no notes in the NotesList"
    }
}

impl fmt::Display for NoNotesInListError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Yea error lol")
    }
}

#[derive(Debug)]
pub struct WrongSampleRateError;

impl Error for WrongSampleRateError {
    fn description(&self) -> &str {
        "Custom Function returned PCM data at a different sample rate than the project default"
    }
}

impl fmt::Display for WrongSampleRateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Yea error lol")
    }
}

pub fn check_correct_frequency(frequency: &f64) -> Result<(), NotValidFrequencyError> {
    if !frequency.is_normal() && frequency.is_sign_positive() {
        Err(NotValidFrequencyError {})
    } else {
        Ok(())
    }
}

#[derive(Debug)]
pub struct NotValidFrequencyError;

impl Error for NotValidFrequencyError {
    fn description(&self) -> &str {
        "Value can not be a frequency"
    }
}

impl fmt::Display for NotValidFrequencyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Yea error lol")
    }
}

#[derive(Debug)]
pub struct MissingBaseFrequencyError;

impl Error for MissingBaseFrequencyError {
    fn description(&self) -> &str {
        "base_frequency value is not defined but needed"
    }
}

impl fmt::Display for MissingBaseFrequencyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Yea error lol")
    }
}

#[derive(Debug)]
pub struct OriginalKeyNotFoundError;

impl Error for OriginalKeyNotFoundError {
    fn description(&self) -> &str {
        "PCMAudio for key base_frequency is missing from keys HashMap"
    }
}

impl fmt::Display for OriginalKeyNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Yea error lol")
    }
}
