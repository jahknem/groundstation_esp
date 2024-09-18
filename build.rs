fn main() {
    embuild::espidf::sysenv::output();
    prost_build::compile_protos(&["src/command.proto"], &["src/"]).unwrap();
}
