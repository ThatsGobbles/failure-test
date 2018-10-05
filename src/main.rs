extern crate failure;

use failure::Fail;

#[derive(Clone, PartialEq, Eq, Debug, Fail)]
enum ErrorKindA {
    #[fail(display = "error a: {}", _0)]
    ErrorA1(String),
    #[fail(display = "error b: {}", _0)]
    ErrorA2(String),
}

#[derive(Clone, PartialEq, Eq, Debug, Fail)]
enum ErrorKindB {
    #[fail(display = "error a: {}", _0)]
    ErrorB1(String),
    #[fail(display = "error b: {}", _0)]
    ErrorB2(String),
}

mod mod_a {
    use failure::Error;

    use super::ErrorKindA;
    use super::ErrorKindB;

    pub fn oh_no(i: u8) -> Result<(), Error> {
        match i {
            0 => Err(ErrorKindA::ErrorA1("awful".to_string()))?,
            1 => Err(ErrorKindA::ErrorA2("awful".to_string()))?,
            2 => Err(ErrorKindB::ErrorB1("awful".to_string()))?,
            3 => Err(ErrorKindB::ErrorB2("awful".to_string()))?,
            _ => Ok(()),
        }
    }
}

use self::mod_a::oh_no;

fn main() {
    for i in 0..=5 {
        println!("i value: {}", i);

        let result = oh_no(i);

        if let Err(e) = result {
            match e.downcast_ref::<ErrorKindA>() {
                Some(kind) => {
                    match kind {
                        ErrorKindA::ErrorA1(ref s) => println!("AN AWFUL ERROR! {}", s),
                        _ => println!("error, but don't care"),
                    }
                },
                None => println!("error, but don't care"),
            }
        }
        else {
            println!("Everything is OK!");
        }
    }
}
