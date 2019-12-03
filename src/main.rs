extern crate cpal;

use cpal::traits::{DeviceTrait, EventLoopTrait, HostTrait};

mod noise;

fn main() {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("failed to find a default output device");

    let format = device.default_output_format().expect("no output format");
    let event_loop = host.event_loop();

    let stream_id = event_loop
        .build_output_stream(&device, &format)
        .expect("no output stream");

    event_loop
        .play_stream(stream_id.clone())
        .expect("can't play stream");

    let mut merzbox = noise::Noise::new();

    event_loop.run(move |id, result| {
        let data = match result {
            Ok(data) => data,
            Err(err) => {
                eprintln!("an error occurred on stream {:?}: {}", id, err);
                return;
            }
        };

        match data {
            cpal::StreamData::Output {
                buffer: cpal::UnknownTypeOutputBuffer::U16(mut buffer),
            } => {
                for sample in buffer.chunks_mut(format.channels as usize) {
                    let value = ((merzbox.shshsh() * 0.5 + 0.5) * std::u16::MAX as f32) as u16;
                    for out in sample.iter_mut() {
                        *out = value;
                    }
                }
            }
            cpal::StreamData::Output {
                buffer: cpal::UnknownTypeOutputBuffer::I16(mut buffer),
            } => {
                for sample in buffer.chunks_mut(format.channels as usize) {
                    let value = (merzbox.shshsh() * std::i16::MAX as f32) as i16;
                    for out in sample.iter_mut() {
                        *out = value;
                    }
                }
            }
            cpal::StreamData::Output {
                buffer: cpal::UnknownTypeOutputBuffer::F32(mut buffer),
            } => {
                for sample in buffer.chunks_mut(format.channels as usize) {
                    let value = merzbox.shshsh();
                    for out in sample.iter_mut() {
                        *out = value.clone();
                    }
                }
            }
            _ => (),
        }
    });
}
