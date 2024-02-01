use rscenes::prelude::*;

static mut SFX: Option<SfxManager> = None;

#[derive(Clone, Copy, Debug)]
pub enum SfxType {
    CLAPPING,
    ERROR,
    LOCK,
    SET,
    UNSET,
}

#[derive(Clone, Copy, Debug)]
pub struct SfxManager {
    clapping: &'static [u8],
    error: &'static [u8],
    lock: &'static [u8],
    set: &'static [u8],
    unset: &'static [u8],
}

impl SfxManager {
    pub fn load_assets() {
        unsafe {
            SFX = Some(SfxManager::default());
        }
    }

    pub fn get_instance() -> Option<Sfx> {
        unsafe {
            if let Some(sfx) = SFX {
                let wave = Wave::load_from_memory(WaveType::Wav, sfx.clapping).ok()?;
                let clapping = Sound::load_from_wave(wave);
                wave.unload();
                let wave = Wave::load_from_memory(WaveType::Wav, sfx.error).ok()?;
                let error = Sound::load_from_wave(wave);
                wave.unload();
                let wave = Wave::load_from_memory(WaveType::Wav, sfx.lock).ok()?;
                let lock = Sound::load_from_wave(wave);
                wave.unload();
                let wave = Wave::load_from_memory(WaveType::Wav, sfx.set).ok()?;
                let set = Sound::load_from_wave(wave);
                wave.unload();
                let wave = Wave::load_from_memory(WaveType::Wav, sfx.unset).ok()?;
                let unset = Sound::load_from_wave(wave);
                wave.unload();
                return Some(Sfx {
                    clapping,
                    error,
                    lock,
                    set,
                    unset,
                });
            }
        }

        None
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
        let sound = Sound::load_from_wave(wave);
        wave.unload();
        sound.play();
        Ok(())
    }
}

impl Default for SfxManager {
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

#[derive(Debug)]
pub struct Sfx {
    clapping: Sound,
    error: Sound,
    lock: Sound,
    set: Sound,
    unset: Sound,
}

impl Sfx {
    pub fn play(&self, tpe: SfxType) {
        match tpe {
            SfxType::CLAPPING => self.clapping.play(),
            SfxType::ERROR => self.error.play(),
            SfxType::LOCK => self.lock.play(),
            SfxType::SET => self.set.play(),
            SfxType::UNSET => self.unset.play(),
        }
    }
}

impl Drop for Sfx {
    fn drop(&mut self) {
        self.clapping.unload();
        self.error.unload();
        self.lock.unload();
        self.set.unload();
        self.unset.unload();
    }
}
