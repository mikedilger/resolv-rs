
use super::{RecordData, RecordType};
use ::error::Error;
use ::libresolv_sys::__ns_rr as Rr;
use ::libresolv_sys::__ns_msg as Message;
use ::libresolv_sys::MAXDNAME;

use std::ffi::CStr;

#[derive(Debug, Clone)]
pub struct SOA {
    mname: String,
    rname: String,
    serial: u32,
    refresh: u32,
    retry: u32,
    expire: u32,
    minimum: u32,
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
                MAXDNAME as usize)
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
                MAXDNAME as usize)
            };
            if count < 0 {
                return Err(Error::UncompressError);
            }
            offset += count as isize;
            unsafe { CStr::from_ptr(buffer.as_ptr() as *const i8)
                     .to_string_lossy().into_owned() }
        };

        soa.serial = unsafe {
            ((*rr.rdata.offset(offset) as u32) << 24)
                + ((*rr.rdata.offset(offset+1) as u32) << 16)
                + ((*rr.rdata.offset(offset+2) as u32) << 8)
                + (*rr.rdata.offset(offset+3) as u32)
        };
        offset += 4;

        soa.refresh = unsafe {
            ((*rr.rdata.offset(offset) as u32) << 24)
                + ((*rr.rdata.offset(offset+1) as u32) << 16)
                + ((*rr.rdata.offset(offset+2) as u32) << 8)
                + (*rr.rdata.offset(offset+3) as u32)
        };
        offset += 4;

        soa.retry = unsafe {
            ((*rr.rdata.offset(offset) as u32) << 24)
                + ((*rr.rdata.offset(offset+1) as u32) << 16)
                + ((*rr.rdata.offset(offset+2) as u32) << 8)
                + (*rr.rdata.offset(offset+3) as u32)
        };
        offset += 4;

        soa.expire = unsafe {
            ((*rr.rdata.offset(offset) as u32) << 24)
                + ((*rr.rdata.offset(offset+1) as u32) << 16)
                + ((*rr.rdata.offset(offset+2) as u32) << 8)
                + (*rr.rdata.offset(offset+3) as u32)
        };
        offset += 4;

        soa.minimum = unsafe {
            ((*rr.rdata.offset(offset) as u32) << 24)
                + ((*rr.rdata.offset(offset+1) as u32) << 16)
                + ((*rr.rdata.offset(offset+2) as u32) << 8)
                + (*rr.rdata.offset(offset+3) as u32)
        };

        Ok(soa)
    }
}

