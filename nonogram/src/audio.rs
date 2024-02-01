use rscenes::prelude::*;

#[derive(Clone, Copy, Debug)]
pub enum SfxType {
    CLAPPING,
    ERROR,
    LOCK,
    SET,
    UNSET,
}

#[derive(Clone, Copy, Debug)]
pub struct Sfx {
    clapping: Option<Sound>,
    error: Option<Sound>,
    lock: Option<Sound>,
    set: Option<Sound>,
    unset: Option<Sound>,
}

impl Sfx {
    pub fn new(rl: impl Raudio + Copy) -> Self {
        Self {
            clapping: Self::load_sound(rl, include_bytes!("assets/clapping.wav")),
            error: Self::load_sound(rl, include_bytes!("assets/error.wav")),
            lock: Self::load_sound(rl, include_bytes!("assets/lock.wav")),
            set: Self::load_sound(rl, include_bytes!("assets/set.wav")),
            unset: Self::load_sound(rl, include_bytes!("assets/unset.wav")),
        }
    }

    pub fn play(&self, tpe: SfxType) {
        if let Some(sound) = match tpe {
            SfxType::CLAPPING => self.clapping,
            SfxType::ERROR => self.error,
            SfxType::LOCK => self.lock,
            SfxType::SET => self.set,
            SfxType::UNSET => self.unset,
        } {
            sound.play();
        }
    }

    fn load_sound(rl: impl Raudio + Copy, data: &'static [u8]) -> Option<Sound> {
        let wave = rl.load_wave_from_memory(WaveType::Wav, data).ok()?;
        Some(rl.load_sound_from_wave(wave))
    }
}
