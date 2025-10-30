use byten::{
    Decode, Encode, Measure,
    SelfCodec,
    prim::{U16LE, U16BE, U32BE, U64BE},
    var,
};

#[derive(Decode, Encode, Measure)]
struct Person {
    #[byten(U32BE)]
    id: u32,
    age: u8,
    #[byten(var::Vec::<var::USizeBE, SelfCodec::<Color>>::default())]
    favorite_colors: Vec<Color>,
}

#[derive(Clone, Decode, Encode, Measure)]
#[repr(u16)]
#[byten(U16LE)]
enum Color {
    Red = 1,
    Green = 2,
    Blue = 3,
    Grayscale(
        #[byten(U16BE)]
        u16
    ) = 4,
    RGBa {
        red: u8,
        green: u8,
        blue: u8,
        #[byten(U16BE)]
        alpha: u16,
    } = 5,
    Gradient(Box::<Color>, Box::<Color>) = 6,
    ColorCode(
        #[byten(U64BE)]
        u64
    ) = 7,
    Unknown() = 255,
}
