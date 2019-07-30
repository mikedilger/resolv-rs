
use super::{RecordData, RecordType};
use crate::error::Error;
use libresolv_sys::__ns_rr as Rr;
use libresolv_sys::__ns_msg as Message;

#[derive(Debug, Clone)]
pub struct TXT {
    pub dname: String,
}

impl RecordData for TXT {
    fn get_record_type() -> RecordType {
        RecordType::TXT
    }

    fn extract(_msg: &mut Message, rr: &Rr) -> Result<TXT, Error> {
        if rr.type_ != Self::get_record_type() as u16 { return Err(Error::WrongRRType); }

        let slice: &[u8] = unsafe {
            ::std::slice::from_raw_parts(rr.rdata.offset(1), *rr.rdata as usize)
        };

        Ok(TXT {
            dname: String::from_utf8_lossy(slice).into_owned()
        })
    }
}
