use serde::{Deserialize, Serialize};

/// Controller state
#[derive(Serialize, Deserialize, Debug)]
pub struct Controller {
    pub lightbar_colour: Vec<u8>,
    pub lightbar_enabled: bool,
    pub battery_percentage: u8,
    pub playerleds: u8,
    pub microphone: bool,
    pub microphone_led: bool,
    pub speaker: Speaker,
    pub volume: u8,
    pub attenuation: Vec<u8>,
    pub trigger: Trigger,
}

/// Sensible defaults for a controller
///
/// Used when we find no saved profile
impl Default for Controller {
    fn default() -> Self {
        Self {
            lightbar_colour: vec![255, 255, 255, 255],
            lightbar_enabled: true,
            battery_percentage: 100,
            playerleds: 1,
            microphone: false,
            microphone_led: false,
            speaker: Speaker::default(),
            volume: 0,
            attenuation: vec![0, 0],
            trigger: Trigger::default(),
        }
    }
}

/// Speaker mode enum
///
/// Default Internal
#[derive(Debug, Serialize, Deserialize, Default)]
pub enum Speaker {
    #[default]
    Internal,
    Headphone,
    Monoheadphone,
    Both,
}

/// Trigger modes with varying fields
///
/// Default Off
#[derive(Debug, Serialize, Deserialize, Default)]
pub enum TriggerEffect {
    #[default]
    Off,
    Feedback {
        position: u8,
        strength: u8,
    },
    Weapon {
        start: u8,
        stop: u8,
        strength: u8,
    },
    Bow {
        start: u8,
        stop: u8,
        strength: u8,
        snapforce: u8,
    },
    Galloping {
        start: u8,
        stop: u8,
        first_foot: u8,
        second_foot: u8,
        frequency: u8,
    },
    Machine {
        start: u8,
        stop: u8,
        strength_a: u8,
        strength_b: u8,
        frequency: u8,
        period: u8,
    },
    Vibration {
        position: u8,
        amplitude: u8,
        frequency: u8,
    },
    FeedbackRaw {
        strength: [u8; 10],
    },
    VibrationRaw {
        amplitude: [u8; 10],
        frequency: u8,
    },
    Mode {
        params: Vec<String>,
    },
}

/// Trigger struct with mode string and params
#[derive(Debug, Serialize, Deserialize)]
pub struct Trigger {
    pub side: String,
    pub effect: TriggerEffect,
}

impl Default for Trigger {
    /// Trigger struct default, disables both
    fn default() -> Self {
        Trigger {
            side: "both".to_string(),
            effect: TriggerEffect::Off,
        }
    }
}

impl Trigger {
    /// Trigger struct to dualsensectl command string
    pub fn to_command(&self) -> String {
        match &self.effect {
            TriggerEffect::Off => format!("trigger {} off", self.side),
            TriggerEffect::Feedback { position, strength } => {
                format!("trigger {} feedback {} {}", self.side, position, strength)
            }
            TriggerEffect::Weapon {
                start,
                stop,
                strength,
            } => {
                format!(
                    "trigger {} weapon {} {} {}",
                    self.side, start, stop, strength
                )
            }
            TriggerEffect::Bow {
                start,
                stop,
                strength,
                snapforce,
            } => {
                format!(
                    "trigger {} bow {} {} {} {}",
                    self.side, start, stop, strength, snapforce
                )
            }
            TriggerEffect::Galloping {
                start,
                stop,
                first_foot,
                second_foot,
                frequency,
            } => {
                format!(
                    "trigger {} galloping {} {} {} {} {}",
                    self.side, start, stop, first_foot, second_foot, frequency
                )
            }
            TriggerEffect::Machine {
                start,
                stop,
                strength_a,
                strength_b,
                frequency,
                period,
            } => {
                format!(
                    "trigger {} machine {} {} {} {} {} {}",
                    self.side, start, stop, strength_a, strength_b, frequency, period
                )
            }
            TriggerEffect::Vibration {
                position,
                amplitude,
                frequency,
            } => {
                format!(
                    "trigger {} vibration {} {} {}",
                    self.side, position, amplitude, frequency
                )
            }
            TriggerEffect::FeedbackRaw { strength } => {
                let strengths = strength
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(" ");
                format!("trigger {} feedback-raw [{}]", self.side, strengths)
            }
            TriggerEffect::VibrationRaw {
                amplitude,
                frequency,
            } => {
                let amplitudes = amplitude
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(" ");
                format!(
                    "trigger {} vibration-raw [{}] {}",
                    self.side, amplitudes, frequency
                )
            }
            TriggerEffect::Mode { params } => {
                let params = params.join(" ");
                format!("trigger {} mode {}", self.side, params)
            }
        }
    }
}
