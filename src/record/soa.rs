
use super::{RecordData, RecordType};
use crate::error::Error;
use libresolv_sys::__ns_rr as Rr;
use libresolv_sys::__ns_msg as Message;
use libresolv_sys::MAXDNAME;
use byteorder::{BigEndian, ByteOrder};
use std::ffi::CStr;
use std::slice;

#[derive(Debug, Clone)]
pub struct SOA {
    pub mname: String,
    pub rname: String,
    pub serial: u32,
    pub refresh: u32,
    pub retry: u32,
    pub expire: u32,
    pub minimum: u32,
}

impl RecordData for SOA {
    fn get_record_type() -> RecordType {
        RecordType::SOA
    }

    fn extract(msg: &mut Message, rr: &Rr) -> Result<SOA, Error> {
        if rr.type_ != Self::get_record_type() as u16 { return Err(Error::WrongRRType); }

        let mut soa = SOA {
            mname: "".to_owned(),
            rname: "".to_owned(),
            serial: 0,
            refresh: 0,
            retry: 0,
            expire: 0,
            minimum: 0,
        };

        let mut buffer: [u8; MAXDNAME as usize] = [0; MAXDNAME as usize];

        let mut offset = 0;

        soa.mname = {
            let count = unsafe { ::libresolv_sys::ns_name_uncompress(
                msg._msg,
                msg._eom,
                rr.rdata.offset(offset),
                buffer.as_mut_ptr() as *mut i8,
                MAXDNAME as u64)
            };
            if count < 0 {
                return Err(Error::UncompressError);
            }
            offset += count as isize;
            unsafe { CStr::from_ptr(buffer.as_ptr() as *const i8)
                     .to_string_lossy().into_owned() }
        };

        soa.rname = {
            let count = unsafe { ::libresolv_sys::ns_name_uncompress(
                msg._msg,
                msg._eom,
                rr.rdata.offset(offset),
                buffer.as_mut_ptr() as *mut i8,
                MAXDNAME as u64)
            };
            if count < 0 {
                return Err(Error::UncompressError);
            }
            offset += count as isize;
            unsafe { CStr::from_ptr(buffer.as_ptr() as *const i8)
                     .to_string_lossy().into_owned() }
        };

        soa.serial = unsafe {
            let slice: &[u8] = slice::from_raw_parts(rr.rdata.offset(offset), 4);
            BigEndian::read_u32(slice)
        };
        offset += 4;

        soa.refresh = unsafe {
            let slice: &[u8] = slice::from_raw_parts(rr.rdata.offset(offset), 4);
            BigEndian::read_u32(slice)
        };
        offset += 4;

        soa.retry = unsafe {
            let slice: &[u8] = slice::from_raw_parts(rr.rdata.offset(offset), 4);
            BigEndian::read_u32(slice)
        };
        offset += 4;

        soa.expire = unsafe {
            let slice: &[u8] = slice::from_raw_parts(rr.rdata.offset(offset), 4);
            BigEndian::read_u32(slice)
        };
        offset += 4;

        soa.minimum = unsafe {
            let slice: &[u8] = slice::from_raw_parts(rr.rdata.offset(offset), 4);
            BigEndian::read_u32(slice)
        };

        Ok(soa)
    }
}

