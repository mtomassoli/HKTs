pub unsafe trait HKT1 {
    type With<W1>;
}

pub unsafe trait HKT2 {
    type With<W1, W2>;
}

pub unsafe trait HKT3 {
    type With<W1, W2, W3>;
}

pub unsafe trait HKT4 {
    type With<W1, W2, W3, W4>;
}

pub unsafe trait HKT5 {
    type With<W1, W2, W3, W4, W5>;
}

#[macro_export]
macro_rules! implHKT1 {
    ($TypeIdent:ident) => {
        unsafe impl<T1> HKT1 for $TypeIdent<T1> {
            type With<S1> = $TypeIdent<S1>;
        }
    };
}

#[macro_export]
macro_rules! implHKT2 {
    ($TypeIdent:ident) => {
        unsafe impl<T1, T2> HKT2 for $TypeIdent<T1, T2> {
            type With<S1, S2> = $TypeIdent<S1, S2>;
        }
    };
}

#[macro_export]
macro_rules! implHKT3 {
    ($TypeIdent:ident) => {
        unsafe impl<T1, T2, T3> HKT3 for $TypeIdent<T1, T2, T3> {
            type With<S1, S2, S3> = $TypeIdent<S1, S2, S3>;
        }
    };
}

#[macro_export]
macro_rules! implHKT4 {
    ($TypeIdent:ident) => {
        unsafe impl<T1, T2, T3, T4> HKT4 for $TypeIdent<T1, T2, T3, T4> {
            type With<S1, S2, S3, S4> = $TypeIdent<S1, S2, S3, S4>;
        }
    };
}

#[macro_export]
macro_rules! implHKT5 {
    ($TypeIdent:ident) => {
        unsafe impl<T1, T2, T3, T4, T5> HKT5 for $TypeIdent<T1, T2, T3, T4, T5> {
            type With<S1, S2, S3, S4, S5> = $TypeIdent<S1, S2, S3, S4, S5>;
        }
    };
}
