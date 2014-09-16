// TODO Copyright Header

//! actually printing debug info.

use core::fmt::*;
use core::result::Ok;
use core::slice::*;
use io;

pub struct DbgWriter;

pub static PORT : u16 = 0x3f8;
pub static PORT_INTR : u8 = 0x0d;
pub static DBG_WRITER : DbgWriter = DbgWriter;

impl FormatWriter for DbgWriter {
    fn write(&mut self, data: &[u8]) -> Result {
        for &x in data.iter() {
            unsafe {
                while io::inb(PORT + 5) & 0x20 == 0 {}
                io::outb(PORT, x);
            }
        }
        Ok(())
    }
}
