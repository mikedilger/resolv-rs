use super::{RecordData, RecordType};
use crate::error::Error;
use libresolv_sys::ns_msg as Message;
use libresolv_sys::ns_rr as Rr;

use std::mem;
use std::net::Ipv6Addr;

#[derive(Debug, Clone)]
pub struct AAAA {
    pub address: Ipv6Addr,
}

impl RecordData for AAAA {
    fn get_record_type() -> RecordType {
        RecordType::AAAA
    }

    fn extract(_msg: &mut Message, rr: &Rr) -> Result<AAAA, Error> {
        if rr.type_ != Self::get_record_type() as u16 {
            return Err(Error::WrongRRType);
        }

        Ok(AAAA {
            address: Ipv6Addr::from(unsafe {
                let input: &[u8; 16] = mem::transmute(rr.rdata);
                *input
            }),
        })
    }
}
