use itertools::traits::HomogeneousTuple;

macro_rules! first {
    ($first:tt, $_:tt) => {
        $first
    };
}

pub trait Tuple {
    const LEN: usize;

    // TODO: remove when inlining `EquisizedCopyArray`
    type EquisizedHomogenous<T>: HomogeneousTuple<Item = T>;
    // TODO: inline to `[T; Self::LEN]` when `const_generics` gets stabilised
    type EquisizedCopyArray<T: Copy>: Copy + AsMut<[T]> + From<Self::EquisizedHomogenous<T>>;
}

macro_rules! impl_tuple {
    ($($generic:ident),*; $len:literal) => {
        impl<$($generic),*> Tuple for ($($generic,)*) {
            const LEN: usize = $len;

            type EquisizedHomogenous<T> = ($(first!(T, $generic),)*);
            type EquisizedCopyArray<T: Copy> = [T; $len];
        }
    };
}

impl_tuple!(A; 1);
impl_tuple!(A, B; 2);
impl_tuple!(A, B, C; 3);
impl_tuple!(A, B, C, D; 4);
