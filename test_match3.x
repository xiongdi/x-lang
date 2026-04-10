function test() {
    match (1, 2) {
        (k, v) when k == 1 => {
            let x = (k, v);
        }
        _ => {}
    }
}
