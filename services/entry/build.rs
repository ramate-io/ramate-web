use std::{env, path::PathBuf};

fn main() {

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let proto_file = "./proto/entry.proto"; 
    let proto_descriptor_file = "entry_descriptor.bin";

    tonic_build::configure()
        .file_descriptor_set_path(
            out_dir.join(proto_descriptor_file)
        )
        .build_server(true)
        .out_dir("src")
        .compile(&[proto_file], &["."])
        .unwrap_or_else(|e| panic!("protobuf compile error: {}", e));
  
        println!("cargo:rerun-if-changed={}", proto_file);
}