use crate::error::Error;

/// DNS Class.
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u16)]
pub enum Class {
    /// Internet Class.  By far the most common.
    IN = 1,
    /// CSNET class (obsoleted)
    CSNET = 2,
    /// CHAOS class (https://en.wikipedia.org/wiki/Chaosnet)
    CHAOS = 3,
    /// Hesoid (https://en.wikipedia.org/wiki/Hesiod_(name_service))
    HS = 4,
    /// None.  Used for "name is not in use" requests.
    NONE = 254,
    /// Any.  Used for "name is in use" requests.
    ANY = 255,
}

impl Class {
    pub fn from_rr_class(rr_class: u16) -> Result<Class, Error> {
        Ok(match rr_class {
            1 => Class::IN,
            2 => Class::CSNET,
            3 => Class::CHAOS,
            4 => Class::HS,
            254 => Class::NONE,
            255 => Class::ANY,
            other => return Err(Error::UnknownClass(other)),
        })
    }
}
