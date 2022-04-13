// Copyright (C) 2022 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

#![warn(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]

//! *folderer* is a single-element folding container that can be used
//! to sum, append, select, etc. values in an ad-hoc fashion.

mod adder;
pub use self::adder::*;

mod dynfolder;
pub use self::dynfolder::*;
