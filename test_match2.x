function test(entries: List<(Int, Int)>) {
    match (1, 2) {
        (k, v) when k == 1 => {
            entries.insert(0, (k, v));
        }
        _ => {}
    }
}
