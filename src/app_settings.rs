use std::fs;
use std::io::{Read, Write};
use ratatui::style::Color;
use serde::{Deserialize, Serialize};

const SETTINGS_FILE: &str = "paranoia_settings.toml";

// Used to display stuff at the top of the settings file.
#[derive(Debug, Serialize, Deserialize)]
pub struct ParanoiaGeneral {
    author: String,
    version: String,
}

impl Default for ParanoiaGeneral {
    fn default() -> Self {
        Self {
            author: String::from("aravix"),
            version: String::from("0.0.1"),
        }
    }
}

// The struct that the settings file gets serialized into
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SettingsToml {
    paranoia: ParanoiaGeneral,
    settings: AppSettings,
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum AppTheme {
    Default = 0,
    Vaporwave = 1,
}

impl AppTheme {
    pub fn fg_color(theme: &AppTheme) -> Color {
        match theme {
            AppTheme::Default => Color::LightYellow,
            AppTheme::Vaporwave => Color::LightMagenta,
        }
    }

    pub fn highlight_color(theme: &AppTheme) -> Color {
        match theme {
            AppTheme::Default => Color::LightRed,
            AppTheme::Vaporwave => Color::LightCyan,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct AppSettings {
    pub theme: AppTheme,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            theme: AppTheme::Default
        }
    }
}

impl AppSettings {
    pub fn initialize() -> Self {
        let settings_file = fs::File::open(SETTINGS_FILE);
        let mut settings = String::new();

        if let Ok(mut file) = settings_file {
            file.read_to_string(&mut settings).expect("");
            Self::set_existing_settings(settings)
        } else {
            // Settings file couldn't be found, create it
            Self::create_default_settings()
        }
    }

    fn set_existing_settings(settings_file: String) -> Self {
        let settings: SettingsToml = toml::from_str(&settings_file).expect("settings file failed to read");

        settings.settings
    }

    fn create_default_settings() -> Self {
        let settings = SettingsToml::default();
        let toml_settings = toml::to_string(&settings).unwrap();

        let settings_file = fs::File::create(SETTINGS_FILE);

        if let Ok(mut file) = settings_file {
            file.write_all(toml_settings.as_bytes()).expect("Yep");
        }
        settings.settings
    }
    
    pub fn save_settings(&self) {
        let settings = SettingsToml {
            paranoia: ParanoiaGeneral::default(),
            settings: *self
        };
        let toml_settings = toml::to_string(&settings).unwrap();
        
        let settings_file = fs::File::create(SETTINGS_FILE);
        if let Ok(mut file) = settings_file {
            file.write_all(toml_settings.as_bytes()).expect("Yep");
        }
    }

    pub fn change_theme(&mut self, theme: AppTheme) {
        self.theme = theme;
        
        self.save_settings();
    }
}