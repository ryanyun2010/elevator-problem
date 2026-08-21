#[derive(Debug)]
pub struct Worker {
    pub arrival_time: isize, // seconds since 8:00
    pub target_floor: i32,
    pub clock_in_time: Option<isize>, // seconds since 8:00
}