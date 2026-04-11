// 测试管道运算符 |>
let numbers = [1, 2, 3, 4, 5]
let result = numbers
    |> map(x -> x * 2)
    |> filter(x -> x > 3)
    |> sum
println(result)