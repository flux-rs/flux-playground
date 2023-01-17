#[flux::sig(fn(bool[true]) -> i32)]
fn assert(b: bool) -> i32 {
    0
}

#[flux::sig(fn(x: i32) -> i32[x + 1])]
fn incr(x: i32) -> i32 {
    x + 1
}

pub fn test() {
    assert(incr(1) <= 2); // ok
    assert(incr(2) <= 2); // fail
}
