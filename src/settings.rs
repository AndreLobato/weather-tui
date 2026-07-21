use crate::widgets::WeatherWidget;

#[derive(Debug)]
pub enum WindSpeedUnits {
    Knots,
    MetersPerSecond,
    KilomoterPerHour
}

#[derive(Debug)]
pub enum WaveHeightUnits {
    Meters,
    Feets,
    BodyPart
}

#[derive(Debug)]
pub struct UnitsSettings {
    pub wind_speed: WindSpeedUnits,
    pub wave_height: WaveHeightUnits
}

impl Default for UnitsSettings {
    fn default() -> Self {
       Self {
            wind_speed: WindSpeedUnits::Knots,
            wave_height: WaveHeightUnits::Feets,
        } 
    }
}

#[derive(Debug)]
enum AvailableWidgets {
    Home
}

#[derive(Debug, Default)]
pub struct UserSettings {
    unit_settings: UnitsSettings,
    active_widgets: Vec<AvailableWidgets>,
}
