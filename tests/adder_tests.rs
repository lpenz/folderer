// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use folderer::*;

use anyhow::Result;
use derive_more::AddAssign;

/* Test builtin type */

#[test]
fn test_builtin_usize() -> Result<()> {
    let mut sum: Adder<usize> = (1..=5).collect();
    eprintln!("{:?}", sum);
    assert_eq!(*sum, 15);
    sum.extend((6..=10).rev());
    assert_eq!(*sum, 55);
    Ok(())
}

#[test]
fn test_builtin_f32() -> Result<()> {
    let mut sum: Adder<f32> = (1..=5).map(|v| v as f32).collect();
    assert_eq!(*sum, 15_f32);
    sum += 10_f32;
    assert_eq!(*sum, 25_f32);
    sum = 10_f32.into();
    assert_eq!(sum.into_inner(), 10_f32);
    Ok(())
}

/* Test newtype wrapper with default */

#[derive(Default, AddAssign, PartialEq, Eq, Debug)]
struct Usize1(usize);

#[test]
fn test_newtype_with_default() -> Result<()> {
    let mut sum: Adder<Usize1> = (1..=5).map(Usize1).collect();
    assert_eq!(*sum, Usize1(15));
    sum.extend((6..=10).map(Usize1).rev());
    assert_eq!(*sum, Usize1(55));
    Ok(())
}

/* Test newtype wrapper without default: collect doesn't work */

#[derive(AddAssign, Debug)]
struct Usize2(pub usize);

#[test]
fn test_newtype_without_default() -> Result<()> {
    let mut sum = Adder::<Usize2>::from(Usize2(0));
    sum.extend((1..=5).map(Usize2).rev());
    assert_eq!((*sum).0, 15);
    Ok(())
}

/* Test newtype wrapper with a vector of strings,
 * both which don't impl Copy,
 * and impl AddAssign as push */

#[derive(Debug, Default)]
struct Myvec(pub Vec<String>);

impl std::ops::AddAssign<String> for Myvec {
    fn add_assign(&mut self, other: String) {
        self.0.push(other)
    }
}

#[test]
fn test_newtype_vec() -> Result<()> {
    let f = |v| format!("{}", v);
    let mut vec: Adder<Myvec> = (1..=5).map(f).collect();
    vec.extend((6..10).map(f).rev());
    assert_eq!((*vec).0, vec!["1", "2", "3", "4", "5", "9", "8", "7", "6"]);
    assert_eq!(
        vec.into_inner().0,
        vec!["1", "2", "3", "4", "5", "9", "8", "7", "6"]
    );
    Ok(())
}
