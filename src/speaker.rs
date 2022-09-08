extern crate sdl2;

use sdl2::AudioSubsystem;
use sdl2::audio::{AudioCallback, AudioSpecDesired, AudioDevice};

#[derive(Clone, Copy)]
pub struct SpeakerSettings {
    pub frequency: u16,
    pub volume: u8,
}

impl Default for SpeakerSettings {
    fn default() -> Self {
        Self {
            frequency: 220,
            volume: 20,
        }
    }
}

struct Beep {
    settings: SpeakerSettings,
    sample_frequency: f32,
    phase: f32,
}

impl AudioCallback for Beep {
    type Channel = i8;

    fn callback(&mut self, out: &mut [i8]) {
        let amplitude = (self.settings.volume / 2) as i8;
        let phase_delta = self.settings.frequency as f32 / self.sample_frequency as f32;
        for i in 0..out.len() {
            out[i] = if self.phase <= 0.5 { amplitude } else { -amplitude };
            self.phase = (self.phase + phase_delta) % 1.0;
        }
    }
}

pub struct Speaker {
    audio_device: AudioDevice<Beep>,
}

impl Speaker {
    pub fn new(audio_subsystem: AudioSubsystem, settings: SpeakerSettings) -> Self {
        let desired_spec = AudioSpecDesired {
            freq: Some(44100),
            channels: Some(1),
            samples: None,
        };

        let audio_device = audio_subsystem.open_playback(None, &desired_spec, |spec| {
            Beep {
                settings: settings,
                sample_frequency: spec.freq as f32,
                phase: 0.0,
            }
        }).expect("Could not create audio device.");

        Self {
            audio_device: audio_device,
        }
    }

    pub fn volume(&mut self) -> u8 {
        let guard = self.audio_device.lock();
        guard.settings.volume
    }

    pub fn set_volume(&mut self, volume: u8) {
        let mut guard = self.audio_device.lock();
        guard.settings.volume = volume;
    }

    pub fn play(&mut self) {
        self.audio_device.resume();
    }

    pub fn pause(&mut self) {
        self.audio_device.pause();
    }
}