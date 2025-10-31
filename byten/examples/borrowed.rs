use byten::{Decode, Encode, Measure, prelude::EncodeToVec as _, prim, util::Convert, var};

type U16BEAsUSize = Convert<prim::U16BE, usize>;

#[derive(Debug, Encode, Decode, Measure)]
pub struct Person<'encoded> {
    #[byten(var::str::Str::<U16BEAsUSize>::default())]
    pub first_name: &'encoded str,
    #[byten(var::str::Str::<U16BEAsUSize>::default())]
    pub last_name: &'encoded str,
}

fn main() {
    let person = Person {
        first_name: "Alice",
        last_name: "Smith",
    };

    let encoded = person.encode_to_vec().unwrap();
    println!("Encoded Person: {:?}", encoded);

    let mut offset = 0;
    let decoded_person = Person::decode(&encoded, &mut offset).unwrap();
    println!("Decoded Person: {:?}", decoded_person);
}
