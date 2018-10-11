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

#[derive(Copy, Clone, Eq, PartialEq, Debug, Fail)]
pub enum ErrorKindX {
    #[fail(display = "XA1")]
    XA1,
    #[fail(display = "XA2")]
    XA2,
    #[fail(display = "XB1")]
    XB1,
    #[fail(display = "XB2")]
    XB2,
}

pub fn example() {
    fn as_context(ek: ErrorKind) -> Result<(), Error> {
        use std::fs::File;

        File::open("DOES_NOT_EXIST").context(ek)?;

        Ok(())
    }

    fn solo(ek: ErrorKind) -> Result<(), Error> {
        Err(ek)?
    }

    let eks = &[ErrorKind::A1, ErrorKind::A2, ErrorKind::B1, ErrorKind::B2];

    for ek in eks {
        println!("{:?}", solo(*ek));
        println!("{:?}", as_context(*ek));
    }
}
