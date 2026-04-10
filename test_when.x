function get(self: List<T>, index: Int) -> Option<T> {
    if index < 0 or index >= self.len() {
        None
    } else {
        Some(self.data[index])
    }
}
