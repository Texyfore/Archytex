#[macro_export]
macro_rules! color {
    ($hex:literal) => {
        hex_literal::hex!($hex).map(|b| b as f32 / 255.0)
    };
}
