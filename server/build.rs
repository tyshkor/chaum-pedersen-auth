fn main() {
    tonic_build::configure()
        .compile(&["proto/zkp_auth.proto"], &["../"])
        .unwrap_or_else(|e| panic!("Failed to compile Protobuf definitions: {}", e));
}
