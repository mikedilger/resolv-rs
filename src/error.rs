
use std::fmt;
use std::convert::From;
use std::ffi::{NulError, FromBytesWithNulError};
use std::str::Utf8Error;
use crate::Section;

#[derive(Clone, PartialEq, Eq)]
// Taken in part from glibc-2.23/resolv/herror.c h_errlist
#[repr(i32)]
pub enum ResolutionError {
    /// Success
    Success = 0,
    /// Authoritative Answer "Host not found"
    HostNotFound = 1,
    /// Non-Authoritative "Host not found" or SERVERFAIL.
    TryAgain = 2,
    /// Non recoverable errors, FORMERR, REFUSED, NOTIMP.
    NoRecovery = 3,
    /// Valid name, no data record of requested type.
    NoData = 4,
}
impl fmt::Debug for ResolutionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ResolutionError::Success =>  write!(f, "Resolver Error 0 (no error)"),
            ResolutionError::HostNotFound =>  write!(f, "Unknown host"),
            ResolutionError::TryAgain =>  write!(f, "Host name lookup failure"),
            ResolutionError::NoRecovery =>  write!(f, "Unknown server error"),
            ResolutionError::NoData =>  write!(f, "No address associated with name"),
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum Error {
    /// Name Resolution failed
    Resolver(ResolutionError),
    /// String contains null bytes
    CString(NulError),
    /// Stirng contains null bytes
    CStr(FromBytesWithNulError),
    /// Name service response does not parse
    ParseError,
    /// Section/Index is out of bounds
    NoSuchSectionIndex(Section, usize),
    /// Uncompress Error
    UncompressError,
    /// Result from dn_expand was not null terminated
    Unterminated,
    /// Wrong Resource record type
    WrongRRType,
    /// String is not valid UTF-8
    Utf8(Utf8Error),
    /// Unknown class
    UnknownClass(u16),
}
impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        match *self {
            Error::Resolver(ref e) => write!(f, "{}: {:?}", self.description(), e),
            Error::CString(ref e) => write!(f, "Name supplied contains a null byte at \
                                                position {}", e.nul_position()),
            Error::CStr(ref e) => write!(f, "{}: {:?}", self.description(), e),
            Error::NoSuchSectionIndex(s,i) => write!(f, "No such section index \
                                                         (section={:?}, index={})",
                                                     s, i),
            Error::Utf8(ref e) => write!(f, "{}: {:?}", self.description(), e),
            Error::UnknownClass(u) => write!(f, "{}: {}", self.description(), u),
            _ => write!(f, "{}", self.description()),
        }
    }
}
impl Error {
    fn description(&self) -> &str
    {
        match *self {
            Error::Resolver(_) => "Name Resolution failed",
            Error::CString(_) => "Name supplied contains a null byte",
            Error::CStr(_) => "CStr failed",
            Error::ParseError => "Name service response does not parse",
            Error::NoSuchSectionIndex(_,_) => "No such section index",
            Error::UncompressError => "Error uncompressing domain name",
            Error::Unterminated => "Result from dn_expand was not null terminated",
            Error::WrongRRType => "Wrong Resource Record type",
            Error::Utf8(_) => "UTF-8 error",
            Error::UnknownClass(_) => "Unknown class",
        }
    }

}
impl ::std::error::Error for Error {
    fn cause(&self) -> Option<&dyn (::std::error::Error)>
    {
        match *self {
            Error::CString(ref e) => Some(e),
            Error::Utf8(ref e) => Some(e),
            _ => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Resolver(ref e) => write!(f, "{}: {:?}", self.description(), e),
            Error::CString(ref e) => write!(f, "Name supplied contains a null byte at \
                                                position {}", e.nul_position()),
            Error::CStr(ref e) => write!(f, "{}: {:?}", self.description(), e),
            Error::NoSuchSectionIndex(s,i) => write!(f, "No such section index \
                                                         (section={:?}, index={})",
                                                     s, i),
            Error::Utf8(ref e) => write!(f, "{}: {}", self.description(), e),
            Error::UnknownClass(u) => write!(f, "{}: {}", self.description(), u),
            _ =>  write!(f, "{}", self.description()),
        }
    }
}

impl From<ResolutionError> for Error {
    fn from(err: ResolutionError) -> Error {
        Error::Resolver( err )
    }
}
impl From<Utf8Error> for Error {
    fn from(err: Utf8Error) -> Error {
        Error::Utf8( err )
    }
}
impl From<FromBytesWithNulError> for Error {
    fn from(err: FromBytesWithNulError) -> Error {
        Error::CStr( err )
    }
}
