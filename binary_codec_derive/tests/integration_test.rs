use binary_codec::{Decode, Encode, Measure};

#[derive(Decode, Encode, Measure)]
struct Person {
    #[binary_codec(binary_codec::primitive::u32::be)]
    id: u32,
    age: u8,
    favorite_color: Color,
}

#[derive(Decode, Encode, Measure)]
#[repr(u16)]
#[binary_codec(binary_codec::primitive::u16::le)]
enum Color {
    Red = 1,
    Green = 2,
    Blue = 3,
    Grayscale(
        #[binary_codec(binary_codec::primitive::u16::be)]
        u16
    ) = 4,
    RGBa {
        red: u8,
        green: u8,
        blue: u8,
        #[binary_codec(binary_codec::primitive::u16::be)]
        alpha: u16,
    } = 5,
    Gradient(Box<Color>, Box<Color>) = 6,
    Unknown() = 255,
}
