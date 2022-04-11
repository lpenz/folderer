// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

#![warn(missing_debug_implementations)]
#![warn(missing_docs)]

use std::iter;
use std::ops;

/// `Adder` type that folds values by using `+=`
///
/// This is a wrapper type that implements `FromIterator` and
/// `Extend`, and folds incoming items by using `+=`, which is
/// implemented by the [`std::ops::AddAssign`] trait.
///
/// Note: to use `FromIterator` (via `collect`) the `Inner` type must
/// implement `Default`.
#[derive(Debug, Default)]
pub struct Adder<Inner>(pub Inner);

impl<Inner> Adder<Inner> {
    /// Returns the inner value of `Adder`, deconstructing it.
    pub fn into_inner(self) -> Inner {
        self.0
    }
}

impl<Inner> From<Inner> for Adder<Inner> {
    fn from(inner: Inner) -> Self {
        Self(inner)
    }
}

impl<Inner> ops::Deref for Adder<Inner> {
    type Target = Inner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<Inner, Item> ops::AddAssign<Item> for Adder<Inner>
where
    Inner: ops::AddAssign<Item>,
{
    fn add_assign(&mut self, other: Item) {
        self.0 += other;
    }
}

impl<Inner, Item> iter::FromIterator<Item> for Adder<Inner>
where
    Inner: Default,
    Inner: ops::AddAssign<Item>,
{
    fn from_iter<It: IntoIterator<Item = Item>>(iter: It) -> Self {
        let mut accum = Adder::<Inner>::default();
        iter.into_iter().for_each(|i| accum.0 += i);
        accum
    }
}

impl<Inner, Item> Extend<Item> for Adder<Inner>
where
    Inner: ops::AddAssign<Item>,
{
    fn extend<It: IntoIterator<Item = Item>>(&mut self, iter: It) {
        iter.into_iter().for_each(|i| self.0 += i);
    }
}
