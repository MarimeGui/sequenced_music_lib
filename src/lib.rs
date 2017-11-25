extern crate ordered_float;

use std::collections::HashMap;
use std::cmp::max;
use ordered_float::NotNaN;

pub mod utils;
mod error;

static mut SAMPLE_RATE: u32 = 48000;

pub fn get_sample_rate() -> u32 {
    unsafe {
        SAMPLE_RATE.clone()
    }
}

pub fn set_sample_rate(new_sr: u32) {
    unsafe {
        SAMPLE_RATE = new_sr;
    }
}

#[derive(Clone)]
pub struct PCMAudio {
    pub samples: Vec<Vec<i32>>,
    pub sample_rate: u32,
    pub channels: u8
}

impl PCMAudio {
    pub fn change_pitch(&mut self, original_freq: f64, target_freq: f64) {
        unimplemented!();
    }
}

#[derive(Clone)]
pub struct MusicSequencer {
    pub pcm: PCMAudio,
    pub note_list: NoteList,
    pub instrument_list: InstrumentList
}

impl MusicSequencer {
    pub fn gen_instrument_keys(&mut self) {
        self.note_list.list_frequencies_used_by_instruments();
    }
}

#[derive(Clone)]
pub struct NoteList{
    changed_since_last_calc_max_notes_at_once: bool,
    changed_since_last_list_frequencies_used_by_instruments: bool,
    pub notes: Vec<Note>,
    pub max_notes_at_once: Option<u32>,
    pub frequencies_used_by_instruments: Option<HashMap<u16, Vec<f64>>>
}

impl NoteList {
    pub fn new() -> NoteList {
        NoteList {
            changed_since_last_calc_max_notes_at_once: false,
            changed_since_last_list_frequencies_used_by_instruments: false,
            notes: Vec::new(),
            max_notes_at_once: None,
            frequencies_used_by_instruments: None
        }
    }
    pub fn add_note(&mut self, new_note: Note) {
        self.changed_since_last_calc_max_notes_at_once = true;
        self.changed_since_last_list_frequencies_used_by_instruments = true;
        self.notes.push(new_note);
    }
    pub fn merge_other(&mut self, other: &mut NoteList) {
        self.changed_since_last_calc_max_notes_at_once = true;
        self.changed_since_last_list_frequencies_used_by_instruments = true;
        self.notes.append(&mut other.notes);
    }
    pub fn sort_by_time(&mut self) {
        self.notes.sort_by(|a, b| a.start_at.cmp(&b.start_at));
    }
    pub fn calc_max_notes_at_once(&mut self) {
        if self.changed_since_last_calc_max_notes_at_once {
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
                        to_delete.push(current_index.clone());
                    }
                    current_index += 1;
                }
                for index in &to_delete {
                    notes_to_compare.remove(index.clone() as usize);
                }
                notes_to_compare.push(current_note.clone());
                max_notes_at_once = max(max_notes_at_once, notes_to_compare.len() as u32);
            }
            self.max_notes_at_once = Some(max_notes_at_once);
            self.changed_since_last_calc_max_notes_at_once = false;
        }
    }
    pub fn list_frequencies_used_by_instruments(&mut self) {
        if self.changed_since_last_list_frequencies_used_by_instruments {
            let mut frequencies_used_by_instruments: HashMap<u16, Vec<f64>> = HashMap::new();
            for current_note in &self.notes {
                let frequency_list = &mut *frequencies_used_by_instruments.entry(current_note.instrument_id).or_insert(Vec::new());
                if !(frequency_list.contains(&current_note.frequency)) {
                    frequency_list.push(current_note.frequency);
                };
            }
            self.changed_since_last_list_frequencies_used_by_instruments = false;
            self.frequencies_used_by_instruments = Some(frequencies_used_by_instruments);
        }
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
    pub key_gen_function: Option<fn(f64) -> PCMAudio>
}

impl Instrument {
    pub fn gen_key(&mut self, frequency: f64) {
        if self.key_gen_function.is_some() {
            let new_key = self.key_gen_function.unwrap()(frequency);
            self.keys.insert(NotNaN::new(frequency).unwrap(), new_key);
        } else {
            let base_frequency = self.base_frequency.unwrap();
            let orig_key = self.keys.get(&NotNaN::new(base_frequency).unwrap()).unwrap().clone();
            let mut new_key = orig_key.clone();
            new_key.change_pitch(base_frequency, frequency);
            self.keys.insert(NotNaN::new(frequency).unwrap(), new_key);
        }
    }
}
