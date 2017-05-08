extern crate pcan_basic_bindings;
extern crate libloading;
use pcan_basic_bindings as pcan;

fn main() {
   println!("{:?}", unsafe {pcan::CAN_Initialize(1, 0, 0, 0, 0)});
}