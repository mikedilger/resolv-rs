
use super::{RecordData, RecordType};
use ::error::Error;
use ::libresolv_sys::__ns_rr as Rr;
use ::libresolv_sys::__ns_msg as Message;
use ::libresolv_sys::MAXDNAME;

use std::ffi::CStr;

#[derive(Debug, Clone)]
pub struct CNAME {
    pub cname: String,
}

impl RecordData for CNAME {
    fn get_record_type() -> RecordType {
        RecordType::CNAME
    }

    fn extract(msg: &mut Message, rr: &Rr) -> Result<CNAME, Error> {
        if rr.type_ != Self::get_record_type() as u16 { return Err(Error::WrongRRType); }

        let mut buffer: [u8; MAXDNAME as usize] = [0; MAXDNAME as usize];

        let size = unsafe { ::libresolv_sys::ns_name_uncompress(
            msg._msg,
            msg._eom,
            rr.rdata,
            buffer.as_mut_ptr() as *mut i8,
            MAXDNAME as usize)
        };
        if size < 0 {
            return Err(Error::UncompressError);
        }

        Ok(CNAME {
            cname: unsafe { CStr::from_ptr(buffer.as_ptr() as *const i8)
                            .to_string_lossy().into_owned() },
        })
    }
}
