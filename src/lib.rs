//! This library consists of a high-level interface to Gnu libc's (glibc) `libresolv` DNS
//! resolver.  It allows you to look up DNS resource records of any type (e.g. A, AAAA, MX, TXT,
//! etc), use recursion (if your system's DNS resolver permits it), and perform the DNS search
//! algorithm to complete incomplete names, all via your operating system (glibc, not the kernel).
//!
//! The lower level library, libresolv-sys, was generated from glibc version 2.23 on linux,
//! using the newer thread-safe function calls.  It may not work on older version of glibc, or
//! on other operating systems.  Pull-requests which improve portability are appreciated.
//!
//! # Example
//!
//! ````
//! extern crate resolv;
//!
//! use resolv::{Resolver, Class, RecordType, Section, Record};
//! use resolv::record::MX;
//!
//! fn main() {
//!     // You must create a mutable resolver object to hold the context.
//!     let mut resolver = Resolver::new().unwrap();
//!
//!     // .query() and .search() are the main interfaces to the resolver.
//!     let mut response = resolver.query(b"gmail.com", Class::IN,
//!                                       RecordType::MX).unwrap();
//!
//!     // .get_section_count() returns the number of records in that
//!     // section of the response.  There are four sections, but we
//!     // usually care about `Section::Answer`
//!     for i in 0..response.get_section_count(Section::Answer) {
//!
//!         // As records are strongly typed, you must know what you are
//!         // looking for.  If you assign into the wrong type, a run-time
//!         // error `WrongRRType` will be returned.
//!         let mx: Record<MX> = response.get_record(Section::Answer, i)
//!                              .unwrap();
//!
//!        println!("{:?}", mx);
//!     }
//! }
//! ````

extern crate libresolv_sys;
extern crate byteorder;

pub mod error;
use error::{Error, ResolutionError};

mod response;
pub use response::{Response, Section, Flags, RecordItems};

pub mod record;
pub use record::{Record, RecordType, Class};

#[cfg(test)]
mod tests;

use std::ffi::CString;
use std::ops::{Deref, DerefMut};

type Context = libresolv_sys::__res_state;

/// Options for the Resolver
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ResolverOption {
    /// address initialized
    Init = libresolv_sys::RES_INIT,
    /// print debug messages
    Debug = libresolv_sys::RES_DEBUG,
    /// use virtual circuit
    UseVC = libresolv_sys::RES_USEVC,
    /// ignore truncation errors
    IgnTc = libresolv_sys::RES_IGNTC,
    /// recursion desired
    Recurse = libresolv_sys::RES_RECURSE,
    /// use default domain name
    DefNames = libresolv_sys::RES_DEFNAMES,
    /// Keep TCP socket open
    StayOpen = libresolv_sys::RES_STAYOPEN,
    /// search up local domain tree
    DNSrch = libresolv_sys::RES_DNSRCH,
    /// shuts off HOSTALIASES feature
    NoAliases = libresolv_sys::RES_NOALIASES,
    /// rotate ns list after each query
    Rotate = libresolv_sys::RES_ROTATE,
    /// Use EDNS0.
    UseEDNS0 = libresolv_sys::RES_USE_EDNS0,
    /// one outstanding request at a time
    SngLkup = libresolv_sys::RES_SNGLKUP,
    /// one outstanding request at a time, but open new socket for each request
    SngLkupReop = libresolv_sys::RES_SNGLKUPREOP,
    /// use DNSSEC using OK bit in OPT
    UseDNSSEC = libresolv_sys::RES_USE_DNSSEC,
    /// Do not look up unqualified name as a TLD.
    NoTLDQuery = libresolv_sys::RES_NOTLDQUERY,
    /// No automatic configuration reload (since glibc 2.26; invalid in prior versions)
    NoReload = libresolv_sys::RES_NORELOAD,
    /// Request AD bit, keep it in responses (since glibc 2.31; invalid in prior version)
    TrustAD = libresolv_sys::RES_TRUSTAD,
    /// Default values
    Default = libresolv_sys::RES_DEFAULT,
}

pub struct Resolver {
    context: Context
}

impl Resolver {
    pub fn new() -> Option<Resolver>
    {
        let mut resolver = Resolver {
            context: libresolv_sys::__res_state::default(),
        };

        if unsafe {
            libresolv_sys::__res_ninit(&mut resolver.context)
        } != 0 {
            return None
        }

        resolver.option(ResolverOption::Default, true);

        Some(resolver)
    }

    /// Set or unset an option
    pub fn option(&mut self, option: ResolverOption, value: bool) {
        if value {
            self.context.options = self.context.options | (option as u64);
        } else {
            self.context.options = self.context.options & !(option as u64);
        }
    }

    /// Lookup the record.  Applies the search algorithm to the domain name given
    /// (if not fully qualified, it completes it using rules specified in `resolv.conf`
    /// search entries).  In addition, this also searches your hosts file.  Applies
    /// recursion if available and not turned off (it is on by default).
    ///
    /// This is the highest level resolver routine, and is the one called by
    /// gethostbyname.
    pub fn search(&mut self,
                  name: &[u8],
                  class: Class,
                  typ: RecordType)
                  -> Result<Response, Error>
    {
        let name = match CString::new(name) {
            Ok(c) => c,
            Err(n) => return Err(Error::CString(n)),
        };
        let buflen: usize = libresolv_sys::NS_PACKETSZ as usize;
        let mut buffer: Box<Vec<u8>> = Box::new(Vec::with_capacity(buflen));

        let rlen: i32 = unsafe {
            libresolv_sys::res_nsearch(
                &mut self.context,
                name.as_ptr(),
                class as i32,
                typ as i32,
                buffer.deref_mut().as_mut_ptr(),
                buflen as i32)
        };
        if rlen==-1 {
            return Err(From::from(self.get_error()));
        }

        let mut msg: libresolv_sys::__ns_msg = libresolv_sys::__ns_msg::default();
        unsafe {
            if libresolv_sys::ns_initparse(buffer.deref().as_ptr(), rlen, &mut msg) < 0 {
                return Err(Error::ParseError);
            }
        }

        Ok(Response::new(msg, buffer))
    }

    /// Lookup the record.  Does not apply the search algorithm, so `dname` must be a complete
    /// domain name, and only DNS will be consulted (not your hosts file).  Applies recursion
    /// if available and not turned off (it is on by default).
    pub fn query(&mut self,
                 dname: &[u8],
                 class: Class,
                 typ: RecordType)
                 -> Result<Response, Error>
    {
        let name = match CString::new(dname) {
            Ok(c) => c,
            Err(n) => return Err(Error::CString(n)),
        };
        let buflen: usize = libresolv_sys::NS_PACKETSZ as usize;
        let mut buffer: Box<Vec<u8>> = Box::new(Vec::with_capacity(buflen));

        let rlen: i32 = unsafe {
            libresolv_sys::res_nquery(
                &mut self.context,
                name.as_ptr(),
                class as i32,
                typ as i32,
                buffer.deref_mut().as_mut_ptr(),
                buflen as i32)
        };
        if rlen==-1 {
            return Err(From::from(self.get_error()));
        }

        let mut msg: libresolv_sys::__ns_msg = libresolv_sys::__ns_msg::default();
        unsafe {
            if libresolv_sys::ns_initparse(buffer.deref().as_ptr(), rlen, &mut msg) < 0 {
                return Err(Error::ParseError);
            }
        }

        Ok(Response::new(msg, buffer))
    }

    fn get_error(&self) -> ResolutionError
    {
        match self.context.res_h_errno {
            0 => ResolutionError::Success,
            1 => ResolutionError::HostNotFound,
            2 => ResolutionError::TryAgain,
            3 => ResolutionError::NoRecovery,
            4 => ResolutionError::NoData,
            _ => ResolutionError::HostNotFound, // fallback
        }
    }
}
