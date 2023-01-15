// NOTEs:
// - We could try using a single tuple-like HKTInner, but it doesn't seem worth it.
// - Should we try to hide the HKTInnerN types since they're just used internally?

pub trait HKT1 {
    type HKTInner1;
    type With<W1>:
        HKT1<HKTInner1 = W1> +
        HKT1<With<Self::HKTInner1> = Self> +
        HKT1<With<W1> = Self::With<W1>>;
}

pub trait HKT2 {
    type HKTInner1;
    type HKTInner2;
    type With<W1, W2>:
        HKT2<HKTInner1 = W1, HKTInner2 = W2> +
        HKT2<With<Self::HKTInner1, Self::HKTInner2> = Self> +
        HKT2<With<W1, W2> = Self::With<W1, W2>>;
}

pub trait HKT3 {
    type HKTInner1;
    type HKTInner2;
    type HKTInner3;
    type With<W1, W2, W3>:
        HKT3<HKTInner1 = W1, HKTInner2 = W2, HKTInner3 = W3> +
        HKT3<With<Self::HKTInner1, Self::HKTInner2, Self::HKTInner3> = Self> +
        HKT3<With<W1, W2, W3> = Self::With<W1, W2, W3>>;
}

pub trait HKT4 {
    type HKTInner1;
    type HKTInner2;
    type HKTInner3;
    type HKTInner4;
    type With<W1, W2, W3, W4>:
        HKT4<HKTInner1 = W1, HKTInner2 = W2, HKTInner3 = W3, HKTInner4 = W4> +
        HKT4<With<Self::HKTInner1, Self::HKTInner2, Self::HKTInner3, Self::HKTInner4> = Self> +
        HKT4<With<W1, W2, W3, W4> = Self::With<W1, W2, W3, W4>>;
}

pub trait HKT5 {
    type HKTInner1;
    type HKTInner2;
    type HKTInner3;
    type HKTInner4;
    type HKTInner5;
    type With<W1, W2, W3, W4, W5>:
        HKT5<HKTInner1 = W1, HKTInner2 = W2, HKTInner3 = W3, HKTInner4 = W4, HKTInner5 = W5> +
        HKT5<With<Self::HKTInner1, Self::HKTInner2, Self::HKTInner3, Self::HKTInner4, Self::HKTInner5> = Self> +
        HKT5<With<W1, W2, W3, W4, W5> = Self::With<W1, W2, W3, W4, W5>>;
}


#[macro_export]
macro_rules! implHKT1 {
    ($TypeIdent:ident) => {
        impl<T1> HKT1 for $TypeIdent<T1> {
            type HKTInner1 = T1;
            type With<S1> = $TypeIdent<S1>;
        }
    };
}

#[macro_export]
macro_rules! implHKT2 {
    ($TypeIdent:ident) => {
        impl<T1, T2> HKT2 for $TypeIdent<T1, T2> {
            type HKTInner1 = T1;
            type HKTInner2 = T2;
            type With<S1, S2> = $TypeIdent<S1, S2>;
        }
    };
}

#[macro_export]
macro_rules! implHKT3 {
    ($TypeIdent:ident) => {
        impl<T1, T2, T3> HKT3 for $TypeIdent<T1, T2, T3> {
            type HKTInner1 = T1;
            type HKTInner2 = T2;
            type HKTInner3 = T3;
            type With<S1, S2, S3> = $TypeIdent<S1, S2, S3>;
        }
    };
}

#[macro_export]
macro_rules! implHKT4 {
    ($TypeIdent:ident) => {
        unsafe impl<T1, T2, T3, T4> HKT4 for $TypeIdent<T1, T2, T3, T4> {
            type HKTInner1 = T1;
            type HKTInner2 = T2;
            type HKTInner3 = T3;
            type HKTInner4 = T4;
            type With<S1, S2, S3, S4> = $TypeIdent<S1, S2, S3, S4>;
        }
    };
}

#[macro_export]
macro_rules! implHKT5 {
    ($TypeIdent:ident) => {
        unsafe impl<T1, T2, T3, T4, T5> HKT5 for $TypeIdent<T1, T2, T3, T4, T5> {
            type HKTInner1 = T1;
            type HKTInner2 = T2;
            type HKTInner3 = T3;
            type HKTInner4 = T4;
            type HKTInner5 = T5;
            type With<S1, S2, S3, S4, S5> = $TypeIdent<S1, S2, S3, S4, S5>;
        }
    };
}
