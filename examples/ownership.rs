#[allow(unused)]

#[flux::sig(fn(bool[true]) -> i32)]
pub fn assert(_: bool) -> i32 {
    0
}

// Increment via a value
#[flux::sig(fn(n: i32) -> i32[n+1])]
pub fn inc_val(n: i32) -> i32 {
    n + 1
}

// Increment via a mutable reference
#[flux::sig(fn(x: &mut i32{v: 0 < v}))]
pub fn inc_mut(x: &mut i32) {
    *x += 1;

    // this would be rejected
    // *x -= 1;
}

// Increment via a strong reference
#[flux::sig(fn(x: &strg i32[@n]) ensures x: i32[n+1])]
pub fn inc_strg(x: &mut i32) {
    *x += 1;
}

// Test
fn test() {
    // client of inc_val
    assert(inc_val(10) == 11);

    // client of inc_strg
    let mut p = 10;
    inc_strg(&mut p);
    assert(p == 11);

    inc_mut(&mut p);
    assert(p > 0);
    // this would be rejected
    // assert(p == 11)
}
