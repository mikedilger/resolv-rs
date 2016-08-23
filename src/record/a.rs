
use super::{RecordData, RecordType};
use ::error::Error;
use ::libresolv_sys::__ns_rr as Rr;
use ::libresolv_sys::__ns_msg as Message;

use std::net::Ipv4Addr;
use std::mem;

#[derive(Debug, Clone)]
pub struct A {
    pub address: Ipv4Addr,
}

impl RecordData for A {
    fn get_record_type() -> RecordType {
        RecordType::A
    }

    fn extract(_msg: &mut Message, rr: &Rr) -> Result<A, Error> {
        if rr.type_ != Self::get_record_type() as u16 { return Err(Error::WrongRRType); }
        Ok(A {
            address: Ipv4Addr::from( unsafe {
                let input: &[u8; 4] = mem::transmute(rr.rdata);
                *input
            }),
        })
    }
}
