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
            println!("error kinds: {} {}", ek_a, ek_b);

            let result = as_context(*ek_a, *ek_b);

            if let Err(e) = result {

                let fail: &Fail = &e;

                // Get the chain of causes (which includes the cause of this error itself).
                let mut causes = fail.iter_chain();

                // Inspect the first two.
                let found_ek_a = causes.next().and_then(|c| c.downcast_ref::<Error>()).map(|e| e.kind());
                let found_ek_b = causes.next().and_then(|c| c.downcast_ref::<Error>()).map(|e| e.kind());

                match (found_ek_a, found_ek_b) {
                    (Some(ErrorKind::A2), Some(ErrorKind::B1)) => println!("HANDLE THE ERROR"),
                    (_, _) => println!("FAIL WITH MESSAGE"),
                }

                // while let Some(cause) = fail.cause() {
                //     println!("cause: {}", cause);
                //     fail = cause;
                // }

                // for ch in fail.iter_chain() {
                //     let ch: &Fail = ch;

                //     println!("chain item: {:?}", ch);

                //     match ch.downcast_ref::<Error>() {
                //         Some(our_error) => println!("was our Error with ErrorKind {}", our_error.kind()),
                //         _ => println!("was something else"),
                //     }
                // }
            }

            println!("--------------------------------------------------------------------------------");
        }
    }
}
