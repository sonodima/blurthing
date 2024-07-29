#[derive(Debug, Clone)]
pub struct Parameters {
    pub components: (u32, u32),
    pub blur: i32,

    pub hue_rotate: i32,
    pub brightness: i32,
    pub contrast: i32,
}

impl Default for Parameters {
    fn default() -> Self {
        Self {
            components: (4, 3),
            blur: 0,

            hue_rotate: 0,
            brightness: 0,
            contrast: 0,
        }
    }
}
