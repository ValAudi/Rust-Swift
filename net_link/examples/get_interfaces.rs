use std::os::raw::*;

fn main() {
    let interfaces = pnet::datalink::interfaces();

    let r = interfaces[0].name.len() as c_uint;
    println!("{:#?}", interfaces[1]);
    println!("{}", r);
}
