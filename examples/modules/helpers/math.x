module helpers.math;

export square;
export cube;
export factorial;

function square(x: integer) -> integer {
    x * x
}

function cube(x: integer) -> integer {
    x * x * x
}

function factorial(n: integer) -> integer {
    if n <= 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}
