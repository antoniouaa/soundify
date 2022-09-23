use std::collections::{HashMap, HashSet};
use std::f32::consts::PI;
use std::fs;
use std::io::Error;

// Usage:
// & "C:\Program Files\ffmpeg\bin\ffplay.exe" -showmode 1 -f f32le -ar 48000 .\output.bin

type Hz = f32;
type Seconds = f32;
type Pulse = f32;
type Wave = Vec<Pulse>;

const MIDDLE_A: Hz = 432.0;
const SAMPLE_RATE: f32 = 48000.0;
const VOLUME: f32 = 1.0;
const DURATION_SCALE: f32 = 88.0;

fn scale_pitch(diff: f32) -> f32 {
    MIDDLE_A * (2.0_f32.powf(1.0 / 12.0)).powf(diff / 2.0)
}

fn dedup(source: &mut Vec<char>) {
    let mut uniques = HashSet::new();
    source.retain(|c| uniques.insert(*c));
}

fn clean_source(source_code: &str) -> Vec<char> {
    source_code
        .chars()
        .map(|c| c.to_ascii_lowercase())
        .filter(|c| c.is_alphanumeric() || c.is_ascii_punctuation())
        .collect()
}

fn generate_wave(frequency: Hz, duration: Seconds) -> Wave {
    let step = (frequency * 2.0 * PI) / SAMPLE_RATE;

    (0..(SAMPLE_RATE * duration) as u32)
        .map(|sample| sample as f32 * step)
        .map(|sample| sample.sin())
        .map(|sample| sample * VOLUME)
        .collect()
}

fn construct_frequencies(source_code: &str) -> HashMap<char, Hz> {
    let mut characters: Vec<char> = clean_source(source_code);
    dedup(&mut characters);
    characters.sort();

    let letter_map = HashMap::from_iter(
        characters
            .into_iter()
            .enumerate()
            .map(|(i, e)| (e, scale_pitch(i as Hz)))
            .collect::<Vec<(char, Hz)>>(),
    );

    letter_map
}

fn calculate_averages(source_code: &str, mapping: HashMap<char, f32>) -> Vec<(Hz, Seconds)> {
    source_code
        .split("\n")
        .map(|l| clean_source(l))
        .filter(|l| !l.is_empty())
        .map(|l| {
            (
                l.iter().map(|c| mapping[&c]).sum::<f32>() / l.len() as Hz,
                l.len() as Seconds / DURATION_SCALE,
            )
        })
        .collect()
}

pub fn convert(source_code: &str) -> Vec<Hz> {
    let letter_map = construct_frequencies(source_code);
    let line_averages = calculate_averages(source_code, letter_map);

    let waves: Vec<Hz> = line_averages
        .clone()
        .into_iter()
        .map(|(c, l)| generate_wave(c, l))
        .flatten()
        .collect();

    waves
}

pub fn write_to_file(converted: Vec<Hz>) -> Result<(), Error> {
    let out: Vec<u8> = converted
        .iter()
        .map(|val| val.to_le_bytes())
        .flatten()
        .collect();
    fs::write("output.bin", out)?;
    Ok(())
}
