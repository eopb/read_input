use std::{
    cmp::PartialOrd,
    ops::{
        Bound::{Excluded, Included, Unbounded},
        Range, RangeBounds, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive,
    },
};use std::rc::Rc;

/// This trait is used to describe constraints with different types.
pub trait IsInFunc<T> {
    /// Returns constraint as a function.
    fn contains_func(self) -> Rc<Fn(&T) -> bool>;
}

impl<T: PartialEq + 'static> IsInFunc<T> for Vec<T> {
    fn contains_func(self) -> Rc<Fn(&T) -> bool> {
        Rc::new(move |x| self.contains(x))
    }
}

impl<T> IsInFunc<T> for [T]
where
    Self: Sized,
    T: PartialEq,
    T: 'static,
    T: Sized,
{
    fn contains_func(self) -> Rc<Fn(&T) -> bool> {
        Rc::new(move |x| self.contains(x))
    }
}

impl<T: 'static, F: Fn(&T) -> bool + 'static> IsInFunc<T> for F {
    fn contains_func(self) -> Rc<Fn(&T) -> bool> {
        Rc::new(self)
    }
}

fn range_contains_func<T, U>(range: U) -> Rc<Fn(&T) -> bool>
where
    T: PartialOrd + 'static,
    U: RangeBounds<T> + 'static,
{
    Rc::new(move |x| {
        (match range.start_bound() {
            Included(ref start) => *start <= x,
            Excluded(ref start) => *start < x,
            Unbounded => true,
        }) && (match range.end_bound() {
            Included(ref end) => x <= *end,
            Excluded(ref end) => x < *end,
            Unbounded => true,
        })
    })
}

macro_rules! impl_default_builder_for_whole {
    ($($t:ty),*) => {$(
        impl<T: PartialOrd + 'static> IsInFunc<T> for $t {
            fn contains_func(self) -> Rc<Fn(&T) -> bool> {
                range_contains_func(self)
            }
        }
    )*}
}

impl_default_builder_for_whole! {
    Range<T>, RangeInclusive<T>, RangeFrom<T>, RangeTo<T>, RangeToInclusive<T>, RangeFull
}
