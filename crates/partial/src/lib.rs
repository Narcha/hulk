pub trait Partial {
    type Partial;

    fn apply_partial(&mut self, partial: Self::Partial);
}

pub type PartialOf<T> = <T as Partial>::Partial;

macro_rules! impl_partial_as_identity {
    ($ty: ty) => {
        impl Partial for $ty {
            type Partial = Self;

            fn apply_partial(&mut self, partial: Self::Partial) {
                *self = partial;
            }
        }
    };
}

impl_partial_as_identity!(i8);
impl_partial_as_identity!(u8);
impl_partial_as_identity!(i16);
impl_partial_as_identity!(u16);
impl_partial_as_identity!(i32);
impl_partial_as_identity!(u32);
impl_partial_as_identity!(i64);
impl_partial_as_identity!(u64);
impl_partial_as_identity!(isize);
impl_partial_as_identity!(usize);
impl_partial_as_identity!(f32);
impl_partial_as_identity!(f64);
impl_partial_as_identity!(String);
