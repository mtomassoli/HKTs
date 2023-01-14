use super::applicative::Applicative;

// class (Applicative m) => Monad m where
//     return :: a -> m a
//     (>>=) :: m a -> (a -> m b) -> m b

pub trait Monad<A> : Applicative<A> {
    fn ret(a: A) -> Self;
    fn flatmap<B, F: FnMut(A) -> Self::With<B>>(self, f: F) -> Self::With<B>;
}

//-----------------------------------------------------------------------------

// NOTE: Vec is already an Applicative or this wouldn't work.
impl<A> Monad<A> for Vec<A> {
    fn ret(a: A) -> Self {
        vec![a]
    }

    fn flatmap<B, F: FnMut(A) -> Self::With<B>>(self, mut f: F) -> Self::With<B> {
        self.into_iter().flat_map(|x| f(x).into_iter()).collect()
    }
}

// NOTE: Option is already an Applicative or this wouldn't work.
impl<A> Monad<A> for Option<A> {
    fn ret(a: A) -> Self {
        Some(a)
    }

    fn flatmap<B, F: FnMut(A) -> Self::With<B>>(self, mut f: F) -> Self::With<B> {
        f(self?)
    }
}

#[test]
pub fn test() {
    // shortcut
    fn ret<T>(e: T) -> Option<T> {
        Option::ret(e)
    }

    assert_eq!(
        vec![1, 2, 3].flatmap(|x| vec![x, x*x, x*x*x]),
        vec![1, 1, 1, 2, 4, 8, 3, 9, 27]
    );

    // pretend flatmap = "do"
    let computation =
        ret(String::from("longer_than_10"))
        .flatmap(|s| ret(s.len()))
        .flatmap(|len| {
            if len > 10 { ret("ok") } else { None }
        }).flatmap(|res| ret(res == "ok"));

    assert_eq!(computation, ret(true));
}
