use std::fmt::Display;
use std::fmt::Result as FmtResult;
use std::fmt::Formatter;

use failure::Backtrace;
use failure::Context;
use failure::Fail;
use failure::ResultExt;

#[derive(Debug)]
pub struct Error {
    inner: Context<ErrorKind>,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Fail)]
pub enum ErrorKind {
    #[fail(display = "A1")]
    A1,
    #[fail(display = "A2")]
    A2,
    #[fail(display = "B1")]
    B1,
    #[fail(display = "B2")]
    B2,
}

impl Fail for Error {
    fn cause(&self) -> Option<&Fail> { self.inner.cause() }
    fn backtrace(&self) -> Option<&Backtrace> { self.inner.backtrace() }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult { Display::fmt(&self.inner, f) }
}

impl Error {
    pub fn kind(&self) -> ErrorKind { *self.inner.get_context() }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error { Error { inner: Context::new(kind) } }
}

impl From<Context<ErrorKind>> for Error {
    fn from(inner: Context<ErrorKind>) -> Error { Error { inner: inner } }
}

pub fn example() {
    fn as_context(ek_a: ErrorKind, ek_b: ErrorKind) -> Result<(), Error> {
        use std::fs::File;

        File::open("DOES_NOT_EXIST").context(ek_a).context(ek_b)?;

        Ok(())
    }

    let eks = &[ErrorKind::A1, ErrorKind::A2, ErrorKind::B1, ErrorKind::B2];

    for ek_a in eks {
        for ek_b in eks {
            let result = as_context(*ek_a, *ek_b);

            if let Err(e) = result {
                println!("error: {}", e);

                // // Can't downcast our custom `Error`! Use `.kind` instead.
                // match e.kind() {
                //     ErrorKind::A1 => println!("Try to handle the error"),
                //     _ => println!("Just fail"),
                // }

                let mut fail: &Fail = &e;

                // // Get the chain of causes (which includes the cause of this error itself).
                // let mut causes = fail.iter_chain();

                // // Inspect the first two.
                // let cause_a: &Fail = causes.next().unwrap();
                // println!("cause a: {:?}", cause_a);
                // let found_ek_a = cause_a.downcast_ref::<ErrorKind>();

                // let cause_b: &Fail = causes.next().unwrap();
                // println!("cause b: {:?}", cause_b);
                // let found_ek_b = cause_b.downcast_ref::<ErrorKind>();

                // match (found_ek_a, found_ek_b) {
                //     (Some(ErrorKind::A2), Some(ErrorKind::B1)) => println!("HANDLE THE ERROR"),
                //     (_, _) => println!("FAIL WITH MESSAGE"),
                // }

                // while let Some(cause) = fail.cause() {
                //     println!("cause: {}", cause);
                //     fail = cause;
                // }

                for ch in fail.iter_chain() {
                    println!("chain item: {}", ch);
                }
            }
        }
    }
}
