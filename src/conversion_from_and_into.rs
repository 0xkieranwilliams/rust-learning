use std::convert::From;
use std::convert::TryFrom;
// use std::convert::TryInto;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from (item: i32) -> Self {
        Number {value: item}
    }
}

// impl Into<Number> for i32 {
//     fn into(self) -> Number {
//         Number { value : self }
//     }
// }

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 { Ok(EvenNumber(value)) } else { Err(()) }
    }
}

pub fn run() {
    let my_str = "hello";
    let my_string = String::from(my_str);
    println!("{}", my_string);

    let num = Number::from(30);
    println!("My number is: {:?}", num);

    let int = 5;
    let num: Number = int.into();
    println!("My number is: {:?}", num);

    // TryFrom 

    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));

    // TryInto
    
    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));
    let result: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));

}



