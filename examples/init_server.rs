fn main() {
    let hg = mercury_rs::HG::init("tcp", true)
        .unwrap()
        .create_context()
        .unwrap();
    let address = hg.target_addr().unwrap();
    println!("Server running at address {}", address);

    loop {
        let mut count = 0;
        loop {
            let (trigger_result, trigger_count) = hg.trigger(0, 1).unwrap();
            count = trigger_count;
            if trigger_result == 0 && count == 1 {
                break;
            }
        }
        hg.progress().unwrap();
        if count == 1 {
            break;
        }
    }
}
