use clap::Parser;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::StreamConfig;
use std::f32::consts;

const SAMPLE_RATE: u32 = 48000;
const VOLUME: f32 = 0.7;

const MIN_FREQUENCY: f32 = 20.0;
const MAX_FREQUENCY: f32 = 1000.0;

/// Tune your damn instruments
#[derive(Parser, Debug)]
#[command(about, long_about = None)]
struct Settings {
    /// Frequency of A4 in Hertz
    #[arg(short, long, default_value_t = 440.0f32)]
    pub frequency: f32,
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

    let args = Settings::parse();

    let frequency = args.frequency;
    println!("a4 = {} Hz", frequency);

    if frequency <= MIN_FREQUENCY || frequency > MAX_FREQUENCY {
        eprintln!("Nope, not doing this");
        return;
    }

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
