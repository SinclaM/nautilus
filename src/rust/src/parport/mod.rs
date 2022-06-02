use core::ffi::c_int;
use core::fmt::Error;

use alloc::string::String;
use bitfield::bitfield;

use chardev::NkCharDev;
use irq::Irq;
use portio::ParportIO;

pub mod nk_shell_cmd;

mod chardev;
mod irq;
mod portio;

const PARPORT0_BASE: u16 = 0x378;
const PARPORT0_IRQ: u8 = 7;

bitfield! {
  struct StatReg(u8);
  reserved, _: 1, 0;
  irq, _: 2;
  err, _: 3;
  sel, _: 4;
  pout, _: 5;
  ack, _: 6;
  busy, _: 7;
}

bitfield! {
struct CtrlReg(u8);
    strobe, set_strobe : 0;     // attached device strobe line - alert device to data (0->1->0)
    autolf, set_autolf : 1;     // attached device autolf line - auomatically add linefeeds to carriage returns (if 1)
    init, set_init : 2;         // attached device init line - init attached device (if 0)
    select, set_select : 3;     // attached device select print/in
    irq_en, set_irq_en : 4;     // enable interrupt when ack line is asserted by attached device
    bidir_en, set_bidir_en : 5; // select transfer direction 0 => write to attached device
    reserved, _ : 7, 6;         // reserved
}

struct DataReg {
    data: u8,
}

enum ParportStatus {
    Ready,
    Busy,
}

struct Parport<'a> {
    dev: NkCharDev<'a>,
    port: ParportIO,
    irq: Irq,
    state: ParportStatus,
}

//unsafe impl Sync for Parport {}
//unsafe impl Send for Parport {}
impl<'a> Parport<'a> {
    pub unsafe fn new(port: ParportIO, irq: Irq, name: &str) -> Result<Self, Error> {
        unimplemented!()
    }

    fn write(self, data: u8) {
        unimplemented!()
    }

    fn read(self) -> u8 {
        unimplemented!()
    }

    fn get_name(self) -> String {
        self.dev.get_name()
    }

    fn is_ready(self) -> bool {
        unimplemented!()
    }
}

fn discover_and_bringup_devices() -> Result<(), Error> {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn nk_parport_init() -> c_int {
    if discover_and_bringup_devices().is_err() {
        -1
    } else {
        0
    }
}
