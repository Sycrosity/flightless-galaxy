use bevy::prelude::*;
use std::path::Path;

pub fn parse_ron<P, T>(path: P, message: &str) -> T
where
    P: AsRef<Path>,
    T: Default + for<'de> serde::Deserialize<'de>,
{
    match std::fs::read(Path::new("assets").join(&path)) {
        Ok(bytes) => match ron::de::from_bytes(&bytes) {
            Ok(settings) => settings,
            Err(error) => handle_parse_error(error, message),
        },
        Err(error) => handle_parse_error(error, message),
    }
}

pub fn handle_parse_error<T: Default, E: std::error::Error>(error: E, message: &str) -> T {
    warn!("{message}: {error}");
    T::default()
}
