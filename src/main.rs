#![allow(dead_code, unused_variables)]

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::StreamConfig;

const SAMPLE_RATE: u32 = 48000;
const FREQUENCY: f32 = 440.0;
const BUFFER_SIZE: u32 = 512;
const TAU: f32 = 2.0 * std::f32::consts::PI;
const VOLUME: f32 = 0.7;

fn main() {
    let host: cpal::Host = cpal::default_host();
    let device: cpal::Device = host
        .default_output_device()
        .expect("no output device available");

    let config = StreamConfig {
        channels: 1,
        sample_rate: cpal::SampleRate(SAMPLE_RATE),
        buffer_size: cpal::BufferSize::Fixed(BUFFER_SIZE),
    };

    let mut phase: f32 = 0.0;
    let audio_fn = move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
        for sample in data.iter_mut() {
            *sample = VOLUME * phase.sin();
            phase += TAU * FREQUENCY / SAMPLE_RATE as f32;
            if phase > TAU {
                phase -= TAU;
            }
        }
    };

    let stream = device
        .build_output_stream(
            &config,
            audio_fn,
            |e| eprintln!("An error has occured on the audio thread: {e}"),
            None,
        )
        .expect("failed to create output stream");

    stream.play().unwrap();

    loop {}
}
