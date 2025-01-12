#![allow(unused)]

#[flux_rs::refined_by(n:int)]
#[flux_rs::invariant(n >= 0)]
enum List<T> {
    #[flux_rs::variant(List<T>[0])]
    Nil,
    #[flux_rs::variant((T, Box<List<T>[@n]>) -> List<T>[n+1])]
    Cons(T, Box<List<T>>),
}

impl<T> List<T> {
    #[flux_rs::sig(fn(&List<T>[@n]) -> usize[n])]
    fn len(&self) -> usize {
        match self {
            List::Nil => 0,
            List::Cons(_, tl) => 1 + tl.len(),
        }
    }

    #[flux_rs::sig(fn(&List<T>[@n]) -> bool[n == 0])]
    fn empty(&self) -> bool {
        match self {
            List::Nil => true,
            List::Cons(_, _) => false,
        }
    }

    #[flux_rs::sig(fn({&List<T>[@n] | 0 < n}) -> &T)]
    fn head(&self) -> &T {
        match self {
            List::Nil => unreachable(),
            List::Cons(hd, _) => hd,
        }
    }

    #[flux_rs::sig(fn({&List<T>[@n] | 0 < n}) -> &List<T>)]
    fn tail(&self) -> &List<T> {
        match self {
            List::Nil => unreachable(),
            List::Cons(_, tl) => tl,
        }
    }

    #[flux_rs::sig(
        fn(self: &strg List<T>[@n1], List<T>[@n2])
        ensures self: List<T>[n1 + n2]
    )]
    fn append(&mut self, other: List<T>) {
        match self {
            List::Nil => *self = other,
            List::Cons(_, tl) => tl.append(other),
        }
    }

    #[flux_rs::sig(fn(&List<T>[@n], idx: usize{idx < n} ) -> &T)]
    fn get_nth(&self, idx: usize) -> &T {
        match self {
            List::Cons(hd, tl) => {
                if idx == 0 {
                    hd
                } else {
                    tl.get_nth(idx - 1)
                }
            }
            List::Nil => unreachable(),
        }
    }
}

#[flux_rs::sig(fn() -> T requires false)]
fn unreachable<T>() -> T {
    loop {}
}
