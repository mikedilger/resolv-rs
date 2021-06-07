use super::{RecordData, RecordType};
use crate::error::Error;
use byteorder::{BigEndian, ByteOrder};
use libresolv_sys::__ns_msg as Message;
use libresolv_sys::__ns_rr as Rr;
use libresolv_sys::MAXHOSTNAMELEN;
use std::ffi::CStr;
use std::slice;

#[derive(Debug, Clone)]
pub struct SRV {
    pub priority: i16,
    pub weight: i16,
    pub port: u16,
    pub name: String,
}

impl RecordData for SRV {
    fn get_record_type() -> RecordType {
        RecordType::SRV
    }

    fn extract(msg: &mut Message, rr: &Rr) -> Result<SRV, Error> {
        if rr.type_ != Self::get_record_type() as u16 {
            return Err(Error::WrongRRType);
        }

        let mut buffer: [u8; MAXHOSTNAMELEN as usize] = [0; MAXHOSTNAMELEN as usize];
        let size = unsafe {
            ::libresolv_sys::ns_name_uncompress(
                msg._msg,
                msg._eom,
                rr.rdata.offset(6),
                buffer.as_mut_ptr() as *mut i8,
                MAXHOSTNAMELEN as u64,
            )
        };
        if size < 0 {
            return Err(Error::UncompressError);
        }

        Ok(SRV {
            priority: unsafe {
                let slice: &[u8] = slice::from_raw_parts(rr.rdata.offset(0), 2);
                BigEndian::read_i16(slice)
            },
            weight: unsafe {
                let slice: &[u8] = slice::from_raw_parts(rr.rdata.offset(2), 2);
                BigEndian::read_i16(slice)
            },
            port: unsafe {
                let slice: &[u8] = slice::from_raw_parts(rr.rdata.offset(4), 2);
                BigEndian::read_u16(slice)
            },
            name: unsafe {
                CStr::from_ptr(buffer.as_ptr() as *const i8)
                    .to_string_lossy()
                    .into_owned()
            },
        })
    }
}
