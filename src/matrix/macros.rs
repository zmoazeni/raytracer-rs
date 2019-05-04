#[macro_export]
macro_rules! matrix {
    (
        $(
            $($x:expr),+
        );* $(;)?
    ) => (
        Matrix::with_values(
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
