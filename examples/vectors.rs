mod rvec;
use rvec::RVec;

// An `assert` function, whose precondition expects only `true`
#[flux_rs::sig(fn(bool[true]))]
pub fn assert(_: bool) {}

#[flux_rs::sig(fn() -> usize{v: 10 <= v})]
fn test_push_pop() {
    let mut v = RVec::new();
    v.push(10);
    v.push(20);
    v.pop();
    v.pop();
    v.pop();
}

fn test_macro_pop() {
    let mut v = rvec![10, 20];
    v.pop();
    v.pop();
    v.pop();
}

fn test_push_pop_len() {
    let mut v = rvec![10, 20, 30];
    v.push(40);
    v.push(50);
    assert(v.len() == 5);
    v.pop();
    assert(v.len() == 40);
}

pub fn vec_sum(vec: &RVec<i32>) -> i32 {
    let mut i = 0;
    let mut res = 0;
    while i < vec.len() {
        res += vec[i];
        i += 1;
    }
    res
}

pub fn fib(n: usize) -> i32 {
    let mut r = RVec::new();
    let mut i = 0;
    while i < n {
        if i == 0 {
            r.push(0);
        } else if i == 1 {
            r.push(1);
        } else {
            let a = r[i - 1];
            let b = r[i - 2];
            r.push(a + b);
        }
        i += 1;
    }
    r.pop()
}

pub fn binary_search(vec: &RVec<i32>, x: i32) -> Result<usize, usize> {
    let mut size = vec.len();
    let mut left = 0;
    let mut right = size;
    while left <= right {
        let mid = left + size / 2;
        let val = vec[mid];
        if val < x {
            left = mid + 1;
        } else if x < val {
            right = mid;
        } else {
            return Ok(mid);
        }
        size = right - left;
    }
    Err(left)
}
