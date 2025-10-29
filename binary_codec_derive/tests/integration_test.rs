use binary_codec::{Decode, Encode, Measure};

#[derive(Decode, Encode, Measure)]
struct Person {
    id: u32,
    age: u8,
    favorite_color: Color,
}

#[derive(Decode, Encode, Measure)]
#[repr(u8)]
enum Color {
    Red = 1,
    Green = 2,
    Blue = 3,
    Grayscale(u8) = 4,
    RGB {
        red: u8,
        green: u8,
        blue: u8,
    } = 5,
    Gradient(Box<Color>, Box<Color>) = 6,
    Unknown() = 255,
}
