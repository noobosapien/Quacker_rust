use serde::Deserialize;

#[derive(Deserialize, Clone, Default)]
pub struct Point {
    pub x: i16,
    pub y: i16,
}
