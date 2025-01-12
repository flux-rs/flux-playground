#![allow(unused)]

#[flux_rs::refined_by(b:bool)]
enum Opt<T> {
    #[flux_rs::variant(Opt<T>[false])]
    None,
    #[flux_rs::variant((T) -> Opt<T>[true])]
    Some(T),
}

impl<T> Opt<T> {
    #[flux_rs::sig(fn(&Opt<T>[@b]) -> bool[b])]
    fn is_some(&self) -> bool {
        match self {
            Opt::None => false,
            Opt::Some(_) => true,
        }
    }

    #[flux_rs::sig(fn(Opt<T>[true]) -> T)]
    fn unwrap(self) -> T {
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

#[flux_rs::sig(fn() -> T requires false)]
fn unreachable<T>() -> T {
    loop {}
}
