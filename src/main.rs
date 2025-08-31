use clap::Parser;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::StreamConfig;
use std::f32::consts;

const SAMPLE_RATE: u32 = 48000;
const VOLUME: f32 = 0.7;

const MIN_FREQUENCY: f32 = 20.0;
const MAX_FREQUENCY: f32 = 1000.0;

const MIN_OFFSET: i8 = -12;
const MAX_OFFSET: i8 = 12;

/// Tune your damn instruments
#[derive(Parser, Debug)]
#[command(about, long_about = None)]
struct Settings {
    /// Frequency of A4 in Hertz, must be within [20, 1000] Hz
    #[arg(short, long, default_value_t = 440.0f32)]
    pub reference: f32,

    /// Offset in semitones, must be within [-12, 12] semitones
    #[arg(allow_hyphen_values = true, short, long, default_value_t = 0)]
    pub offset: i8,
}

fn main() {
    let host: cpal::Host = cpal::default_host();
    let device: cpal::Device = host
        .default_output_device()
        .expect("no output device available");

    let config = StreamConfig {
        channels: 1,
        sample_rate: cpal::SampleRate(SAMPLE_RATE),
        buffer_size: cpal::BufferSize::Default,
    };

    let settings = Settings::parse();
    log_settings(&settings);

    if let Some(err_msg) = verify_settings(&settings) {
        eprintln!("\n{}", err_msg);
        eprintln!("For more information, try '--help'");
        return;
    }

    let reference = settings.reference;
    let offset = settings.offset as f32;

    let frequency = reference * 2.0f32.powf(offset / 12.0);

    let mut phase: f32 = 0.0;
    let audio_fn = move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
        for sample in data.iter_mut() {
            *sample = VOLUME * phase.sin();
            phase += consts::TAU * frequency / SAMPLE_RATE as f32;
            phase = phase.rem_euclid(consts::TAU);
        }
    };

    let stream: cpal::Stream = device
        .build_output_stream(
            &config,
            audio_fn,
            |e| eprintln!("An error has occured on the audio thread: {e}"),
            None,
        )
        .expect("failed to create output stream");

    stream.play().unwrap();

    loop {
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if input == "\n" {
            return;
        }
    }
}

fn log_settings(settings: &Settings) {
    println!("a4 = {} Hz", settings.reference);
    if settings.offset != 0 {
        println!("offset = {} semitones", settings.offset);
    }
}

fn verify_settings(settings: &Settings) -> Option<&str> {
    if !(MIN_FREQUENCY..=MAX_FREQUENCY).contains(&settings.reference) {
        return Some("Frequency out of bounds");
    }

    if !(MIN_OFFSET..=MAX_OFFSET).contains(&settings.offset) {
        return Some("Offset out of bounds");
    }

    None
}
