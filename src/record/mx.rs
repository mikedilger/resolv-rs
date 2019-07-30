
use super::{RecordData, RecordType};
use crate::error::Error;
use libresolv_sys::__ns_rr as Rr;
use libresolv_sys::__ns_msg as Message;
use libresolv_sys::MAXDNAME;
use byteorder::{BigEndian, ByteOrder};
use std::ffi::CStr;
use std::slice;

#[derive(Debug, Clone)]
pub struct MX {
    pub preference: i16,
    pub exchange: String,
}

impl RecordData for MX {
    fn get_record_type() -> RecordType {
        RecordType::MX
    }

    fn extract(msg: &mut Message, rr: &Rr) -> Result<MX, Error> {
        if rr.type_ != Self::get_record_type() as u16 { return Err(Error::WrongRRType); }

        let mut buffer: [u8; MAXDNAME as usize] = [0; MAXDNAME as usize];

        let size = unsafe { ::libresolv_sys::ns_name_uncompress(
            msg._msg,
            msg._eom,
            rr.rdata.offset(2),
            buffer.as_mut_ptr() as *mut i8,
            MAXDNAME as usize)
        };
        if size < 0 {
            return Err(Error::UncompressError);
        }

        Ok(MX {
            preference: unsafe {
                let slice: &[u8] = slice::from_raw_parts(rr.rdata, 2);
                BigEndian::read_i16(slice)
            },
            exchange: unsafe { CStr::from_ptr(buffer.as_ptr() as *const i8)
                               .to_string_lossy().into_owned() },
        })
    }
}
