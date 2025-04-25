use std::time::Duration;

use bevy::prelude::*;

#[derive(Resource, Debug, Reflect)]
#[reflect(Resource)]
pub struct TimeController {
    speed_factor: f32,
    scaled_delta: Duration,
    simulated_elapsed_secs: f64,
}

impl TimeController {
    pub fn speed_factor(&self) -> f32 {
        self.speed_factor
    }

    pub fn scaled_delta(&self) -> Duration {
        self.scaled_delta
    }

    pub fn simulated_elapsed_secs(&self) -> f64 {
        self.simulated_elapsed_secs
    }

    pub fn seconds_to_formatted(total_seconds: u64) -> (u64, u16, u8, u8, u8) {
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

        (
            years,
            days as u16,
            hours as u8,
            minutes as u8,
            seconds as u8,
        )
    }

    pub fn simulated_elapsed_time_string(&self) -> String {
        let (years, days, hours, minutes, seconds) =
            Self::seconds_to_formatted(self.simulated_elapsed_secs.round() as u64);
        Self::formatted_string(years, days, hours, minutes, seconds)
    }

    pub fn formatted_string(years: u64, days: u16, hours: u8, minutes: u8, seconds: u8) -> String {
        format!(
            "{} years, {:02} days, {:02} hours, {:02} minutes, {:02} seconds",
            years, days, hours, minutes, seconds
        )
    }
}

pub struct TimeControlPlugin;

impl Plugin for TimeControlPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TimeController {
            speed_factor: 1.,
            scaled_delta: Duration::default(),
            simulated_elapsed_secs: 0.,
        })
        .add_systems(Update, control_time_speed)
        .add_systems(FixedUpdate, scale_time)
        .register_type::<TimeController>();
    }
}

fn control_time_speed(
    mut time_controller: ResMut<TimeController>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.pressed(KeyCode::Digit1) {
        time_controller.speed_factor = 1.;
        println!("Speed 1s/s");
    } else if keyboard_input.pressed(KeyCode::Digit2) {
        time_controller.speed_factor = 60.;
        println!("Speed 1min/s");
    } else if keyboard_input.pressed(KeyCode::Digit3) {
        time_controller.speed_factor = 60.0 * 60.0;
        println!("Speed 1hr/s");
    } else if keyboard_input.pressed(KeyCode::Digit4) {
        time_controller.speed_factor = 60.0 * 60.0 * 24.0;
        println!("Speed 1day/s");
    } else if keyboard_input.pressed(KeyCode::Backquote) {
        time_controller.speed_factor = 0.0;
    }
}

fn scale_time(time: Res<Time>, mut time_controller: ResMut<TimeController>) {
    let scaled_delta_secs = time.delta_secs_f64() * time_controller.speed_factor as f64;

    time_controller.simulated_elapsed_secs += scaled_delta_secs;

    time_controller.scaled_delta = Duration::from_secs_f64(scaled_delta_secs);
}

#[cfg(test)]
mod tests {
    use super::TimeController;

    #[test]
    fn zero_seconds() {
        assert_eq!(TimeController::seconds_to_formatted(0), (0, 0, 0, 0, 0));
    }

    #[test]
    fn only_seconds() {
        // less than a minute
        assert_eq!(TimeController::seconds_to_formatted(45), (0, 0, 0, 0, 45));
    }

    #[test]
    fn exactly_one_minute() {
        assert_eq!(TimeController::seconds_to_formatted(60), (0, 0, 0, 1, 0));
    }

    #[test]
    fn minutes_and_seconds() {
        // 2 minutes, 30 seconds
        assert_eq!(
            TimeController::seconds_to_formatted(2 * 60 + 30),
            (0, 0, 0, 2, 30)
        );
    }

    #[test]
    fn exactly_one_hour() {
        assert_eq!(
            TimeController::seconds_to_formatted(60 * 60),
            (0, 0, 1, 0, 0)
        );
    }

    #[test]
    fn hours_minutes_seconds() {
        // 1h 15m 20s
        let secs = 1 * 3600 + 15 * 60 + 20;
        assert_eq!(
            TimeController::seconds_to_formatted(secs),
            (0, 0, 1, 15, 20)
        );
    }

    #[test]
    fn exactly_one_day() {
        assert_eq!(
            TimeController::seconds_to_formatted(24 * 3600),
            (0, 1, 0, 0, 0)
        );
    }

    #[test]
    fn days_hours_minutes_seconds() {
        // 3d 4h 5m 6s
        let secs = 3 * 86400 + 4 * 3600 + 5 * 60 + 6;
        assert_eq!(TimeController::seconds_to_formatted(secs), (0, 3, 4, 5, 6));
    }

    #[test]
    fn exactly_one_year() {
        // 1 year = 365 days
        assert_eq!(
            TimeController::seconds_to_formatted(365 * 86400),
            (1, 0, 0, 0, 0)
        );
    }

    #[test]
    fn full_combination() {
        // 2y 10d 6h 30m 15s
        let secs = 2 * 365 * 86400 + 10 * 86400 + 6 * 3600 + 30 * 60 + 15;
        assert_eq!(
            TimeController::seconds_to_formatted(secs),
            (2, 10, 6, 30, 15)
        );
    }

    #[test]
    fn days_overflow() {
        // 300 days worth of seconds:
        let secs = 300 * 24 * 3600;
        let (years, days, hours, minutes, seconds) = TimeController::seconds_to_formatted(secs);

        // We expect 0 years, 300 days, 0h 0m 0s:
        assert_eq!(years, 0);
        assert_eq!(days, 300);
        assert_eq!(hours, 0);
        assert_eq!(minutes, 0);
        assert_eq!(seconds, 0);
    }

    #[test]
    fn large_number_of_seconds() {
        // arbitrary large number
        let secs = 1234567890;
        let (y, d, h, m, s) = TimeController::seconds_to_formatted(secs);
        // sanity-check by recomposing
        let recomposed =
            y as u64 * 365 * 86400 + d as u64 * 86400 + h as u64 * 3600 + m as u64 * 60 + s as u64;
        assert_eq!(recomposed, secs);
    }
}
