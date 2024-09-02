use std::{thread, time::Instant};
use tinyaudio::prelude::*;
use crossbeam::channel;

use crate::utils;

const SAMPLE_COUNT: usize = 44100;
pub struct Synth {
    sin: Vec<Vec<f32>>
}

impl Synth {
    pub fn new() -> Self {
        let mut sin: Vec<Vec<f32>> = vec![vec![0.0; SAMPLE_COUNT]; 88];
        
        for i in 0..88 {
            for j in 0..SAMPLE_COUNT {
                sin[i][j] = ((j as f32) * utils::midi_note_to_frequency(i as u8) * 2.0 * std::f32::consts::PI / SAMPLE_COUNT as f32).sin()
            }
        }
        Self {
            sin: sin
        }
    }

    pub fn run(&mut self, key_rx: channel::Receiver<Vec<u8>>) {
        let params = OutputDeviceParameters {
            channels_count: 1,
            sample_rate: 44100,
            channel_sample_count: 2000,
        };
        let sin_table = self.sin.clone();
        thread::spawn(move || {
            let _device = run_output_device(params, {
                let mut clock = 0f32;
                let mut note_played = Vec::with_capacity(88);
                move |data| {
                    for samples in data.chunks_mut(params.channels_count) {
                        match key_rx.try_recv() {
                            Ok(input_note_played) => {
                                println!("Audio {:?}", Instant::now());
                                note_played = input_note_played;
                            }
                            _ => {}
                        };
                        clock = (clock + 1.0) % params.sample_rate as f32;
                        let mut value = 0.;
                        for note in &mut note_played { 
                            let note_index = *note;
                            // println!("{} {}", note_index as usize, clock as usize);
                            value += 0.4 * sin_table[note_index as usize][clock as usize];
                        }
                        for sample in samples {
                            *sample = value;
                        }
                    }
                }
            })
            .unwrap();
            loop {
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
        });
    }

}
