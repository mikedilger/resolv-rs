
use std::ffi::CStr;
use ::error::Error;
use ::libresolv_sys::__ns_rr as Rr;
use ::libresolv_sys::__ns_msg as Message;

mod class;
pub use self::class::Class;

/// For internal use.
pub trait RecordData: Sized {
    /// Get type of record
    fn get_record_type() -> RecordType;

    /// Convert from low level resource record.  For internal use.
    fn extract(msg: &mut Message, rr: &Rr) -> Result<Self, Error>;
}

/// A DNS response record of a particular type
#[derive(Debug, Clone)]
pub struct Record<T: RecordData> {
    name: String,
    class: Class,
    ttl: u32,
    data: T
}

impl<T: RecordData> Record<T> {
    /// For internal use.
    pub fn extract(msg: &mut Message, rr: &Rr) -> Result<Record<T>, Error> {
        Ok(Record {
            name: unsafe { CStr::from_ptr(rr.name.as_ptr()).to_string_lossy().into_owned() },
            class: try!(Class::from_rr_class(rr.rr_class)),
            ttl: rr.ttl,
            data: try!(<T as RecordData>::extract(msg, rr)),
        })
    }
}

/// This is a simple u16 value indicating the type of a resource record, and is equal in
/// value to `::libresolv_sys::__ns_type`
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(non_camel_case_types)]
#[repr(u16)]
pub enum RecordType {
    A = 1,
    NS = 2,
    MD = 3,
    MF = 4,
    CNAME = 5,
    SOA = 6,
    MB = 7,
    MG = 8,
    MR = 9,
    NULL = 10,
    WKS = 11,
    PTR = 12,
    HINFO = 13,
    MINFO = 14,
    MX = 15,
    TXT = 16,
    RP = 17,
    AFSDB = 18,
    X25 = 19,
    ISDN = 20,
    RT = 21,
    NSAP = 22,
    NSAP_PTR = 23,
    SIG = 24,
    KEY = 25,
    PX = 26,
    GPOS = 27,
    AAAA = 28,
    LOC = 29,
    NXT = 30,
    EID = 31,
    NIMLOC = 32,
    SRV = 33,
    ATMA = 34,
    NAPTR = 35,
    KX = 36,
    CERT = 37,
    A6 = 38,
    DNAME = 39,
    SINK = 40,
    OPT = 41,
    APL = 42,
    TKEY = 249,
    TSIG = 250,
    IXFR = 251,
    AXFR = 252,
    MAILB = 253,
    MAILA = 254,
    ANY = 255,
    ZXFR = 256,
}

// FIXME: Add the other record types
pub use self::a::A;
pub use self::ns::NS;
pub use self::cname::CNAME;
pub use self::soa::SOA;
pub use self::ptr::PTR;
pub use self::mx::MX;
pub use self::txt::TXT;
pub use self::aaaa::AAAA;

// FIXME: Add the other record types
mod a;
mod ns;
mod cname;
mod soa;
mod ptr;
mod mx;
mod txt;
mod aaaa;
