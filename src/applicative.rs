use super::functor::Functor;

// class (Functor f) => Applicative f where
//     pure  :: a -> f a
//     (<*>) :: f (a -> b) -> f a -> f b

pub trait Applicative<A> : Functor<A> {
    fn pure(a: A) -> Self;
    fn apply<B, F: FnMut(A) -> B>(self, ff: Self::With<F>) -> Self::With<B>;
}

//-----------------------------------------------------------------------------

// NOTE: Vec is already a Functor or this wouldn't work.
impl<A> Applicative<A> for Vec<A> {
    fn pure(a: A) -> Self {
        vec![a]
    }

    fn apply<B, F: FnMut(A) -> B>(self, ff: Self::With<F>) -> Self::With<B> {
        ff.into_iter().zip(self.into_iter()).map(
            |(mut f, x)| f(x)
        ).collect()
    }
}

// NOTE: Option is already a Functor or this wouldn't work.
impl<A> Applicative<A> for Option<A> {
    fn pure(a: A) -> Self {
        Some(a)
    }

    fn apply<B, F: FnMut(A) -> B>(self, ff: Self::With<F>) -> Self::With<B> {
        Some(ff?(self?))
    }
}

#[test]
pub fn test() {
    let fs: Vec<Box<dyn Fn(i32) -> i32>> = vec![
        Box::new(|x| x + 1),
        Box::new(|x| x * 2)
    ];
    assert_eq!(vec![1, 5].apply(fs), vec![2, 10]);

    assert_eq!(Some(6).apply(Some(|x| x / 3)), Some(2));
    assert_eq!(None.apply(Some(|x: i32| x / 3)), None);
}
