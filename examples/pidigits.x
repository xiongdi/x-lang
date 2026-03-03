// pidigits (Benchmarks Game)
// Compute N digits of Pi using the spigot algorithm with big integers.
// Reference: https://benchmarksgame-team.pages.debian.net/benchmarksgame/description/pidigits.html

function main() {
  let n = 27
  let digits = compute_pi_digits(n)

  let mutable printed = 0
  let mutable line = ""

  while printed < n {
    let remaining = n - printed
    let mutable take = 10
    if take > remaining {
      take = remaining
    }

    line = substring(digits, printed, printed + take)
    printed = printed + take

    // Pad to 10 chars if needed
    let mutable padded = line
    while len(padded) < 10 {
      padded = concat(padded, " ")
    }

    print(concat(padded, concat("\t:", to_string(printed))))
  }
}
