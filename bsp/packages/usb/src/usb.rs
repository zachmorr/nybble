use log::trace;
use std::io::{Error, ErrorKind, Result, Read, Write};
use std::path::{Path, PathBuf};

/*
Documentation
https://www.beyondlogic.org/usbnutshell/usb6.shtml#SetupPacket-
https://github.com/torvalds/linux/blob/master/include/uapi/linux/usb/ch9.h#L788
https://github.com/torvalds/linux/blob/master/include/uapi/linux/usb/functionfs.h#L145
https://github.com/torvalds/linux/blob/master/drivers/usb/gadget/function/f_fs.c#L2919
https://github.com/torvalds/linux/blob/master/tools/usb/ffs-test.c#L659
*/

pub mod event {
    pub const BIND: u8 = 0;
    pub const UNBIND: u8 = 1;
    pub const ENABLE: u8 = 2;
    pub const DISABLE: u8 = 3;
    pub const SETUP: u8 = 4;
    pub const SUSPEND: u8 = 5;
    pub const RESUME: u8 = 6;
}

#[derive(Debug, PartialEq, Clone)]
pub enum Event {
    Bind,
    Unbind,
    Enable,
    Disable,
    Setup,
    Suspend,
    Resume,
    Unknown(Vec<u8>),
}

fn parse_event(buf: &[u8]) -> Event {
    let event = match buf.get(8) {
        Some(&event::BIND) => Event::Bind,
        Some(&event::UNBIND) => Event::Unbind,
        Some(&event::ENABLE) => Event::Enable,
        Some(&event::DISABLE) => Event::Disable,
        Some(&event::SUSPEND) => Event::Suspend,
        Some(&event::RESUME) => Event::Resume,
        Some(&event::SETUP) => Event::Setup,
        _other => Event::Unknown(buf.to_vec()),
    };
    event
}

pub struct Function {
    pub ffs_dir: PathBuf,
    ctrl: std::fs::File,
}

impl Function {
    pub fn new(ffs_dir: &Path) -> Result<Self> {
        let file = std::fs::File::options()
            .read(true)
            .write(true)
            .open(&ffs_dir.join("ep0"))?;
        Ok(Function {
            ffs_dir: ffs_dir.to_owned(),
            ctrl: file,
        })
    }

    pub fn write_descriptors(&mut self) -> Result<()> {
        pub const FUNC_DESCRIPTORS: [u8; 66] = [
            0x03, 0x00, 0x00, 0x00, 0x42, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x03, 0x00,
            0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x09, 0x04, 0x00, 0x00, 0x02, 0xff, 0x01, 0x02,
            0x01, 0x07, 0x05, 0x01, 0x02, 0x00, 0x02, 0x01, 0x07, 0x05, 0x82, 0x02, 0x00, 0x02,
            0x00, 0x09, 0x04, 0x00, 0x00, 0x02, 0xff, 0x01, 0x02, 0x01, 0x07, 0x05, 0x01, 0x02,
            0x00, 0x02, 0x01, 0x07, 0x05, 0x82, 0x02, 0x00, 0x02, 0x00,
        ];

        trace!("Writing FFS descriptors: {:X?}", FUNC_DESCRIPTORS);
        self.ctrl.write(&FUNC_DESCRIPTORS)?;

        pub const STRING_DESCRIPTORS: [u8; 35] = [
            0x02, 0x00, 0x00, 0x00, 0x23, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00,
            0x00, 0x00, 0x09, 0x04, 0x63, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x20, 0x69, 0x6e, 0x74,
            0x65, 0x72, 0x66, 0x61, 0x63, 0x65, 0x00,
        ];
        trace!("Writing string descriptors: {:X?}", STRING_DESCRIPTORS);
        self.ctrl.write(&STRING_DESCRIPTORS)?;

        Ok(())
    }

    pub fn event(&mut self) -> Result<Event> {
        let mut buf = [0u8; 512];
        let n = self.ctrl.read(&mut buf)?;
        trace!("event: {} bytes: {:?}", n, &buf[..n]);
        if n == 0 {
            return Err(Error::new(
                ErrorKind::UnexpectedEof,
                "control endpoint returned 0 bytes",
            ));
        }
        let event = parse_event(&buf[..n]);
        Ok(event)
    }

    pub fn open_endpoints(&self) -> Result<(std::fs::File, std::fs::File)> {
        let ep_out = self.ffs_dir.join("ep1");
        let ep_in = self.ffs_dir.join("ep2");

        let writer = std::fs::File::options().write(true).open(&ep_in)?;

        let reader = std::fs::File::options().read(true).open(&ep_out)?;

        Ok((writer, reader))
    }
}
