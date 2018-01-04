use std::collections::HashMap;
use ordered_float::NotNaN;
use super::Note;
use super::NoteList;

pub fn calculate_frequency(key: f64, base_key: i32) -> f64 {
    (2f64.powf((key - f64::from(base_key)) / 12f64)) * 440f64
}

#[derive(Clone)]
struct PartialNote {
    pub start_at: u32,
    pub on_velocity: u32,
    pub instrument: u16
}

#[derive(Clone, Default)]
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
    pub fn start_note(&mut self, start_at: u32, frequency: f64, on_velocity: u32, instrument: u16) {
        let ord_frequency = NotNaN::new(frequency).unwrap();
        self.current_keys.entry(&ord_frequency).or_insert(PartialNote {
            start_at,
            on_velocity,
            instrument
        });
    }
    pub fn stop_note(&mut self, frequency: f64, end_at: u32, off_velocity: u32) {
        let ord_frequency = NotNaN::new(frequency).unwrap();
        if self.current_keys.contains_key(&ord_frequency) {
            let new_note = Note {
                start_at: self.current_keys[&ord_frequency].start_at,
                end_at,
                duration: end_at - self.current_keys[&ord_frequency].start_at,
                frequency,
                on_velocity: self.current_keys[&ord_frequency].on_velocity,
                off_velocity,
                instrument_id: self.current_keys[&ord_frequency].instrument
            };
            self.note_list.add_note(new_note)
        }
    }
    pub fn get_note_list(&mut self) -> NoteList {
        self.note_list.clone()
    }
}