use cpal::traits::{DeviceTrait, HostTrait};
use cpal::StreamConfig;

fn main() {
    let host: cpal::Host = cpal::default_host();
    let device: cpal::Device = host
        .default_output_device()
        .expect("no output device available");

    let config = StreamConfig {
        channels: 1,
        sample_rate: cpal::SampleRate(48000),
        buffer_size: cpal::BufferSize::Default,
    };

    let output_callback = move |data: &[f32], _: &cpal::InputCallbackInfo| {};

    let output_stream = device.build_output_stream(
        &config,
        output_callback,
        |e| eprintln!("An error has occured on the audio thread: {e}"),
        None,
    );
}
