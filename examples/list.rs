#[flux::refined_by(n:int)]
#[flux::invariant(n >= 0)]
pub enum List<T> {
    #[flux::variant(List<T>[0])]
    Nil,
    #[flux::variant((T, Box<List<T>[@n]>) -> List<T>[n+1])]
    Cons(T, Box<List<T>>),
}

impl<T> List<T> {
    #[flux::sig(fn(&List<T>[@n]) -> usize[n])]
    pub fn len(&self) -> usize {
        match self {
            List::Nil => 0,
            List::Cons(_, tl) => 1 + tl.len(),
        }
    }

    #[flux::sig(fn(&List<T>[@n]) -> bool[n == 0])]
    pub fn empty(&self) -> bool {
        match self {
            List::Nil => true,
            List::Cons(_, _) => false,
        }
    }

    #[flux::sig(fn({&List<T>[@n] : 0 < n}) -> &T)]
    pub fn head(&self) -> &T {
        match self {
            List::Nil => unreachable(),
            List::Cons(hd, _) => hd,
        }
    }

    #[flux::sig(fn({&List<T>[@n] : 0 < n}) -> &List<T>)]
    pub fn tail(&self) -> &List<T> {
        match self {
            List::Nil => unreachable(),
            List::Cons(_, tl) => tl,
        }
    }

    #[flux::sig(
        fn(self: &strg List<T>[@n1], List<T>[@n2])
        ensures self: List<T>[n1 + n2]
    )]
    pub fn append(&mut self, other: List<T>) {
        match self {
            List::Nil => *self = other,
            List::Cons(_, tl) => tl.append(other),
        }
    }

    #[flux::sig(fn(&List<T>[@n], idx: usize{idx < n} ) -> &T)]
    pub fn get_nth(&self, idx: usize) -> &T {
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

#[flux::sig(fn() -> T requires false)]
fn unreachable<T>() -> T {
    loop {}
}
