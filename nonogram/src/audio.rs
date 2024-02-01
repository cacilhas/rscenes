use rscenes::prelude::*;

static mut SFX: Option<Sfx> = None;

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
    clapping: &'static [u8],
    error: &'static [u8],
    lock: &'static [u8],
    set: &'static [u8],
    unset: &'static [u8],
}

impl Sfx {
    pub fn load_assets() {
        unsafe {
            SFX = Some(Sfx::default());
        }
    }

    pub fn get_instance() -> Option<Self> {
        unsafe { SFX.clone() }
    }

    pub fn play(&self, tpe: SfxType) -> Result<(), String> {
        let data = match tpe {
            SfxType::CLAPPING => self.clapping,
            SfxType::ERROR => self.error,
            SfxType::LOCK => self.lock,
            SfxType::SET => self.set,
            SfxType::UNSET => self.unset,
        };
        let wave = Wave::load_from_memory(WaveType::Wav, data)?;
        Sound::load_from_wave(wave).play();
        Ok(())
    }
}

impl Default for Sfx {
    fn default() -> Self {
        Self {
            clapping: include_bytes!("assets/clapping.wav"),
            error: include_bytes!("assets/error.wav"),
            lock: include_bytes!("assets/lock.wav"),
            set: include_bytes!("assets/set.wav"),
            unset: include_bytes!("assets/unset.wav"),
        }
    }
}
