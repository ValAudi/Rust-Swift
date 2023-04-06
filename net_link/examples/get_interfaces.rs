
// use net_link::*;
use std::os::raw::*;

fn main() {
    let interfaces = pnet::datalink::interfaces();

    let r = interfaces[0].name.len() as c_uint;
    println!("{:#?}", interfaces[1]);
    println!("{}", r);
    // let l = get_interface_data_count();
    // let v = unsafe{ std::slice::from_raw_parts(prt, l as usize) };
    // for slice in v {
    //     println!("{:#?}", slice);
    // }
}
