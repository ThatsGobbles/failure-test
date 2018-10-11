use failure::Error;
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

pub fn example() {
    fn error_gen(i: u8) -> Result<(), Error> {
        match i {
            0 => Err(ErrorKindA::ErrorA1("A1".to_string()))?,
            1 => Err(ErrorKindA::ErrorA2("A2".to_string()))?,
            2 => Err(ErrorKindB::ErrorB1("B1".to_string()))?,
            3 => Err(ErrorKindB::ErrorB2("B2".to_string()))?,
            _ => Ok(()),
        }
    }

    for i in 0..=5 {
        println!("i value: {}", i);

        let result = error_gen(i);

        if let Err(e) = result {
            match e.downcast_ref::<ErrorKindA>() {
                Some(ErrorKindA::ErrorA1(ref s)) => println!("CRITICAL {}", s),
                _ => println!("WARNING"),
            }
        }
        else {
            println!("NORMAL");
        }
    }
}
