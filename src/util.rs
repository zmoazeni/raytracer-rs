const THRESHOLD: f32 = 0.00001;

pub fn feq(f1: f32, f2: f32) -> bool {
    (f1 - f2).abs() < THRESHOLD
}
