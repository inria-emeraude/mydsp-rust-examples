use std::{error::Error, i16, sync::Arc};

use cxx_juce::{
    JUCE,
    juce_audio_devices::{AudioDeviceManager, AudioIODevice, AudioIODeviceCallback},
};

use cxx_juce::juce_audio_devices::{InputAudioSampleBuffer, OutputAudioSampleBuffer};

use mydsp_rust::AudioComponent;
use mydsp_rust::sine::SineWave;
use mydsp_rust::sine_table::SineTable;

use std::{sync::LazyLock, thread::sleep, time::Duration};

struct SineTone {
    freq: f32,
    my_sine: SineWave,
}

impl AudioIODeviceCallback for SineTone {
    fn about_to_start(&mut self, _device: &mut dyn AudioIODevice) {}
    fn process_block(
        &mut self,
        _: &InputAudioSampleBuffer<'_>,
        o: &mut OutputAudioSampleBuffer<'_>,
    ) {
        for n in 0..o.samples() {
            let sample = self.my_sine.tick(0.0) * 0.5 as f32;
            for c in 0..o.channels() {
                o[c][n] = sample;
            }
        }
    }

    fn stopped(&mut self) {}
}

fn main() -> Result<(), Box<dyn Error>> {
    let juce = JUCE::initialise();
    let mut adm = AudioDeviceManager::new(&juce);
    adm.initialise(0, 2)?;
    static TABLE: LazyLock<SineTable> = std::sync::LazyLock::new(|| SineTable::new(16384));
    let freq = 440.;
    // TODO: get 48000 from the device
    let mut sine = SineWave::new(&TABLE, 48000.);
    sine.set_freq(freq);

    let _hdl = adm.add_audio_callback(SineTone {
        freq: freq,
        my_sine: sine,
    });
     sleep(Duration::from_secs(2));
    Ok(())
}

#[test]
fn test() {}
