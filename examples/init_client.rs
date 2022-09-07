fn main() {
    let _hg = mercury_rs::HG::init("tcp", false)
        .unwrap()
        .create_context()
        .unwrap();
}
