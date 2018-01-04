extern crate ordered_float;

use std::collections::HashMap;
use std::cmp::max;
use ordered_float::NotNaN;
use std::result::Result;
use std::error::Error;

pub mod utils;
mod error;

static mut SAMPLE_RATE: u32 = 48_000;

pub fn get_project_sample_rate() -> u32 {
    unsafe {
        SAMPLE_RATE
    }
}

pub fn set_project_sample_rate(new_sr: u32) {
    unsafe {
        SAMPLE_RATE = new_sr;
    }
}

#[derive(Clone)]
pub struct PCMAudio {
    pub samples: Vec<Vec<f64>>,
    pub sample_rate: u32
}

impl PCMAudio {
    pub fn change_pitch(&mut self, original_freq: &f64, target_freq: &f64) -> Result<(), error::NotValidFrequencyError> {
        error::check_correct_frequency(original_freq)?;
        error::check_correct_frequency(target_freq)?;
        unimplemented!();
        Ok(())
    }
}

#[derive(Clone)]
pub struct MusicSequencer {
    pub pcm: PCMAudio,
    pub note_list: NoteList,
    pub instrument_list: InstrumentList
}

impl MusicSequencer {
    pub fn gen_instrument_keys(&mut self) -> Result<(), Box<Error>> {
        for (instrument_id, frequencies) in &self.note_list.list_frequencies_used_by_instruments().unwrap() {
            let instrument = self.instrument_list.instruments.get_mut(instrument_id).unwrap();
            for frequency in frequencies {
                instrument.gen_key(frequency)?;
            }
        }
        Ok(())
    }
}

#[derive(Clone, Default)]
pub struct NoteList{
    pub notes: Vec<Note>
}

impl NoteList {
    pub fn new() -> NoteList {
        NoteList {
            notes: Vec::new(),
        }
    }
    pub fn add_note(&mut self, new_note: Note) {
        self.notes.push(new_note);
    }
    pub fn merge_other(&mut self, other: &mut NoteList) {
        self.notes.append(&mut other.notes);
    }
    pub fn sort_by_time(&mut self) {
        self.notes.sort_by(|a, b| a.start_at.cmp(&b.start_at));
    }
    pub fn calc_max_notes_at_once(&mut self) -> Result<u32, Box<Error>> {
        if self.notes.is_empty() {
            return Err(Box::new(error::NoNotesInListError {}));
        }
        self.sort_by_time();
        let mut max_notes_at_once = 1u32;
        let mut to_delete: Vec<u32> = Vec::new();
        let mut current_index: u32;
        let mut notes_to_compare: Vec<Note> = Vec::new();
        for current_note in &self.notes {
            to_delete.clear();
            current_index = 0;
            for comparing_note in &notes_to_compare {
                if current_note.start_at > comparing_note.end_at {
                    to_delete.push(current_index);
                }
                current_index += 1;
            }
            for index in &to_delete {
                notes_to_compare.remove(index as usize);
            }
            notes_to_compare.push(current_note.clone());
            max_notes_at_once = max(max_notes_at_once, notes_to_compare.len() as u32);
        }
        Ok(max_notes_at_once)
    }
    pub fn list_frequencies_used_by_instruments(&mut self) -> Result<HashMap<u16, Vec<f64>>, error::NoNotesInListError> {
        if self.notes.is_empty() {
            return Err(error::NoNotesInListError {});
        }
        let mut frequencies_used_by_instruments: HashMap<u16, Vec<f64>> = HashMap::new();
        for current_note in &self.notes {
            let frequency_list = &mut *frequencies_used_by_instruments.entry(current_note.instrument_id).or_insert_with(Vec::new());
            if !(frequency_list.contains(&current_note.frequency)) {
                frequency_list.push(current_note.frequency);
            };
        }
        Ok(frequencies_used_by_instruments)
    }
}

#[derive(Clone)]
pub struct Note {
    pub start_at: u32,
    pub end_at: u32,
    pub duration: u32,
    pub frequency: f64,
    pub on_velocity: u32,
    pub off_velocity: u32,
    pub instrument_id: u16
}

#[derive(Clone)]
pub struct InstrumentList {
    pub instruments: HashMap<u16, Instrument>
}

#[derive(Clone)]
pub struct Instrument {
    pub loopable: bool,
    pub keys: HashMap<NotNaN<f64>, PCMAudio>,
    pub base_frequency: Option<f64>,
    //  key_gen_function: frequency_to_generate
    pub key_gen_function: Option<fn(&f64) -> Result<PCMAudio, Box<Error>>>
}

impl Instrument {
    pub fn gen_key(&mut self, frequency: &f64) -> Result<(), Box<Error>> {
        error::check_correct_frequency(frequency)?;
        if self.key_gen_function.is_some() {
            let new_key = self.key_gen_function.unwrap()(frequency)?;
            if new_key.sample_rate != get_project_sample_rate() {
                return Err(Box::new(error::WrongSampleRateError {}));
            }
            self.keys.insert(NotNaN::new(frequency)?, new_key);
            Ok(())
        } else {
            let base_frequency = &self.base_frequency.ok_or_else(|| Box::new(error::MissingBaseFrequencyError {}))?;
            let mut new_key = self.keys.get(&NotNaN::new(base_frequency)?).ok_or_else(|| Box::new(error::OriginalKeyNotFoundError {}))?.clone();
            new_key.change_pitch(&base_frequency, frequency)?;
            self.keys.insert(NotNaN::new(frequency)?, new_key);
            Ok(())
        }
    }
}
