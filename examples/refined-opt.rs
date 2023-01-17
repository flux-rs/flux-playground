#[allow(unused)]

#[flux::refined_by(b:bool)]
pub enum Opt<T> {
    #[flux::variant(Opt<T>[false])]
    None,
    #[flux::variant((T) -> Opt<T>[true])]
    Some(T),
}

impl<T> Opt<T> {
    #[flux::sig(fn(&Opt<T>[@b]) -> bool[b])]
    pub fn is_some(&self) -> bool {
        match self {
            Opt::None => false,
            Opt::Some(_) => true,
        }
    }

    #[flux::sig(fn(Opt<T>[true]) -> T)]
    pub fn unwrap(self) -> T {
        match self {
            Opt::Some(v) => v,
            Opt::None => unreachable(),
        }
    }
}

fn test(opt: Opt<i32>) -> i32 {
    if opt.is_some() {
        opt.unwrap()
    } else {
        0
    }
}

#[flux::sig(fn() -> T requires false)]
fn unreachable<T>() -> T {
    loop {}
}
