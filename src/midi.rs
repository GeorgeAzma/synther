// use std::fs::File;
// use std::io::{Read, Seek, SeekFrom};

// enum MidiEventType {
//     NoteOff,
//     NoteOn,
//     Other,
// }

// struct MidiEvent {
//     event: MidiEventType,
//     key: u8,
//     velocity: u8,
//     delta_tick: u32,
// }

// struct MidiNote {
//     key: u8,
//     velocity: u8,
//     start_time: u32,
//     duration: u32,
// }

// struct MidiTrack {
//     name: Vec<u8>,
//     instrument: Vec<u8>,
//     events: Vec<MidiEvent>,
//     notes: Vec<MidiNote>,
//     max_note: u8,
//     min_note: u8,
// }

// #[derive(PartialEq)]
// enum EventName {
//     VoiceNoteOff = 0x80,
//     VoiceNoteOn = 0x90,
//     VoiceAftertouch = 0xA0,
//     VoiceControlChange = 0xB0,
//     VoiceProgramChange = 0xC0,
//     VoiceChannelPressure = 0xD0,
//     VoicePitchBend = 0xE0,
//     SystemExclusive = 0xF0,
// }

// impl From<u8> for EventName {
//     fn from(value: u8) -> Self {
//         match value {
//             0x80 => EventName::VoiceNoteOff,
//             0x90 => EventName::VoiceNoteOn,
//             0xA0 => EventName::VoiceAftertouch,
//             0xB0 => EventName::VoiceControlChange,
//             0xC0 => EventName::VoiceProgramChange,
//             0xD0 => EventName::VoiceChannelPressure,
//             0xE0 => EventName::VoicePitchBend,
//             0xF0 => EventName::SystemExclusive,
//             _ => panic!("Invalid u8 value for MyEnum"),
//         }
//     }
// }

// enum MetaEventName {
//     Sequence = 0x00,
//     Text = 0x01,
//     Copyright = 0x02,
//     TrackName = 0x03,
//     InstrumentName = 0x04,
//     Lyrics = 0x05,
//     Marker = 0x06,
//     CuePoint = 0x07,
//     ChannelPrefix = 0x20,
//     EndOfTrack = 0x2F,
//     SetTempo = 0x51,
//     SMPTEOffset = 0x54,
//     TimeSignature = 0x58,
//     KeySignature = 0x59,
//     SequencerSpecific = 0x7F,
// }

// impl From<u8> for MetaEventName {
//     fn from(value: u8) -> Self {
//         match value {
//             0x00 => MetaEventName::Sequence,
//             0x01 => MetaEventName::Text,
//             0x02 => MetaEventName::Copyright,
//             0x03 => MetaEventName::TrackName,
//             0x04 => MetaEventName::InstrumentName,
//             0x05 => MetaEventName::Lyrics,
//             0x06 => MetaEventName::Marker,
//             0x07 => MetaEventName::CuePoint,
//             0x20 => MetaEventName::ChannelPrefix,
//             0x2F => MetaEventName::EndOfTrack,
//             0x51 => MetaEventName::SetTempo,
//             0x54 => MetaEventName::SMPTEOffset,
//             0x58 => MetaEventName::TimeSignature,
//             0x59 => MetaEventName::KeySignature,
//             0x7F => MetaEventName::SequencerSpecific,
//             _ => panic!("Invalid u8 value for MyEnum"),
//         }
//     }
// }

// struct MidiFile {
//     tracks: Vec<MidiTrack>,
//     tempo: u32,
//     bpm: u32,
// }

// impl MidiFile {
//     fn new(file: &str) -> Self {
//         let mut midi_file = Self {
//             tracks: Vec::new(),
//             tempo: 0,
//             bpm: 0,
//         };
//         midi_file.parse_file(file);
//         midi_file
//     }

//     fn parse_file(&mut self, file: &str) -> bool {
//         let mut file = match File::open(&file) {
//             Ok(f) => f,
//             Err(_) => return false,
//         };

//         let mut file_get = || -> u8 {
//             let mut b = [0u8];
//             file.read_exact(&mut b).unwrap();
//             b[0]
//         };

//         let mut read_str = |len: u32| -> Vec<u8> {
//             let mut s = Vec::new();
//             for _ in 0u32..len {
//                 s.push(file_get());
//             }
//             s
//         };

//         let mut read_val = || {
//             let mut val: u32 = 0;
//             let byte: u8 = 0;

//             val = file_get() as u32;

//             if val & 0x80 > 0 {
//                 val &= 0x7F;
//                 while let byte = file_get() {
//                     val = (val << 7) | (byte & 0x7F) as u32;
//                     if byte & 0x80 == 0 {
//                         break;
//                     }
//                 }
//             }

//             val
//         };

//         let read_u32_and_swap = |reader: &mut dyn Read| -> u32 {
//             let n32: u32 = 0;
//             reader
//                 .read_exact(&mut n32.to_le_bytes())
//                 .expect("Failed to read u32");
//             u32::from_be_bytes(n32.to_le_bytes())
//         };

//         let read_u16_and_swap = |reader: &mut dyn Read| -> u16 {
//             let n16: u16 = 0;
//             reader
//                 .read_exact(&mut n16.to_le_bytes())
//                 .expect("Failed to read u16");
//             u16::from_be_bytes(n16.to_le_bytes())
//         };

//         let file_id: u32 = read_u32_and_swap(&mut file);
//         let header_len: u32 = read_u32_and_swap(&mut file);
//         let format: u16 = read_u16_and_swap(&mut file);
//         let track_chunks: u16 = read_u16_and_swap(&mut file);
//         let division: u16 = read_u16_and_swap(&mut file);

//         for chunk in 0usize..track_chunks as usize {
//             let track_id: u32 = read_u32_and_swap(&mut file);
//             let track_len: u32 = read_u32_and_swap(&mut file);
//             let mut end_of_track = false;

//             self.tracks.push(MidiTrack {
//                 name: Vec::new(),
//                 instrument: Vec::new(),
//                 events: Vec::new(),
//                 notes: Vec::new(),
//                 max_note: 64,
//                 min_note: 64,
//             });

//             let wall_time: u32 = 0;

//             let mut prev_status: u8 = 0;

//             while !end_of_track {
//                 let mut status_time_delta: u32 = 0;
//                 let mut status: u8 = 0;

//                 status_time_delta = read_val();

//                 let mut b = [0u8];
//                 match file.read_exact(&mut b) {
//                     Ok(_) => {
//                         status = b[0];
//                         if status < 0x80 {
//                             status = prev_status;
//                             file.seek(SeekFrom::Current(-1));
//                         }

//                         if EventName::from(status & 0xF0) == EventName::VoiceNoteOff {
//                             prev_status = status;
//                             let channel: u8 = status & 0x0F;
//                             let note_id: u8 = file_get();
//                             let note_vel: u8 = file_get();
//                             self.tracks[chunk].events.push(MidiEvent {
//                                 event: MidiEventType::NoteOff,
//                                 key: note_id,
//                                 velocity: note_vel,
//                                 delta_tick: status_time_delta,
//                             });
//                         } else if EventName::from(status & 0xF0) == EventName::VoiceNoteOn {
//                             prev_status = status;
//                             let channel: u8 = status & 0x0F;
//                             let note_id: u8 = file_get();
//                             let note_vel: u8 = file_get();
//                             if note_vel == 0 {
//                                 self.tracks[chunk].events.push(MidiEvent {
//                                     event: MidiEventType::NoteOff,
//                                     key: note_id,
//                                     velocity: note_vel,
//                                     delta_tick: status_time_delta,
//                                 });
//                             } else {
//                                 self.tracks[chunk].events.push(MidiEvent {
//                                     event: MidiEventType::NoteOn,
//                                     key: note_id,
//                                     velocity: note_vel,
//                                     delta_tick: status_time_delta,
//                                 });
//                             }
//                         } else if EventName::from(status & 0xF0) == EventName::VoiceAftertouch {
//                             prev_status = status;
//                             let channel: u8 = status & 0x0F;
//                             let note_id: u8 = file_get();
//                             let note_vel: u8 = file_get();
//                             self.tracks[chunk].events.push(MidiEvent {
//                                 event: MidiEventType::Other,
//                                 key: 0,
//                                 velocity: 0,
//                                 delta_tick: 0,
//                             });
//                         } else if EventName::from(status & 0xF0) == EventName::VoiceControlChange {
//                             prev_status = status;
//                             let channel: u8 = status & 0x0F;
//                             let control_id: u8 = file_get();
//                             let control_val: u8 = file_get();
//                             self.tracks[chunk].events.push(MidiEvent {
//                                 event: MidiEventType::Other,
//                                 key: 0,
//                                 velocity: 0,
//                                 delta_tick: 0,
//                             });
//                         } else if EventName::from(status & 0xF0) == EventName::VoiceProgramChange {
//                             prev_status = status;
//                             let channel: u8 = status & 0x0F;
//                             let program_id: u8 = file_get();
//                             self.tracks[chunk].events.push(MidiEvent {
//                                 event: MidiEventType::Other,
//                                 key: 0,
//                                 velocity: 0,
//                                 delta_tick: 0,
//                             });
//                         } else if EventName::from(status & 0xF0) == EventName::VoiceChannelPressure
//                         {
//                             prev_status = status;
//                             let channel: u8 = status & 0x0F;
//                             let channel_pressure: u8 = file_get();
//                             self.tracks[chunk].events.push(MidiEvent {
//                                 event: MidiEventType::Other,
//                                 key: 0,
//                                 velocity: 0,
//                                 delta_tick: 0,
//                             });
//                         } else if EventName::from(status & 0xF0) == EventName::VoicePitchBend {
//                             prev_status = status;
//                             let channel: u8 = status & 0x0F;
//                             let ls7b: u8 = file_get();
//                             let ms7b: u8 = file_get();
//                             self.tracks[chunk].events.push(MidiEvent {
//                                 event: MidiEventType::Other,
//                                 key: 0,
//                                 velocity: 0,
//                                 delta_tick: 0,
//                             });
//                         } else if EventName::from(status & 0xF0) == EventName::SystemExclusive {
//                             prev_status = 0;

//                             if status == 0xFF {
//                                 let type_: u8 = file_get();
//                                 let len: u8 = read_val() as u8;

//                                 match type_.into() {
//                                     MetaEventName::Sequence => {
//                                         file_get();
//                                         file_get();
//                                     }
//                                     MetaEventName::Text => {
//                                         read_str(len as u32);
//                                     }
//                                     MetaEventName::Copyright => {
//                                         read_str(len as u32);
//                                     }
//                                     MetaEventName::TrackName => {
//                                         self.tracks[chunk].name = read_str(len as u32);
//                                     }
//                                     MetaEventName::InstrumentName => {
//                                         self.tracks[chunk].instrument = read_str(len as u32);
//                                     }
//                                     MetaEventName::Lyrics => {
//                                         read_str(len as u32);
//                                     }
//                                     MetaEventName::Marker => {
//                                         read_str(len as u32);
//                                     }
//                                     MetaEventName::CuePoint => {
//                                         read_str(len as u32);
//                                     }
//                                     MetaEventName::ChannelPrefix => {
//                                         file_get();
//                                     }
//                                     MetaEventName::EndOfTrack => {
//                                         end_of_track = true;
//                                     }
//                                     MetaEventName::SetTempo => {
//                                         if self.tempo == 0 {
//                                             self.tempo |= (file_get() as u32) << 16;
//                                             self.tempo |= (file_get() as u32) << 8;
//                                             self.tempo |= (file_get() as u32) << 0;
//                                             self.bpm = 60000000 / self.tempo;
//                                         }
//                                     }
//                                     MetaEventName::SMPTEOffset => {
//                                         file_get();
//                                         file_get();
//                                         file_get();
//                                         file_get();
//                                         file_get();
//                                     }
//                                     MetaEventName::TimeSignature => {
//                                         file_get();
//                                         file_get();
//                                         file_get();
//                                     }
//                                     MetaEventName::KeySignature => {
//                                         file_get();
//                                         file_get();
//                                     }
//                                     MetaEventName::SequencerSpecific => {
//                                         read_str(len as u32);
//                                     }
//                                 }
//                             }
//                         }
//                     }
//                     Err(ref e) if e.kind() == std::io::ErrorKind::UnexpectedEof => {
//                         break;
//                     }
//                     Err(_) => {
//                         panic!("Failed to read from file");
//                     }
//                 }
//             }
//         }

//         for track in self.tracks.iter() {
//             let mut wall_time: u32 = 0;

//             let mut notes_being_processed: Vec<MidiNote> = Vec::new();

//             for event in &track.events {
//                 wall_time += event.delta_tick;

//                 match event.event {
//                     MidiEventType::NoteOn => {
//                         notes_being_processed.push(MidiNote {
//                             key: event.key,
//                             velocity: event.velocity,
//                             start_time: wall_time,
//                             duration: 0,
//                         });
//                     }
//                     MidiEventType::NoteOff => {
//                         if let Some(index) = notes_being_processed
//                             .iter()
//                             .position(|n| n.key == event.key)
//                         {
//                             let mut note = notes_being_processed.remove(index);
//                             note.duration = wall_time - note.start_time;
//                             track.notes.push(note);
//                             track.min_note = std::cmp::min(track.min_note, note.key);
//                             track.max_note = std::cmp::max(track.max_note, note.key);
//                         }
//                     }
//                     _ => (),
//                 }
//             }
//         }

//         true
//     }
// }

// fn convert_to_notes(midi: &MidiFile, ticks_per_quarter_note: u32) -> Vec<MidiNote> {
//     let mut notes = Vec::new();
//     let mut note_states = [0u8; 256];
//     let mut note_start_times = [0u32; 256];
//     let mut time: u32 = 0;

//     for track in midi.tracks.iter() {
//         for event in track.events {
//             time += event.delta_tick;
//             match event.event {
//                 MidiEventType::NoteOff => {
//                     let note: u8 = event.key;
//                     note_states[note as usize] = 0;
//                     notes.push(MidiNote {
//                         key: note,
//                         velocity: event.velocity,
//                         start_time: note_start_times[note as usize],
//                         duration: time - note_start_times[note as usize],
//                     });
//                 }
//                 MidiEventType::NoteOn => {
//                     let note: u8 = event.key;
//                     note_states[note as usize] = event.velocity;
//                     if event.velocity == 0 {
//                         notes.push(MidiNote {
//                             key: note,
//                             velocity: event.velocity,
//                             start_time: note_start_times[note as usize],
//                             duration: time - note_start_times[note as usize],
//                         });
//                     } else {
//                         note_start_times[note as usize] = time;
//                     }
//                 }
//                 _ => (),
//             }
//         }
//     }

//     for note in 0u8..256u8 {
//         if note_states[note as usize] > 0 {
//             let duration: u32 = time - note_start_times[note as usize];
//             notes.push(MidiNote {
//                 key: note,
//                 velocity: note_states[note as usize],
//                 start_time: note_start_times[note as usize],
//                 duration: duration,
//             });
//         }
//     }

//     return notes;
// }

// fn ntof(note: u8) -> f64 {
//     return 440.0 * 2.0f64.powf((note as f64 - 69.0) / 12.0);
// }

// fn main() {
//     let midi = MidiFile::new("D:/peace/Downloads/Interstellar2.mid");
//     let ticks_per_quarter_note: u32 = 960;
//     let notes: Vec<MidiNote> = convert_to_notes(&midi, ticks_per_quarter_note);

//     let mut index: usize = 0;
//     for note in notes.into_iter() {
//         if index >= 200 {
//             break;
//         }
//         let key = note.key;
//         let start = note.start_time as f64 / ticks_per_quarter_note as f64 * 60.0 / midi.bpm as f64;
//         let duration =
//             note.duration as f64 / ticks_per_quarter_note as f64 * 60.0 / midi.bpm as f64;
//         let l: usize = 200;
//         let nl = index % l == 0;
//         let el = index % l == (l - 1);
//         if nl {
//             print!("s+=");
//         }
//         print!("p({:.2}, {:.2}, {:.2})", ntof(key), start, duration);
//         if el {
//             println!(";");
//         } else {
//             print!("+");
//         }
//         index += 1;
//     }
// }
