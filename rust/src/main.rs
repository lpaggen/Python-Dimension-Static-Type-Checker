use prost::Message;


mod ir;

pub mod pdc {
    pub mod ir {
        include!(concat!(env!("OUT_DIR"), "/pdc.ir.rs"));
    }
}

fn main() {}