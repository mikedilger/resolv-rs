
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
    pub name: String,
    pub class: Class,
    pub ttl: u32,
    pub data: T
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
/// value to `::libresolv_sys::__ns_type`, but we have extended it with record types not
/// present or supported by the underlying library
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(non_camel_case_types)]
#[repr(u16)]
pub enum RecordType {
    /// RFC 1035 - Host Address
    A = 1,
    /// RFC 1035 - Authoritative Name Server
    NS = 2,
    /// RFC 1035 - Mail Destination (Obsolete, use MX)
    MD = 3,
    /// RFC 1035 - Mail Forwarder (Obsolete, use MX)
    MF = 4,
    /// RFC 1035, 4035, 6604 - Canonical Name for an Alias
    CNAME = 5,
    /// RFC 1035, 1982, 2181, 2308 - Start of a Zone of Authority
    SOA = 6,
    /// RFC 1035 - Mailbox Domain Name (EXPERIMENTAL)
    MB = 7,
    /// RFC 1035 - Mail Group Member (EXPERIMENTAL)
    MG = 8,
    /// RFC 1035 - Mail Rename Domain Name (EXPERIMENTAL)
    MR = 9,
    /// RFC 1035 - A null resource record (EXPERIMENTAL)
    NULL = 10,
    /// RFC 1035 - A well known service description
    WKS = 11,
    /// RFC 1035 - A domain name pointer
    PTR = 12,
    /// RFC 1035 - Host information
    HINFO = 13,
    /// RFC 1035 - Mailbox or mail list information
    MINFO = 14,
    /// RFC 1035 - Mail exchange
    MX = 15,
    /// RFC 1035 - Text strings
    TXT = 16,
    /// RFC 1183 - Responsible Person
    RP = 17,
    /// RFC 1183, 6895 - AFS Database Location (Deprecated by RFC5864)
    AFSDB = 18,
    /// RFC 1183 - X.25 Addresses (EXPERIMENTAL)
    X25 = 19,
    /// RFC 1183 - ISDN Addresses (EXPERIMENTAL)
    ISDN = 20,
    /// RFC 1183 - Route Through (EXPERIMENTAL)
    RT = 21,
    /// RFC 1706 - Network Service Access Protocol
    NSAP = 22,
    /// RFC 1706 - Network Service Access Protocol PTR
    NSAP_PTR = 23,
    /// RFC 2535 (Obsolete), 2931 - Signautre
    SIG = 24,
    /// RFC 2535 (Obsolete) - Key
    KEY = 25,
    /// RFC 2163, 3597 - Pointer to X.400/RFC822 mapping information
    PX = 26,
    /// RFC 1712 - Geographical location
    GPOS = 27,
    /// RFC 3596 - IPv6 Address
    AAAA = 28,
    /// RFC 1876 - Location Information
    LOC = 29,
    /// RFC 2535 (Obsolete) - Non-existant Names and Types
    NXT = 30,
    /// https://tools.ietf.org/html/draft-ietf-nimrod-dns-00 - Endpoint Identifier
    EID = 31,
    /// https://tools.ietf.org/html/draft-ietf-nimrod-dns-00 - Nimrod Locator
    NIMLOC = 32,
    /// RFC 2782, 6335 - Service Location
    SRV = 33,
    ATMA = 34,
    NAPTR = 35,
    KX = 36,
    CERT = 37,
    A6 = 38,
    /// RFC 2672, 6604 - Delegation Name
    DNAME = 39,
    SINK = 40,
    /// RFC 6891, 6895 - Option
    OPT = 41,
    APL = 42,
    /// RFC 4033, 4034, 4035, 4509 - Delegation signer
    DS = 43,
    /// RFC 4255 -SSH Public Key Fingerprint
    SSHFP = 44,
    /// IPsec Key
    IPSECKEY = 45,
    /// RFC 4033, 4034, 4035, 5702 - DNSSEC signature
    RRSIG = 46,
    /// RFC 4033, 4034, 4035, 4470 - Next Secure record
    NSEC = 47,
    /// RFC 4033, 4034, 4035, 5702 - DNS Key
    DNSKEY = 48,
    /// RFC 4701 - DHCP identifier
    DHCID = 49,
    /// RFC 5155 - Next Secure record version 3
    NSEC3 = 50,
    /// RFC 5155 - NSEC3 parameters
    NSEC3PARAM = 51,
    /// RFC 5205 - Host Identity Protocol
    HIP = 55,
    /// Child DS - RFC 7344
    CDS = 59,
    /// Child DNSKEY - RFC 7344
    CDNSKEY = 60,
    /// RFC 2930 - Transtion Key record
    TKEY = 249,
    /// RFC 2845, 3645, 4635, 6895 - Transaction Signature
    TSIG = 250,
    /// RFC 1995 - Incremental Zone Transfer
    IXFR = 251,
    /// RFC 1035, 5936 - A request for a transfer of an entire zone
    AXFR = 252,
    /// RFC 1035 - A request for mailbox-related records (MB, MG or MR)
    MAILB = 253,
    /// RFC 1035 - A request for mail agent RRs (Obsolete - see MX)
    MAILA = 254,
    /// RFC 1035 - A request for all records
    ANY = 255,
    /// ALSO URI=256, see RFC 7553
    ZXFR = 256,
    /// RFC 6844 - Certification Authority Authorization
    CAA = 257,
    /// DNSSEC Trust Authorities
    TA = 32768,
    /// RFC 4431 - DNSSEC Lookaside Validation record
    DLV = 32769,
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
