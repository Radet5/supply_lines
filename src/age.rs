use bevy::prelude::*;

use crate::time_control::TimeController;

#[derive(Component, Debug)]
pub struct Age {
    simulated_birth_timestamp: f64,
}

impl Age {
    pub fn new(time_controller: &TimeController) -> Self {
        Self {
            simulated_birth_timestamp: time_controller.simulated_elapsed_secs(),
        }
    }

    pub fn age_seconds(&self, time_controller: &TimeController) -> f64 {
        (time_controller.simulated_elapsed_secs() - self.simulated_birth_timestamp).max(0.0)
    }

    pub fn age_minutes(&self, time_controller: &TimeController) -> f64 {
        self.age_seconds(time_controller) / 60.0
    }

    pub fn age_hours(&self, time_controller: &TimeController) -> f64 {
        self.age_minutes(time_controller) / 60.0
    }

    pub fn age_days(&self, time_controller: &TimeController) -> f64 {
        self.age_hours(time_controller) / 24.0
    }

    pub fn age_years(&self, time_controller: &TimeController) -> f64 {
        self.age_days(time_controller) / 365.0
    }

    /// Formats the age as a tuple of discrete units: (years, days, hours, minutes, seconds).
    /// Each component is computed as an integer.
    pub fn formatted_age(&self, time_controller: &TimeController) -> (u64, u8, u8, u8, u8) {
        let total_seconds = self.age_seconds(time_controller).round() as u64;

        const SECS_PER_MIN: u64 = 60;
        const SECS_PER_HOUR: u64 = 60 * SECS_PER_MIN;
        const SECS_PER_DAY: u64 = 24 * SECS_PER_HOUR;
        const SECS_PER_YEAR: u64 = 365 * SECS_PER_DAY; // using 365 days per year

        let years = total_seconds / SECS_PER_YEAR;
        let rem_years = total_seconds % SECS_PER_YEAR;

        let days = rem_years / SECS_PER_DAY;
        let rem_days = rem_years % SECS_PER_DAY;

        let hours = rem_days / SECS_PER_HOUR;
        let rem_hours = rem_days % SECS_PER_HOUR;

        let minutes = rem_hours / SECS_PER_MIN;
        let seconds = rem_hours % SECS_PER_MIN;

        (years, days as u8, hours as u8, minutes as u8, seconds as u8)
    }

    /// Formats the age as a readable string.
    pub fn formatted_age_string(&self, time_controller: &TimeController) -> String {
        let (years, days, hours, minutes, seconds) = self.formatted_age(time_controller);
        format!(
            "{} years, {} days, {} hours, {} minutes, {} seconds",
            years, days, hours, minutes, seconds
        )
    }
}
