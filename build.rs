fn main() {
    tonic_build::compile_protos("proto/helloperf/helloperf.proto").unwrap();
}
