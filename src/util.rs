const THRESHOLD: f32 = 0.00001;

#[macro_export]
macro_rules! assert_feq {
    ($left:expr, $right:expr) => ({
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(util::feq(*left_val, *right_val)) {
                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    panic!(r#"assertion failed: `feq(left, right)`
  left: `{:?}`,
 right: `{:?}`"#, &*left_val, &*right_val)
                }
            }
        }
    });
}

pub fn feq(f1: f32, f2: f32) -> bool {
    (f1 - f2).abs() < THRESHOLD
}
