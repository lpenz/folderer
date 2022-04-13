// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use folderer::*;

use anyhow::Result;

/* Test builtin type */

#[test]
fn test_builtin_sum_usize() -> Result<()> {
    let mut sum =
        DynFolder::<usize, _, u16>::from((0_usize, |a: &mut usize, i: u16| *a += i as usize));
    sum.fold(10);
    assert_eq!(*sum, 10);
    sum.extend((1..=5).rev());
    assert_eq!(*sum, 25);
    Ok(())
}

/* Test newtype wrapper */
#[derive(Default, PartialEq, Eq, Debug)]
struct Usize1(usize);

#[test]
fn test_newtype_with_default() -> Result<()> {
    let f = |a: &mut Usize1, i| (*a).0 += i;
    let mut sum = DynFolder::<Usize1, _, usize>::from(f);
    sum.extend(1..=5);
    assert_eq!(*sum, Usize1(15));
    sum.extend((6..=10).rev());
    assert_eq!(*sum, Usize1(55));
    Ok(())
}

/* Test newtype wrapper without default */
#[derive(Debug)]
struct Usize2(pub usize);

#[test]
fn test_newtype_without_default() -> Result<()> {
    let f = |a: &mut Usize2, i| (*a).0 += i;
    let mut sum = DynFolder::<Usize2, _, usize>::from((Usize2(0), f));
    sum.extend((1..=5).rev());
    assert_eq!((*sum).0, 15);
    Ok(())
}

/* Test vector of Strings, neither impl Copy */

fn folder(inner: &mut Vec<String>, item: String) {
    inner.push(item);
}

#[test]
fn test_newtype_vec() -> Result<()> {
    let mut autofolder = DynFolder::<Vec<String>, _, String>::from(folder);
    let f = |v| format!("{}", v);
    autofolder.extend((6..10).map(f).rev());
    assert_eq!((*autofolder), vec!["9", "8", "7", "6"]);
    assert_eq!(autofolder.into_inner(), vec!["9", "8", "7", "6"]);
    Ok(())
}
