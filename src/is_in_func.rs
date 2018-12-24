use std::{
    cmp::PartialOrd,
    ops::{Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive},
};

pub trait IsInFunc<T> {
    fn contains_func(self) -> Box<Fn(&T) -> bool>;
}

impl<T: PartialOrd + 'static> IsInFunc<T> for Range<T> {
    fn contains_func(self) -> Box<Fn(&T) -> bool> {
        Box::new(move |x| &self.start <= x && x < &self.end)
    }
}

impl<T: PartialOrd + 'static> IsInFunc<T> for RangeInclusive<T> {
    fn contains_func(self) -> Box<Fn(&T) -> bool> {
        Box::new(move |x| self.start() <= x && x <= self.end())
    }
}

impl<T: PartialOrd + 'static> IsInFunc<T> for RangeFrom<T> {
    fn contains_func(self) -> Box<Fn(&T) -> bool> {
        Box::new(move |x| &self.start <= x)
    }
}

impl<T: PartialOrd + 'static> IsInFunc<T> for RangeTo<T> {
    fn contains_func(self) -> Box<Fn(&T) -> bool> {
        Box::new(move |x| &self.end > x)
    }
}

impl<T: PartialOrd + 'static> IsInFunc<T> for RangeToInclusive<T> {
    fn contains_func(self) -> Box<Fn(&T) -> bool> {
        Box::new(move |x| &self.end >= x)
    }
}

impl<T: PartialOrd + 'static> IsInFunc<T> for RangeFull {
    fn contains_func(self) -> Box<Fn(&T) -> bool> {
        Box::new(|_| true)
    }
}

impl<T: PartialEq + 'static> IsInFunc<T> for Vec<T> {
    fn contains_func(self) -> Box<Fn(&T) -> bool> {
        Box::new(move |x| self.contains(x))
    }
}

impl<T> IsInFunc<T> for [T]
where
    Self: Sized,
    T: PartialEq,
    T: 'static,
    T: Sized,
{
    fn contains_func(self) -> Box<Fn(&T) -> bool> {
        Box::new(move |x| self.contains(x))
    }
}

impl<T: 'static, F: Fn(&T) -> bool + 'static> IsInFunc<T> for F {
    fn contains_func(self) -> Box<Fn(&T) -> bool> {
        Box::new(self)
    }
}
