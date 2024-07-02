use cpal::traits::{DeviceTrait, HostTrait};

fn main() {
    let host: cpal::Host = cpal::default_host();
    let device: cpal::Device = host
        .default_output_device()
        .expect("no output device available");

    let config = device
        .default_output_config()
        .expect("no stream config available somehow")
        .config();

    let output_callback = move |data: &[f32], _: &cpal::InputCallbackInfo| {};
    let output_stream = device.build_output_stream(
        &config,
        output_callback,
        |e| eprintln!("An error has occured on the audio thread: {e}"),
        None,
    );
}
