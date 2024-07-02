#![allow(dead_code, unused_variables)]

use std::sync::mpsc;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::StreamConfig;

const SAMPLE_RATE: u32 = 48000;
const FREQUENCY: f32 = 440.0;
const BUFFER_SIZE: u32 = 512;
const PI: f32 = std::f32::consts::PI;

fn next_sine_sample(i: u32) -> f32 {
    0.0
}

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

    let phase_increment: f32 = FREQUENCY / SAMPLE_RATE as f32;

    let output_callback = move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
        for i in 0..BUFFER_SIZE {
            data[i as usize] = f32::sin(2.0 * PI * phase_increment * i as f32);
        }
    };
    let stream = device
        .build_output_stream(
            &config,
            output_callback,
            |e| eprintln!("An error has occured on the audio thread: {e}"),
            None,
        )
        .expect("failed to create output stream");

    stream.play().unwrap();

    let (exit_tx, exit_rx): (mpsc::Sender<()>, mpsc::Receiver<()>) = mpsc::channel();

    exit_rx.recv().unwrap();
}
