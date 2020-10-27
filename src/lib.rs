pub mod pcan_api;
pub use pcan_api::*;
use std::{fmt, mem};
use std::fmt::Formatter;
use std::os::raw::c_void;

pub enum CanBusKind {
    ISA,
    PCI,
    USB,
    LAN,
}

type Channel = u16;

impl fmt::Display for CanBusKind {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            CanBusKind::ISA => write!(f, "ISA"),
            CanBusKind::PCI => write!(f, "PCI"),
            CanBusKind::USB => write!(f, "USB"),
            CanBusKind::LAN => write!(f, "LAN"),
        }
    }
}

pub fn scan_ifaces(can_bus_kind: CanBusKind) -> Vec<Channel> {
    match can_bus_kind {
        CanBusKind::USB => {
            let mut i_buffer: u32 = 0;
            let i_buffer_ptr: *mut u32 = &mut i_buffer;
            let is_connected_predicate = |i: u32| -> Option<u16> {
                let res = unsafe {
                    CAN_GetValue(i as u16, PCAN_CHANNEL_CONDITION as u8, i_buffer_ptr as *mut c_void, mem::size_of_val(&i_buffer) as u32)
                };
                if (res == PCAN_ERROR_OK) & ((i_buffer & PCAN_CHANNEL_AVAILABLE) == PCAN_CHANNEL_AVAILABLE) {
                    Some(i as u16)
                } else {
                    None
                }
            };
            let mut channels: Vec<Channel> = (PCAN_USBBUS1..=PCAN_USBBUS8).filter_map(is_connected_predicate).collect();
            channels.extend((PCAN_USBBUS9..=PCAN_USBBUS16).filter_map(is_connected_predicate));
            channels
        }
        _ => unimplemented!()
    }
}