extern crate prost_build;

fn main() {
    prost_build::compile_protos(&["src/cl_rcon.proto", "src/sv_rcon.proto"], &["src/"]).unwrap();
}
