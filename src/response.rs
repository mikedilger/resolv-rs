
use libresolv_sys::ns_msg as Message;
use libresolv_sys::ns_sect as NSSection;
use libresolv_sys::ns_rr as Rr;

use crate::error::Error;
use crate::record::{Record, RecordData};

pub struct Flags(pub u16);

impl Flags {
    #[inline]
    pub fn question_response(&self) -> bool {
        ((self.0 & 0x8000) >> 15) > 0
    }
    #[inline]
    pub fn operation_code(&self) -> u16 {
        ((self.0 & 0x7800) >> 11) as u16
    }
    #[inline]
    pub fn authoritative_answer(&self) -> bool {
        ((self.0 & 0x0400) >> 10) > 0
    }
    #[inline]
    pub fn truncation_occurred(&self) -> bool {
        ((self.0 & 0x0200) >> 9) > 0
    }
    #[inline]
    pub fn recursion_desired(&self) -> bool {
        ((self.0 & 0x0100) >> 8) > 0
    }
    #[inline]
    pub fn recursion_available(&self) -> bool {
        ((self.0 & 0x0080) >> 7) > 0
    }
    #[inline]
    pub fn must_be_zero(&self) -> bool {
        ((self.0 & 0x0040) >> 6) > 0
    }
    #[inline]
    pub fn authentic_data(&self) -> bool {
        ((self.0 & 0x0020) >> 5) > 0
    }
    #[inline]
    pub fn checking_disabled(&self) -> bool {
        ((self.0 & 0x0010) >> 4) > 0
    }
    #[inline]
    pub fn response_code(&self) -> u16 {
        (self.0 & 0x000f) as u16
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Section {
    Question,
    Answer,
    Authority,
    Additional
}
impl Section {
    fn ns_sect(&self) -> NSSection
    {
        use ::libresolv_sys::{__ns_sect_ns_s_qd, __ns_sect_ns_s_an,
                              __ns_sect_ns_s_ns, __ns_sect_ns_s_ar};

        match *self {
            Section::Question => __ns_sect_ns_s_qd,
            Section::Answer => __ns_sect_ns_s_an,
            Section::Authority => __ns_sect_ns_s_ns,
            Section::Additional => __ns_sect_ns_s_ar,
        }
    }
}


pub struct Response {
    msg: Message,
    #[allow(dead_code)] // buffer is where the real data is; keep it until Response drops.
    buffer: Box<Vec<u8>>,
}

impl Response {
    /// This is for internal use.
    pub fn new(msg: Message, buffer: Box<Vec<u8>>) -> Response {
        Response {
            msg: msg,
            buffer: buffer
        }
    }

    /// Gets the ID field of the Name Server response
    pub fn get_id(&self) -> u16
    {
        self.msg._id
    }

    /// Gets flags (and opcodes) in the header of the Name Server response
    pub fn get_flags(&self) -> Flags
    {
        Flags(self.msg._flags)
    }

    /// Returns a count of how many records exist in the given section
    pub fn get_section_count(&self, section: Section) -> usize
    {
        self.msg._counts[section.ns_sect() as usize] as usize
    }

    /// Gets a record from a section.  Returns an error if index is out of bounds
    /// (use get_section_count()).  Also returns an error (at run-time) if assigned into
    /// a Record of the wrong type.
    pub fn get_record<T>(&mut self, section: Section, index: usize)
                         -> Result<Record<T>, Error>
        where T: RecordData
    {
        if index >= self.get_section_count(section) {
            return Err(Error::NoSuchSectionIndex(section, index));
        }
        let mut rr: Rr = Rr::default();
        unsafe {
            if ::libresolv_sys::ns_parserr(&mut self.msg, section.ns_sect(),
                                           index as i32, &mut rr) < 0
            {
                return Err(Error::ParseError);
            }
        }

        Record::extract(&mut self.msg, &rr)
    }

    pub fn questions<T>(&mut self) -> RecordItems<T>
        where T: RecordData
    {
        RecordItems {
            msg: &mut self.msg,
            section: Section::Question,
            index: 0,
            _marker: ::std::marker::PhantomData,
        }
    }

    pub fn answers<T>(&mut self) -> RecordItems<T>
        where T: RecordData
    {
        RecordItems {
            msg: &mut self.msg,
            section: Section::Answer,
            index: 0,
            _marker: ::std::marker::PhantomData,
        }
    }

    pub fn authorities<T>(&mut self) -> RecordItems<T>
        where T: RecordData
    {
        RecordItems {
            msg: &mut self.msg,
            section: Section::Authority,
            index: 0,
            _marker: ::std::marker::PhantomData,
        }
    }

    pub fn additional_records<T>(&mut self) -> RecordItems<T>
        where T: RecordData
    {
        RecordItems {
            msg: &mut self.msg,
            section: Section::Additional,
            index: 0,
            _marker: ::std::marker::PhantomData,
        }
    }
}

/// An iterator to iterate through DNS records
pub struct RecordItems<'a, T: RecordData> {
    msg: &'a mut Message,
    section: Section,
    index: usize,
    _marker: ::std::marker::PhantomData<T>,
}

impl<'a, T: RecordData> Iterator for RecordItems<'a, T> {
    type Item = Record<T>;

    fn next(&mut self) -> Option<Record<T>>
    {
        let len = self.msg._counts[self.section.ns_sect() as usize] as usize;

        loop {
            if self.index >= len {
                return None;
            }

            let mut rr: Rr = Rr::default();
            unsafe {
                if ::libresolv_sys::ns_parserr(self.msg, self.section.ns_sect(),
                                               self.index as i32, &mut rr) < 0
                {
                    self.index = len;
                    return None;
                }
            }
            self.index += 1;

            match Record::extract(self.msg, &rr) {
                Ok(record) => return Some(record),
                Err(_e) => {} // skip the record by looping around
            }
        }
    }
}
