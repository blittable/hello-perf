fn main() {
    println!("Building protobufs");

    tonic_build::compile_protos("proto/helloperf/helloperf.proto").unwrap();
}
