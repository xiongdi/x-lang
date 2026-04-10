module test

export function contains(value: Int) -> Bool {
    let data = [1, 2, 3]
    for i in 0..3 {
        if value == data[i] {
            return true
        }
    }
    false
}
