use std::collections::HashMap;
use ordered_float::NotNaN;
use super::Note;
use super::NoteList;

pub fn calculate_frequency(key: f64, base_key: f64) -> f64 {
    (2f64.powf((key - base_key) / 12f64)) * 440f64
}

#[derive(Clone)]
struct PartialNote {
    pub start_at: u32,
    pub velocity: u32,
    pub instrument: u16
}

#[derive(Clone)]
pub struct KeyHelper {
    current_keys: HashMap<NotNaN<f64>, PartialNote>,
    note_list: NoteList
}

impl KeyHelper {
    pub fn new() -> KeyHelper {
        KeyHelper {
            current_keys: HashMap::new(),
            note_list: NoteList::new()
        }
    }
    pub fn start_note(&mut self, start_at: u32, frequency: f64, velocity: u32, instrument: u16) {
        let ord_frequency = NotNaN::new(frequency).unwrap();
        if !self.current_keys.contains_key(&ord_frequency) {
            let new_partial_note = PartialNote {
                start_at,
                velocity,
                instrument
            };
            self.current_keys.insert(ord_frequency, new_partial_note);
        }
    }
    pub fn stop_note(&mut self, frequency: f64, end_at: u32) {
        let ord_frequency = NotNaN::new(frequency).unwrap();
        if self.current_keys.contains_key(&ord_frequency) {
            let new_note = Note {
                start_at: self.current_keys.get(&ord_frequency).unwrap().start_at,
                end_at,
                duration: end_at - self.current_keys.get(&ord_frequency).unwrap().start_at,
                frequency,
                velocity: self.current_keys.get(&ord_frequency).unwrap().velocity,
                instrument_id: self.current_keys.get(&ord_frequency).unwrap().instrument
            };
            self.note_list.add_note(new_note)
        }
    }
    pub fn get_note_list(&mut self) -> NoteList {
        self.note_list.clone()
    }
}