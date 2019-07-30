use super::{RecordData, RecordType};
use crate::error::Error;
use libresolv_sys::__ns_rr as Rr;
use libresolv_sys::__ns_msg as Message;
use std::io::Cursor;
use byteorder::ReadBytesExt;
use std::slice;

#[derive(Debug, Clone)]
pub struct TLSA {
    pub usage: u8,
    pub selector: u8,
    pub matching_type: u8,
    pub data: Vec<u8>,
}

impl RecordData for TLSA {
    fn get_record_type() -> RecordType {
        RecordType::TLSA
    }

    fn extract(_msg: &mut Message, rr: &Rr) -> Result<TLSA, Error> {
        if rr.type_ != Self::get_record_type() as u16 { return Err(Error::WrongRRType); }

        let mut reader = unsafe{
            Cursor::new(slice::from_raw_parts(rr.rdata, rr.rdlength as usize))
        };

        Ok(TLSA {
            usage: reader.read_u8().unwrap(),
            selector: reader.read_u8().unwrap(),
            matching_type: reader.read_u8().unwrap(),
            data: reader.into_inner()[3..].to_vec(),
        })
    }
}
