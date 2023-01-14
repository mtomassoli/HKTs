use super::hkts::*;
use super::*;

trait Mixuppable<T1, U1> : HKT2 {
    fn mixup<T2, U2>(self, other: Self::With<T2, U2>) -> Self::With<T1, U2>;
}

//-----------------------------------------------------------------------------

#[derive(Debug, PartialEq)]
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

#[test]
pub fn test() {
    let p1 = Point { x: 1, y: "a" };
    let p2 = Point { x: 10, y: "b" };
    assert_eq!(p1.mixup(p2), Point { x: 1, y: "b" });
}
