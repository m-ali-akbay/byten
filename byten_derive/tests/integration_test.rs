use byten::{
    Decode, Encode, Measure,
    SelfCodec,
    prim::{U16LE, U16BE, U32BE, U64BE},
    var,
};

#[derive(Debug, Decode, PartialEq, Eq, Encode, Measure)]
struct Person {
    #[byten(U32BE)]
    id: u32,
    age: u8,
    #[byten(var::Vec::<var::USizeBE, SelfCodec::<Color>>::default())]
    favorite_colors: Vec<Color>,
}

#[derive(Clone, Debug, PartialEq, Eq, Decode, Encode, Measure)]
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

#[cfg(test)]
mod test {
    use byten::prelude::EncodeToVec;

    use super::*;
    
    #[test]
    fn test_person_codec() {
        let person = Person {
            id: 123456,
            age: 30,
            favorite_colors: vec![
                Color::Red,
                Color::Grayscale(128),
                Color::RGBa {
                    red: 255,
                    green: 0,
                    blue: 0,
                    alpha: 65535,
                },
                Color::Gradient(Box::new(Color::Green), Box::new(Color::Blue)),
                Color::ColorCode(0b110010101010),
                Color::Unknown(),
            ],
        };
        
        let encoded = person.encode_to_vec().expect("Encoding failed");
        assert_eq!(encoded, vec![
            0x00, 0x01, 0xe2, 0x40, // id: U32BE(123456)
            30,                     // age: 30

            0b00000110,             // favorite_colors length: var::USizeBE(6)

            0x01, 0x00,             // Color::Red discriminant: U16LE(1)

            0x04, 0x00,             // Color::Grayscale discriminant: U16LE(4)
            0x00, 0x80,             // Grayscale value: U16BE(128)

            0x05, 0x00,             // Color::RGBa discriminant: U16LE(5)
            0xff, 0x00, 0x00,       // RGB values
            0xff, 0xff,             // alpha: U16BE(65535)

            0x06, 0x00,             // Color::Gradient discriminant: U16LE(6)

            0x02, 0x00,             // Color::Green discriminant: U16LE(2)

            0x03, 0x00,             // Color::Blue discriminant: U16LE(3)

            0x07, 0x00,             // Color::ColorCode discriminant: U16LE(7);

            // Color::ColorCode value: U64BE(0b110010101010)
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // padding for U64BE
            0b1100, 0b10101010, // full bytes

            0xff, 0x00,          // Color::Unknown discriminant: U16LE(255)
        ]);
        let decoded = Person::decode(&encoded, &mut 0).expect("Decoding failed");
        assert_eq!(person, decoded);
    }
}
