use std::thread;

use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    FromSample, Sample, SizedSample, StreamConfig,
};
use crossbeam::channel;

use crate::utils;

pub struct Synth {}

impl Synth {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(&mut self, key_rx: channel::Receiver<Vec<f32>>) {
        println!("toto!");
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .expect("failed to find audio output device");
        let config = device.default_output_config().unwrap();
        let sample_format = config.sample_format();
        let config: StreamConfig = config.into();
        thread::spawn(move || match sample_format {
            cpal::SampleFormat::F32 => Synth::play_audio_stream::<f32>(key_rx, &device, &config),
            cpal::SampleFormat::F64 => Synth::play_audio_stream::<f64>(key_rx, &device, &config),
            cpal::SampleFormat::I8 => Synth::play_audio_stream::<i8>(key_rx, &device, &config),
            cpal::SampleFormat::I16 => Synth::play_audio_stream::<i16>(key_rx, &device, &config),
            cpal::SampleFormat::I32 => Synth::play_audio_stream::<i32>(key_rx, &device, &config),
            cpal::SampleFormat::I64 => Synth::play_audio_stream::<i32>(key_rx, &device, &config),
            cpal::SampleFormat::U8 => Synth::play_audio_stream::<u8>(key_rx, &device, &config),
            cpal::SampleFormat::U16 => Synth::play_audio_stream::<u16>(key_rx, &device, &config),
            cpal::SampleFormat::U32 => Synth::play_audio_stream::<u32>(key_rx, &device, &config),
            cpal::SampleFormat::U64 => Synth::play_audio_stream::<u32>(key_rx, &device, &config),
            _ => todo!(),
        });
    }

    pub fn play_audio_stream<T>(
        key_rx: channel::Receiver<Vec<f32>>,
        device: &cpal::Device,
        config: &cpal::StreamConfig,
    ) where
        T: cpal::Sample,
        T: SizedSample + FromSample<f32>,
    {
        let err_fn = |err| eprintln!("an error occurred on stream: {}", err);
        let sample_rate = config.sample_rate.0 as f32;
        let channels = config.channels as usize;
        let mut sample_clock = 0f32;
        let mut note_played = Vec::new();
        let mut next_value = move || {
            sample_clock = (sample_clock + 1.0) % sample_rate;
            match key_rx.try_recv() {
                Ok(input_note_played) => {
                    note_played = input_note_played;
                }
                _ => {}
            };
            let mut value = 0.;
            for note in &mut note_played {
                let freq = *note;
                value += (sample_clock * freq * 2.0 * std::f32::consts::PI / sample_rate).sin();
            }
            println!("{}", sample_clock);
            value
        };
        let stream = device
            .build_output_stream(
                config,
                move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
                    Synth::write_data(data, channels, &mut next_value)
                },
                err_fn,
                None,
            )
            .unwrap();
        stream.play().unwrap();
        loop {
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
    }

    fn write_data<T>(output: &mut [T], channels: usize, next_sample: &mut dyn FnMut() -> f32)
    where
        T: Sample + FromSample<f32>,
    {
        for frame in output.chunks_mut(channels) {
            let value: T = T::from_sample(next_sample());
            for sample in frame.iter_mut() {
                *sample = value;
            }
        }
    }
}
