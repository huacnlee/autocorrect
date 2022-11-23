use std::time::SystemTime;

pub trait SystemTimeDuration {
    /// Time elapsed duration in ms
    fn elapsed_millis(&self) -> f64;
}

impl SystemTimeDuration for SystemTime {
    fn elapsed_millis(&self) -> f64 {
        let micros = self.elapsed().unwrap_or_default().as_micros();
        if micros == 0 {
            return 0.0;
        }

        micros as f64 / 1_000.00
    }
}
