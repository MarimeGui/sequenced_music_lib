extern crate ordered_float;

use std::collections::HashMap;
use std::cmp::max;

pub mod utils;

pub trait ValidSampleTypes {}
impl ValidSampleTypes for i8 {}
impl ValidSampleTypes for u8 {}
impl ValidSampleTypes for i16 {}
impl ValidSampleTypes for u16 {}
impl ValidSampleTypes for i32 {}
impl ValidSampleTypes for u32 {}

#[derive(Clone)]
pub enum BitsPerSample {
    Signed8,
    Unsigned8,
    Signed16,
    Unsigned16,
    Signed32,
    Unsigned32
}

#[derive(Clone)]
pub struct MusicSequencer<N: ValidSampleTypes> {
    pub sample_type: BitsPerSample,
    pub sample_rate: u32,
    pub channels: u8,
    pub notes: NoteList,
    pub instruments: InstrumentList<N>
}

#[derive(Clone)]
pub struct NoteList{
    pub notes: Vec<Note>,
    pub max_notes_at_once: Option<u32>,
    pub frequencies_used_by_instruments: Option<HashMap<u16, Vec<f64>>>
}

impl NoteList {
    pub fn new() -> NoteList {
        NoteList {
            notes: Vec::new(),
            max_notes_at_once: None,
            frequencies_used_by_instruments: None
        }
    }
    pub fn add_note(&mut self, new_note: Note) {
        self.notes.push(new_note)
    }
    pub fn merge_other(&mut self, other: &mut NoteList) {
        self.notes.append(&mut other.notes)
    }
    pub fn sort_by_time(&mut self) {
        self.notes.sort_by(|a, b| a.start_at.cmp(&b.start_at));
    }
    pub fn calc_max_notes_at_once(&mut self) {
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
    }
    pub fn list_frequencies_used_by_instruments(&mut self) {
        let mut frequencies_used_by_instruments: HashMap<u16, Vec<f64>> = HashMap::new();
        for current_note in &self.notes {
            let frequency_list = &mut *frequencies_used_by_instruments.entry(current_note.instrument_id).or_insert(Vec::new());
            if !(frequency_list.contains(&current_note.frequency)) {
                frequency_list.push(current_note.frequency);
            };
        }
        self.frequencies_used_by_instruments = Some(frequencies_used_by_instruments);
    }
}

#[derive(Clone)]
pub struct Note {
    pub start_at: u32,
    pub end_at: u32,
    pub duration: u32,
    pub frequency: f64,
    pub velocity: u32,
    pub instrument_id: u16
}

#[derive(Clone)]
pub struct InstrumentList<N: ValidSampleTypes> {
    pub instruments: HashMap<u16, Instrument<N>>
}

#[derive(Clone)]
pub struct Instrument<N: ValidSampleTypes> {
    pub loopable: bool,
    pub pcm: Vec<N>
}
