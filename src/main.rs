use rodio::buffer::SamplesBuffer;
use rodio::{OutputStream, Sink};
mod sound;

const SAMPLE_RATE: u32 = 44100;
const DURATION: f32 = 15.0;
const OFFSET: f32 = 0.0;

fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    println!("Generating Audio...");
    let mut samples = vec![0f32; (SAMPLE_RATE as f32 * DURATION) as usize];
    for i in 0..samples.len() {
        let t = i as f32 / SAMPLE_RATE as f32 + OFFSET;
        samples[i] = sound::sound(t);
    }
    let audio = SamplesBuffer::new(1, SAMPLE_RATE, samples);
    println!("Generated.");
    sink.append(audio);
    sink.sleep_until_end();
}
