fn main() {
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .out_dir("src")
        .compile(&[
            "proto/items.proto"
        ], &["proto/"])
        .unwrap();

    println!("cargo:rerun-if-changed=proto/items.proto");
}
