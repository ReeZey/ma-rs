pub struct Rover {
    username: String,
    password: String,
    x: u32,
    y: u32,
    points: u32,
    connected: bool,
}

impl Rover {
    fn new(username: String, x: u32, y: u32) -> Self {
        Self { x, y, ..Default::default() }
    }
}

impl Default for Rover {
    fn default() -> Self {
        Self { x: Default::default(), y: Default::default(), points: Default::default(), username: "".into(), password: "".into(), connected: false }
    }
}