use anyhow::Result;
use esp_alloc as _;
use rtc_hal::error::Error;
use rtc_hal::square_wave::{SquareWave, SquareWaveFreq};
use rtc_hal::{datetime::DateTime, rtc::Rtc};

pub struct DemoApp<RTC> {
    rtc: RTC,
}

impl<RTC> DemoApp<RTC>
where
    RTC: Rtc,
{
    pub fn new(rtc: RTC) -> Self {
        Self { rtc }
    }

    pub fn set_datetime(&mut self, dt: &DateTime) -> Result<()> {
        self.rtc
            .set_datetime(dt)
            .map_err(|e| anyhow::anyhow!("RTC error: {:?}", e))?;
        Ok(())
    }

    pub fn print_current_time(&mut self) -> core::result::Result<(), rtc_hal::error::ErrorKind> {
        let current_time = self.rtc.get_datetime().map_err(|e| e.kind())?;

        defmt::info!(
            "📅 {}-{:02}-{:02} 🕐 {:02}:{:02}:{:02}",
            current_time.year(),
            current_time.month(),
            current_time.day_of_month(),
            current_time.hour(),
            current_time.minute(),
            current_time.second()
        );
        Ok(())
    }
}

impl<RTC> DemoApp<RTC>
where
    RTC: SquareWave,
{
    pub fn start_square_wave(&mut self) -> Result<()> {
        self.rtc
            .start_square_wave(SquareWaveFreq::Hz1)
            .map_err(|e| anyhow::anyhow!("RTC error: {:?}", e))?;

        Ok(())
    }

    pub fn stop_square_wave(&mut self) -> Result<()> {
        self.rtc
            .disable_square_wave()
            .map_err(|e| anyhow::anyhow!("RTC error: {:?}", e))?;

        Ok(())
    }
}
