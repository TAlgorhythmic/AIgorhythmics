use std::error::Error;

use cpal::{traits::{DeviceTrait, HostTrait, StreamTrait}, FromSample, SampleRate, StreamConfig};

use crate::neural_net::analyzer::audio::AudioProcessor;

pub fn listen(proc: AudioProcessor) -> Result<(), Box<dyn Error + Send + Sync>> {
    let host = cpal::default_host();
    
    let input = host.default_input_device().expect("Failed to find a default input device");
    let conf = StreamConfig { channels: 1, sample_rate: SampleRate { 0: 24000 }, buffer_size: cpal::BufferSize::Default };
    
    let stream = input.build_input_stream_raw(&conf, cpal::SampleFormat::F32, |data, info| {
        println!("Data: {}", data.bytes().iter().map(|n| n.to_string()).collect::<Vec<_>>().join(" "));

        todo!("Learn how this works and handle data.");
    }, |err| {
        
    }, None)?;

    stream.play()?;
    Ok(())
}
