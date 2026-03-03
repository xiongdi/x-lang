// 快速排序 - X 语言版本
// 用于测试 C23 后端流水线

// 交换两个整数
function swap(a, b) {
  let temp = *a
  *a = *b
  *b = temp
}

// 分区函数
function partition(arr, low, high) {
  let pivot = arr[high]
  let i = low - 1
  let j = low

  while j < high {
    if arr[j] <= pivot {
      i = i + 1
      swap(&arr[i], &arr[j])
    }
    j = j + 1
  }
  swap(&arr[i + 1], &arr[high])
  return i + 1
}

// 快速排序
function quicksort(arr, low, high) {
  if low < high {
    let pi = partition(arr, low, high)
    quicksort(arr, low, pi - 1)
    quicksort(arr, pi + 1, high)
  }
}

// 打印数组
function print_array(arr, size) {
  print("[")
  let i = 0
  while i < size {
    if i > 0 {
      print(", ")
    }
    print(arr[i])
    i = i + 1
  }
  print("]\n")
}

function main() {
  print("========================================\n")
  print("  X-Lang - 快速排序测试\n")
  print("========================================\n\n")

  print("【测试 1】快速排序\n")
  print("----------------------------------------\n")

  let arr1[10]
  let i = 0
  while i < 10 {
    arr1[i] = 10 - i
    i = i + 1
  }

  print("排序前: ")
  print_array(arr1, 10)

  quicksort(arr1, 0, 9)

  print("排序后: ")
  print_array(arr1, 10)

  print("\n【测试 2】随机数组排序\n")
  print("----------------------------------------\n")

  let test_arr[8]
  test_arr[0] = 64
  test_arr[1] = 34
  test_arr[2] = 25
  test_arr[3] = 12
  test_arr[4] = 22
  test_arr[5] = 11
  test_arr[6] = 90
  test_arr[7] = 2

  print("排序前: ")
  print_array(test_arr, 8)

  quicksort(test_arr, 0, 7)

  print("排序后: ")
  print_array(test_arr, 8)

  print("\n✓ 所有测试完成！\n")

  return 0
}
