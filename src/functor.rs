use super::hkts::*;

// class Functor f where
//     fmap :: (a -> b) -> f a -> f b

pub trait Functor<A> : HKT1 {
    fn map<B, F: FnMut(A) -> B>(self, f: F) -> Self::With<B>;
}

//-----------------------------------------------------------------------------

impl<A> Functor<A> for Vec<A> {
    fn map<B, F: FnMut(A) -> B>(self, f: F) -> Self::With<B> {
        self.into_iter().map(f).collect()
    }
}

impl<A> Functor<A> for Option<A> {
    fn map<B, F: FnMut(A) -> B>(self, mut f: F) -> Self::With<B> {
        Some(f(self?))
    }
}

#[test]
pub fn test() {
    assert_eq!(vec![1, 2, 3].map(|x| x + 1), vec![2, 3, 4]);
    assert_eq!(Some(4).map(|x| x / 2), Some(2));
}
