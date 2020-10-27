extern crate pcan_basic_sys;
use pcan_basic_sys as pcan;

fn main() {
   println!("Connected USB busses: {:?}", pcan::scan_ifaces(pcan::CanBusKind::USB)[0]);
   // println!("{:?}", unsafe {pcan::CAN_Initialize(1, 0, 0, 0, 0)});
}