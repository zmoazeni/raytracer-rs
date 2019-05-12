#[macro_export]
macro_rules! matrix {
    (
        $(
            $($x:expr),+
        );* $(;)?
    ) => (
        crate::Matrix::with_values(
            vec![$(
                vec![$($x),*],
            )*]
        )
    )
}

#[macro_export]
macro_rules! tuple {
    ( $($x:expr),* ) => {
        matrix![
            $($x);*
        ]
    };
}

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
