// binary-trees (Benchmarks Game)
// Allocate, traverse, and deallocate many binary trees.
// Reference: https://benchmarksgame-team.pages.debian.net/benchmarksgame/description/binarytrees.html

function tree(depth) {
  if depth <= 0 {
    return [0, -1, -1]
  }
  return [0, tree(depth - 1), tree(depth - 1)]
}

function check(node) {
  if node[1] == -1 {
    return 1
  }
  return 1 + check(node[1]) + check(node[2])
}

function main() {
  let min_depth = 4
  let n = 10
  let mutable max_depth = min_depth + 2
  if n > max_depth {
    max_depth = n
  }

  let stretch_depth = max_depth + 1
  let stretch_tree = tree(stretch_depth)
  print(concat("stretch tree of depth ", concat(to_string(stretch_depth), concat("\t check: ", to_string(check(stretch_tree))))))

  let long_lived_tree = tree(max_depth)

  let mutable depth = min_depth
  while depth <= max_depth {
    let iterations = pow(2.0, to_float(max_depth - depth + min_depth))
    let iters = to_int(iterations)
    let mutable total_check = 0
    let mutable i = 1
    while i <= iters {
      let t = tree(depth)
      total_check = total_check + check(t)
      i = i + 1
    }
    print(concat(to_string(iters), concat("\t trees of depth ", concat(to_string(depth), concat("\t check: ", to_string(total_check))))))
    depth = depth + 2
  }

  print(concat("long lived tree of depth ", concat(to_string(max_depth), concat("\t check: ", to_string(check(long_lived_tree))))))
}
