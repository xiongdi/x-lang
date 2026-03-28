// X 语言标准库 - Prelude（自动导入）

// 外部函数声明 - 使用 puts 自动换行
extern function puts(message: String) -> Int

// println 函数 - 打印字符串并换行
function println(message: String) -> Unit {
    puts(message)
}
