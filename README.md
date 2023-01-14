# Seamless Higher-Kinded Types in Rust

This is actual working code:

```rust
pub trait Functor<A> : HKT1 {
    fn map<B, F: FnMut(A) -> B>(self, f: F) -> Self::With<B>;
}

pub trait Applicative<A> : Functor<A> {
    fn pure(a: A) -> Self;
    fn apply<B, F: FnMut(A) -> B>(self, ff: Self::With<F>) -> Self::With<B>;
}

pub trait Monad<A> : Applicative<A> {
    fn ret(a: A) -> Self;
    fn flatmap<B, F: FnMut(A) -> Self::With<B>>(self, f: F) -> Self::With<B>;
}
```

Here are some implementations for `Vec`:

```rust
implHKT1!(Vec);         // the magic part!

impl<A> Functor<A> for Vec<A> {
    fn map<B, F: FnMut(A) -> B>(self, f: F) -> Self::With<B> {
        self.into_iter().map(f).collect()
    }
}

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

impl<A> Monad<A> for Vec<A> {
    fn ret(a: A) -> Self {
        vec![a]
    }

    fn flatmap<B, F: FnMut(A) -> Self::With<B>>(self, mut f: F) -> Self::With<B> {
        self.into_iter().flat_map(|x| f(x).into_iter()).collect()
    }
}
```

Let's try our `flatmap`:


```rust
assert_eq!(
    vec![1, 2, 3].flatmap(|x| vec![x, x*x, x*x*x]),
    vec![1, 1, 1, 2, 4, 8, 3, 9, 27]
);
```

Here's an example with 2 type parameters:

```rust
trait Mixuppable<T1, U1> : HKT2 {
    fn mixup<T2, U2>(self, other: Self::With<T2, U2>) -> Self::With<T1, U2>;
}

struct Point<T, U> {
    x: T,
    y: U
}

struct _NotPoint<T, U> {
    a: T,
    b: U
}

implHKT2!(Point);

impl<T1, U1> Mixuppable<T1, U1> for Point<T1, U1> {
    // // ERROR: expected struct `Point`, found struct `_NotPoint`
    // fn mixup<T2, U2>(self, other: Point<T2, U2>) -> _NotPoint<T1, U2> {
    //     todo!()
    // }
    
    fn mixup<T2, U2>(self, other: Point<T2, U2>) -> Point<T1, U2> {
        Point { x: self.x, y: other.y }
    }
}
```

*Can we still claim that Rust doesn't support HKTs, in practice?*

## Under the hood

The implementation of `implHKT1` is trivial:

```rust
pub unsafe trait HKT1 {
    type With<W1>;
}

#[macro_export]
macro_rules! implHKT1 {
    ($TypeIdent:ident) => {
        unsafe impl<T1> HKT1 for $TypeIdent<T1> {
            type With<S1> = $TypeIdent<S1>;
        }
    };
}
```

Notice the use of the *unsafe* keyword.

The *unsafe* keyword indicates that `HKT1` has invariants that the compiler can't verify, which is exactly what's happening here!

If users try to implement `HKT1` on their own, they will get an error. Then it's their responsibility to do the right thing and use the macro instead of just marking their implementation as *unsafe*.

A little detail: to avoid redefinitions, I call the macros for external types (e.g. `Vec` and `Option`) once and for all in [ext_hkts_impls.rs](src/ext_hkts_impls.rs).

This is just a POC, so feel free to add *derive macros*, modify names to avoid *name collisions*, and so on... I've just read [The Book](https://doc.rust-lang.org/book/) and been programming in Rust for a couple of days (to code this POC!), so I'm not familiar with Rust ecosystem and its conventions.

That's all! Happy coding!
