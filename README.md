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

## ~~Under the hood~~ [obsolete]

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

## Under the hood v2

It seems Rust community *hates* my use of `unsafe` since I'm extending its meaning beyond what they deem acceptable. I think the pros outweighed the cons, but, luckily, there's a better solution (and possibly others) that avoids the `unsafe` hack and it's even safer:

```rust
pub trait HKT1 {
    type HKTInner1;
    type With<W1>:
        HKT1<HKTInner1 = W1> +
        HKT1<With<Self::HKTInner1> = Self> +
        HKT1<With<W1> = Self::With<W1>>;
}
```

This was suggested by u/Chadshinshin32 in [this comment](https://www.reddit.com/r/rust/comments/10bqmfs/seamless_higherkinded_types_in_rust/j4col9l/) on reddit.

Let's consider a more general case and let's try to show (informally) its correctness:

```rust
pub trait HKT2 {
    type HKTInner1;
    type HKTInner2;
    type With<W1, W2>:
        HKT2<HKTInner1 = W1, HKTInner2 = W2> +
        HKT2<With<Self::HKTInner1, Self::HKTInner2> = Self> +
        HKT2<With<W1, W2> = Self::With<W1, W2>>;
}
```

Let's start with no bounds on `With` and then add them back one at a time, updating our as-general-as-possible implementation so that it doesn't violate them:

```rust
pub trait HKT2 {
    type HKTInner1;
    type HKTInner2;
    type With<W1, W2>:
        //    HKT2<HKTInner1 = W1, HKTInner2 = W2>
        //  + HKT2<With<Self::HKTInner1, Self::HKTInner2> = Self>
        //  + HKT2<With<W1, W2> = Self::With<W1, W2>>
         ;
}

struct A;
struct B;
struct C;

impl<T, U> HKT2 for OK1<T, U> {
    type HKTInner1 = A;
    type HKTInner2 = B;
    type With<W1, W2> = C;
}
```

Now let's add back the first bound:

```rust
pub trait HKT2 {
    type HKTInner1;
    type HKTInner2;
    type With<W1, W2>:
           HKT2<HKTInner1 = W1, HKTInner2 = W2>
        //  + HKT2<With<Self::HKTInner1, Self::HKTInner2> = Self>
        //  + HKT2<With<W1, W2> = Self::With<W1, W2>>
         ;
}

struct A;
struct B;
struct OK1<T, U> { x: (T, U) }
struct OK2<T, U> { x: (T, U) }
struct OK3<T, U> { x: (T, U) }
struct OK4<T, U> { x: (T, U) }

impl<T, U> HKT2 for OK1<T, U> {
    type HKTInner1 = A;
    type HKTInner2 = B;
    type With<W1, W2> = OK2<W1, W2>;
}

impl<T, U> HKT2 for OK2<T, U> {
    type HKTInner1 = T;
    type HKTInner2 = U;
    type With<W1, W2> = OK3<W1, W2>;
}

impl<T, U> HKT2 for OK3<T, U> {
    type HKTInner1 = T;
    type HKTInner2 = U;
    type With<W1, W2> = OK4<W1, W2>;
}

impl<T, U> HKT2 for OK4<T, U> {
    type HKTInner1 = T;
    type HKTInner2 = U;
    type With<W1, W2> = OK3<W1, W2>;
}
```

(In what follows, I'll say `OKx` as short for "the implementation of `HKT2` for `OKx`".)

We can create as long a `WITH`-chain as we want, but we need to "close" it at a certain point. I closed it by making `OK4` refer back to `OK3`. Notice how `OK1` is the only `HKT2` still with arbitrary `HKTInner1` and `HKTInner2`.

Now let's consider the second bound. Let's focus on `OK1`. The bound requires that `OK2<W1, W2>` is an `HKT2<With<A, B> = Self>`, i.e. that `OK2<W1, W2>::With<A, B> = OK1<T, U>`, which implies that `OK2` must refer back to `OK1`, and that `A = T, B = U`:

```rust
pub trait HKT2 {
    type HKTInner1;
    type HKTInner2;
    type With<W1, W2>:
           HKT2<HKTInner1 = W1, HKTInner2 = W2>
         + HKT2<With<Self::HKTInner1, Self::HKTInner2> = Self>
        //  + HKT2<With<W1, W2> = Self::With<W1, W2>>
         ;
}

struct OK1<T, U> { x: (T, U) }
struct OK2<T, U> { x: (T, U) }

impl<T, U> HKT2 for OK1<T, U> {
    type HKTInner1 = T;
    type HKTInner2 = U;
    type With<W1, W2> = OK2<W1, W2>;
}

impl<T, U> HKT2 for OK2<T, U> {
    type HKTInner1 = T;
    type HKTInner2 = U;
    type With<W1, W2> = OK1<W1, W2>;
}
```

Focusing on `OK1`, we can see that the last bound requires that `OK2<W1, W2>::With<W1, W2> = OK2<W1, W2>`, which means that `OK2` must refer to `OK2` instead of `OK1`:

```rust
pub trait HKT2 {
    type HKTInner1;
    type HKTInner2;
    type With<W1, W2>:
           HKT2<HKTInner1 = W1, HKTInner2 = W2>
         + HKT2<With<Self::HKTInner1, Self::HKTInner2> = Self>
         + HKT2<With<W1, W2> = Self::With<W1, W2>>
         ;
}

struct OK1<T, U> { x: (T, U) }
struct OK2<T, U> { x: (T, U) }

impl<T, U> HKT2 for OK1<T, U> {
    type HKTInner1 = T;
    type HKTInner2 = U;
    type With<W1, W2> = OK2<W1, W2>;
}

impl<T, U> HKT2 for OK2<T, U> {
    type HKTInner1 = T;
    type HKTInner2 = U;
    type With<W1, W2> = OK2<W1, W2>;        // changed from OK1 to OK2
}
```

We can't do this, though, as this violates the second bound. Basically, we have:

* (Bound 2) `OK2<W1, W2>::With<T, U> = OK1<T, U>`
* (Bound 3) `OK2<W1, W2>::With<W1, W2> = OK2<W1, W2>`

For all `T` and `U`, by choosing `W1 = T` and `W2 = U`, we get `OK1<T, U> = OK2<T, U>`, that is, `OK1 = OK2`:

```rust
pub trait HKT2 {
    type HKTInner1;
    type HKTInner2;
    type With<W1, W2>:
           HKT2<HKTInner1 = W1, HKTInner2 = W2>
         + HKT2<With<Self::HKTInner1, Self::HKTInner2> = Self>
         + HKT2<With<W1, W2> = Self::With<W1, W2>>
         ;
}

struct OK1<T, U> { x: (T, U) }

impl<T, U> HKT2 for OK1<T, U> {
    type HKTInner1 = T;
    type HKTInner2 = U;
    type With<W1, W2> = OK1<W1, W2>;
}
```

This is the *only allowed* implementation!

---

This is just a POC, so feel free to add *derive macros*, modify names to avoid *name collisions*, and so on... I've just read [The Book](https://doc.rust-lang.org/book/) and been programming in Rust for a couple of days (to code this POC!), so I'm not familiar with Rust ecosystem and its conventions.

That's all! Happy coding!
