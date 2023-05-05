use pnet::{util::MacAddr, ipnetwork::IpNetwork};
use std::os::raw::*;
use std::{ffi::{CString}};

pub struct Interface {
    pub name: String,
    pub description: String,
    pub index: u32,
    pub mac: Option<MacAddr>,
    pub ips: Vec<IpNetwork>,
    pub flag: u32
}

impl Interface {
    fn new(name: String, description: String, index: u32, mac: Option<MacAddr>, ips: Vec<IpNetwork>, flag: u32) -> Interface {
        Interface { name, description, index, mac, ips, flag}
    }
}

pub struct Interfaces {
    pub list: Vec<Interface>
}

impl Interfaces {
    fn new() -> Interfaces {
        let interfaces = pnet::datalink::interfaces();
        let mut ifaces = Vec::new();
        for iface in interfaces {
            let mut ips = Vec::new();
            for ip in iface.ips {
                ips.push(ip);
            }
            ifaces.push(Interface::new(
                        iface.name.clone(),
                        iface.description.clone(),
                        iface.index.clone(),
                        iface.mac.clone(),
                        ips,
                        iface.flags,
                        )
                    );
        }
        Interfaces{ list: ifaces}
    }

    fn iter(&self) -> Iter<'_, Interface> {
        Iter {
            inner: self.list.iter()
        }
    }
}

struct Iter<'a, T: 'a> {
    inner: std::slice::Iter<'a, T>
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct InterfaceData {
    pub name: *mut i8,
    pub description: *mut i8,
    pub index: u32,
    pub mac: [u8; 6],
    pub ips: *mut i8,
    pub flag: c_uint
}

impl InterfaceData {
    pub fn new(interface: &Interface) -> Self {
        let name = CString::new(interface.name.clone()).unwrap();
        let description  = CString::new(interface.description.clone()).unwrap();
        let index = interface.index.clone();
        let ips = interface.ips.iter().map(|ip| ip.to_string()).collect::<Vec<String>>().join(",");
        let ips = CString::new(ips).unwrap();

        InterfaceData {
            name: name.into_raw(),
            description: description.into_raw(),
            index: index,
            mac: interface.mac.unwrap().into(),
            ips: ips.into_raw(),
            flag: interface.flag,
        }
    }
}

#[no_mangle]
pub extern "C" fn get_interface_data() -> *mut InterfaceData {
    let interfaces = Interfaces::new();
    let interfaces_data = interfaces.iter().map(|interface| InterfaceData::new(interface)).collect::<Vec<InterfaceData>>();
    let interface_data_ptr = interfaces_data.as_ptr() as *mut InterfaceData;
    std::mem::forget(interfaces_data);
    interface_data_ptr
}

#[no_mangle]
pub extern "C" fn get_interface_data_count() -> u32 {
    let interfaces = Interfaces::new();
    let interfaces_data = interfaces.iter().map(|interface| InterfaceData::new(interface)).collect::<Vec<InterfaceData>>();
    interfaces_data.len() as u32    
}
