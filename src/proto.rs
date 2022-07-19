//Include generated cl_rcon.proto
pub mod client {
    include!(concat!(env!("OUT_DIR"), "/cl_rcon.rs"));
}

//Inlcude generated sv_rcon.proto
pub mod server {
    include!(concat!(env!("OUT_DIR"), "/sv_rcon.rs"));
}
