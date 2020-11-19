#![allow(unused_imports, unused_macros, dead_code)]

use cargo_snippet::snippet;
use std::cmp::{max, min, Ordering};
use std::collections::*;
use std::{i32, i64, i128, u32, u64, u128, f64};
use std::io::{stdout, Write, BufWriter};
use std::mem::{swap};
use maplit::*;
use proconio::{input, marker::{Isize1, Usize1, Bytes, Chars}};
use itertools::{Position, Itertools};
use itertools_num::ItertoolsNum;
use smallvec::SmallVec;

mod int;
mod macros;
mod str;
mod extslice;
mod union_find;
