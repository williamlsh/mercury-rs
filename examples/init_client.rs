fn main() {
    let hg = mercury_rs::HG::init("tcp", false)
        .unwrap()
        .create_context()
        .unwrap();
        hg.destroy_context();
        hg.finalize();
}
