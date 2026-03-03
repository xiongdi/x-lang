// fannkuch-redux (Benchmarks Game)
// Pancake flipping on all permutations of [1..n].
// Reference: https://benchmarksgame-team.pages.debian.net/benchmarksgame/description/fannkuchredux.html

function main() {
  let n = 7
  let mutable perm = new_array(n, 0)
  let mutable count = new_array(n, 0)
  let mutable perm1 = new_array(n, 0)

  let mutable i = 0
  while i < n {
    perm1[i] = i
    i = i + 1
  }

  let mutable checksum = 0
  let mutable max_flips = 0
  let mutable perm_count = 0

  let mutable r = n
  while true {
    while r > 1 {
      count[r - 1] = r
      r = r - 1
    }

    // Copy perm1 to perm
    let mutable k = 0
    while k < n {
      perm[k] = perm1[k]
      k = k + 1
    }

    // Count flips
    let mutable flips = 0
    let mutable first = perm[0]
    while first != 0 {
      // Reverse perm[0..first]
      let mutable lo = 0
      let mutable hi = first
      while lo < hi {
        let tmp = perm[lo]
        perm[lo] = perm[hi]
        perm[hi] = tmp
        lo = lo + 1
        hi = hi - 1
      }
      flips = flips + 1
      first = perm[0]
    }

    if flips > max_flips {
      max_flips = flips
    }

    if perm_count % 2 == 0 {
      checksum = checksum + flips
    } else {
      checksum = checksum - flips
    }

    perm_count = perm_count + 1

    // Next permutation (rotate perm1)
    let mutable done = false
    r = 1
    while r < n and not done {
      let perm0 = perm1[0]
      let mutable ii = 0
      while ii < r {
        perm1[ii] = perm1[ii + 1]
        ii = ii + 1
      }
      perm1[r] = perm0

      count[r] = count[r] - 1
      if count[r] > 0 {
        done = true
      } else {
        r = r + 1
      }
    }

    if r >= n {
      // All permutations done
      print(checksum)
      print(concat("Pfannkuchen(", concat(to_string(n), concat(") = ", to_string(max_flips)))))
      return 0
    }
  }
}
