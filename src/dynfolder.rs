// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

#![warn(missing_debug_implementations)]
#![warn(missing_docs)]

use std::marker;
use std::ops;

/// The `DynFolder` type has a field with the folder function to use.
#[derive(Debug, Default)]
pub struct DynFolder<Inner, Func, Item>(pub Inner, pub Func, marker::PhantomData<Item>);

impl<Inner, Func, Item> DynFolder<Inner, Func, Item> {
    /// Deconstruct self and return the inner value.
    pub fn into_inner(self) -> Inner {
        self.0
    }
    /// Fold value into self
    pub fn fold(&mut self, item: Item)
    where
        Func: Fn(&mut Inner, Item),
    {
        self.1(&mut self.0, item);
    }
}

impl<Inner, Func, Item> From<(Inner, Func)> for DynFolder<Inner, Func, Item>
where
    Func: Fn(&mut Inner, Item),
{
    fn from(innerfunc: (Inner, Func)) -> Self {
        Self(innerfunc.0, innerfunc.1, marker::PhantomData)
    }
}

impl<Inner, Func, Item> From<Func> for DynFolder<Inner, Func, Item>
where
    Inner: Default,
    Func: Fn(&mut Inner, Item),
{
    fn from(func: Func) -> Self {
        Self(Default::default(), func, marker::PhantomData)
    }
}

impl<Inner, Func, Item> ops::Deref for DynFolder<Inner, Func, Item> {
    type Target = Inner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<Inner, Func, Item> Extend<Item> for DynFolder<Inner, Func, Item>
where
    Func: Fn(&mut Inner, Item),
{
    fn extend<It: IntoIterator<Item = Item>>(&mut self, iter: It) {
        iter.into_iter().for_each(|i| self.fold(i));
    }
}
