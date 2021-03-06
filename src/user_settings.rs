use super::*;

#[derive(Debug, Clone)]
pub struct UserSettings {
    rain_color: (u8, u8, u8),
    head_color: (u8, u8, u8),
    group: Characters,
    shading: bool,
    speed: (u64, u64),
}

impl UserSettings {
    pub fn new(
        rain_color: (u8, u8, u8),
        head_color: (u8, u8, u8),
        group: Characters,
        shading: bool,
        speed: (u64, u64),
    ) -> Self {
        Self {
            rain_color,
            head_color,
            group,
            shading,
            speed,
        }
    }
}
