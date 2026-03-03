// spectral-norm (Benchmarks Game)
// Compute the spectral norm of an infinite matrix A.
// A(i,j) = 1 / ((i+j)*(i+j+1)/2 + i + 1)
// Reference: https://benchmarksgame-team.pages.debian.net/benchmarksgame/description/spectralnorm.html

function A(i, j) {
  return 1.0 / to_float((i + j) * (i + j + 1) / 2 + i + 1)
}

function mul_Av(n, v) {
  let mutable result = new_array(n, 0.0)
  let mutable i = 0
  while i < n {
    let mutable s = 0.0
    let mutable j = 0
    while j < n {
      s = s + A(i, j) * v[j]
      j = j + 1
    }
    result[i] = s
    i = i + 1
  }
  return result
}

function mul_Atv(n, v) {
  let mutable result = new_array(n, 0.0)
  let mutable i = 0
  while i < n {
    let mutable s = 0.0
    let mutable j = 0
    while j < n {
      s = s + A(j, i) * v[j]
      j = j + 1
    }
    result[i] = s
    i = i + 1
  }
  return result
}

function mul_AtAv(n, v) {
  let u = mul_Av(n, v)
  return mul_Atv(n, u)
}

function main() {
  let n = 100
  let mutable u = new_array(n, 1.0)
  let mutable v = new_array(n, 0.0)

  let mutable i = 0
  while i < 10 {
    v = mul_AtAv(n, u)
    u = mul_AtAv(n, v)
    i = i + 1
  }

  let mutable vBv = 0.0
  let mutable vv = 0.0
  let mutable j = 0
  while j < n {
    vBv = vBv + u[j] * v[j]
    vv = vv + v[j] * v[j]
    j = j + 1
  }

  print(format_float(sqrt(vBv / vv), 9))
}
