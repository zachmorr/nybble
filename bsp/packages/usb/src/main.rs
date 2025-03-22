
use std::fs;
use std::env;
use std::io::Write;
use log::debug;
use std::ffi::CString;
use std::path::Path;

const FUNCTIONFS_STRINGS_MAGIC: u32 = 2;
const FUNCTIONFS_DESCRIPTORS_MAGIC_V2: u32 = 3;
const INTERFACE_BDESCRIPTOR_TYPE: u8 = 0x04;
const ENDPOINT_BDESCRIPTOR_TYPE: u8 = 0x05;

#[derive(Debug)]
enum USBError {
    EndpointAddressSize,
}

#[repr(u8)]
#[derive(PartialEq)]
enum Direction {
    In = 0x80,
    Out = 0x00,
}

#[repr(u8)]
enum TransferType {
    Control = 0,
    Isochronous = 1,
    Bulk = 2,
    Interrupt = 3,
}

enum Speed {
    Full,
    High,
    Super,
}

mod FunctionFSFlags {
    pub const FUNCTIONFS_HAS_FS_DESC: u32 = 1;
	pub const FUNCTIONFS_HAS_HS_DESC: u32 = 2;
	pub const FUNCTIONFS_HAS_SS_DESC: u32 = 4;
	pub const FUNCTIONFS_HAS_MS_OS_DESC: u32 = 8;
	pub const FUNCTIONFS_VIRTUAL_ADDR: u32 = 16;
	pub const FUNCTIONFS_EVENTFD: u32 = 32;
	pub const FUNCTIONFS_ALL_CTRL_RECIP: u32 = 64;
	pub const FUNCTIONFS_CONFIG0_SETUP: u32 = 128;
}

trait Descriptor {
    fn descriptor_type(&self) -> u8;
    fn payload(&self) -> Vec<u8>;
    fn descriptor(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = self.payload();
        let descriptor_size: u8 = u8::try_from(bytes.len()).expect("Unable to convert descriptor size into u8") + 2;
        bytes.insert(0, self.descriptor_type());
        bytes.insert(0, descriptor_size);
        bytes
    }
}

#[derive(Debug)]
struct Interface {
    number: u8,
    alternate: u8,
    num_endpoints: u8,
    class: u8,
    subclass: u8,
    protocol: u8,
    string_index: u8,
    endpoints: Vec<Endpoint>,
}

impl Interface {
    fn new(class: u8, subclass: u8, protocol: u8) -> Self {
        Interface {
            number: 0,
            alternate: 0,
            num_endpoints: 0,
            class: class,
            subclass: subclass,
            protocol: protocol,
            string_index: 0,
            endpoints: Vec::new(),
        }
    }

    fn add_endpoint(&mut self, mut endpoint: Endpoint) -> Result<(), USBError> {
        if self.endpoints.len() >= 16 {
            return Err(USBError::EndpointAddressSize)
        }
        let ep_count: u8 = self.endpoints.len() as u8;
        endpoint.set_address(ep_count + 1).unwrap();
        self.endpoints.push(endpoint);
        self.num_endpoints += 1;
        Ok(())
    }
}

impl Descriptor for Interface {
    fn descriptor_type(&self) -> u8 {
        INTERFACE_BDESCRIPTOR_TYPE
    }

    fn payload(&self) -> Vec<u8> {
        let mut payload = vec![self.number];
        payload.push(self.alternate);
        payload.push(self.num_endpoints);
        payload.push(self.class);
        payload.push(self.subclass);
        payload.push(self.protocol);
        payload.push(self.string_index);
        payload
    }
}


#[derive(Debug)]
struct Endpoint {
    address: u8,
    attributes: u8,
    max_packet_size: u16,
    interval: u8
}

impl Endpoint {
    fn new(direction: Direction, transfer_type: TransferType) -> Self {
        let endpoint = Endpoint {
            address: direction as u8,
            attributes: transfer_type as u8,
            max_packet_size: 0,
            interval: 1,
        };

        endpoint
    }

    fn set_address(&mut self, addr: u8) -> Result<(), USBError> {
        if addr >= 16{
            return Err(USBError::EndpointAddressSize)
        }
        self.address |= addr;
        Ok(())
    }
}

impl Descriptor for Endpoint {
    fn descriptor_type(&self) -> u8 {
        ENDPOINT_BDESCRIPTOR_TYPE
    }

    fn payload(&self) -> Vec<u8> {
        let mut payload = vec![self.address];
        payload.push(self.attributes);
        payload.extend(self.max_packet_size.to_le_bytes());
        payload.push(self.interval);
        payload
    }
}

struct Function {
    flags: u32,
    fs_interfaces: Vec<Interface>,
    hs_interfaces: Vec<Interface>,
    ss_interfaces: Vec<Interface>,
    ms_os_interfaces: Vec<Interface>,
    strings: Vec<CString>,
}

impl Function {
    fn new() -> Self {
        Function {
            flags: 0,
            fs_interfaces: Vec::new(),
            hs_interfaces: Vec::new(),
            ss_interfaces: Vec::new(),
            ms_os_interfaces: Vec::new(),
            strings: Vec::new(),
        }
    }

    fn add_string(&mut self, string: &str) -> u8 {
        self.strings.push(
            CString::new(string).expect("Cannot convert into C string")
        );

        let string_index: u8 = self.strings.len().try_into().expect("There's more that 255 strings?");
        string_index
    }

    fn add_fs_interface(&mut self, mut interface: Interface, name: &str){
        let string_index = self.add_string(name);
        interface.string_index = string_index;

        let fs_desc_count: u8 = self.fs_interfaces.len().try_into().expect("Unable to add >255 interfaces");
        interface.number = fs_desc_count;
        interface.alternate = 0;

        self.fs_interfaces.push(interface);
        self.flags |= FunctionFSFlags::FUNCTIONFS_HAS_FS_DESC;
    }

    fn ffs_descriptors(&self) -> Vec<u8> {
        let mut header: Vec<u8> = Vec::new();
        let mut descriptors: Vec<u8> = Vec::new();
    
        if self.flags & FunctionFSFlags::FUNCTIONFS_HAS_FS_DESC != 0 {
            let mut fs_descriptor_count: u32 = 0;
            for interface in self.fs_interfaces.iter() {
                descriptors.extend(interface.descriptor());
                fs_descriptor_count += 1;

                for endpoint in interface.endpoints.iter() {
                    descriptors.extend(endpoint.descriptor());
                   fs_descriptor_count += 1;
                }   
            }
            header.extend(fs_descriptor_count.to_le_bytes());
        }

        descriptors.splice(0..0, header);
        descriptors.splice(0..0, self.flags.to_le_bytes());

        let descriptors_len: u32 = descriptors.len().try_into().expect("descriptors packet size cannot fit into u32");
        descriptors.splice(0..0, (descriptors_len + 8).to_le_bytes());
        descriptors.splice(0..0, FUNCTIONFS_DESCRIPTORS_MAGIC_V2.to_le_bytes());
        descriptors
    }
    
    fn string_descriptors(&self) -> Vec<u8> {
        let mut string_data: Vec<u8> = Vec::new();

        for string in self.strings.iter() {
            let lang_code: u16 = 0x0409;
            string_data.extend(lang_code.to_le_bytes());
            string_data.extend(string.as_bytes_with_nul());
        }

        
        let str_count: u32 = self.strings.len().try_into().expect("string number cannot fit into u32");
        let data_len: u32 = string_data.len().try_into().expect("string descriptor size cannot fit into u32");
        let mut descriptors: Vec<u8> = Vec::new();
        descriptors.extend(FUNCTIONFS_STRINGS_MAGIC.to_le_bytes());
        descriptors.extend((data_len + 16).to_le_bytes());
        descriptors.extend(str_count.to_le_bytes());
        descriptors.extend(1u32.to_le_bytes());
        descriptors.extend(string_data);
        descriptors
    }
}

fn main() {
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    let ep0_arg = args.get(1).expect("Not enoug h arguments!");
    let ep0_path = Path::new(ep0_arg);

    if !ep0_path.exists() {
        println!("{} does not exist!", ep0_path.display());
        std::process::exit(1);
    }

    let ep_in = Endpoint::new(Direction::In, TransferType::Bulk);
    let ep_out = Endpoint::new(Direction::Out, TransferType::Bulk);

    let mut interface = Interface::new(255, 1, 2);
    interface.add_endpoint(ep_out).unwrap();
    interface.add_endpoint(ep_in).unwrap();

    let mut function = Function::new();
    function.add_fs_interface(interface, "custom interface?");
    let ffs_descriptors = function.ffs_descriptors();
    let string_descriptors = function.string_descriptors();
    
    let mut ep0 = fs::File::options().read(true).write(true).open(ep0_path).unwrap();
    

    let ffs_descriptors: [u8; 39] = [
        0x03, 0, 0, 0, 
        0x27, 0, 0, 0, 
        0x01, 0, 0, 0, 
        0x03, 0, 0, 0, 
        9, 4, 0, 0, 2, 0xff, 1, 2, 1, 
        7, 5, 1, 2, 0, 0, 1, 
        7, 5, 0x82, 2, 0, 0, 0, 
    ];
    debug!("Writing FFS descriptors: {:X?}", ffs_descriptors);
    ep0.write(&ffs_descriptors).expect("Failed to write ffs descriptors");

    let string_descriptors: [u8; 35] = [
        0x2, 0x0, 0x0, 0x0, 
        0x23, 0x0, 0x0, 0, 
        1, 0, 0, 0, 
        1, 0, 0, 0, 
        9, 4, 
        0x63, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x20, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x66, 0x61, 0x63, 0x65, 0
    ];
    debug!("Writing string descriptors: {:X?}", string_descriptors);
    ep0.write(&string_descriptors).expect("Failed to write string descriptors");

    loop {
    }
}
