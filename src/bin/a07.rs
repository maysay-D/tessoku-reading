#![allow(unused_imports)]
#![allow(non_snake_case)]
use ac_library::*;
use once_cell::sync::Lazy;
use static_assertions::*;
use varisat::*;
use memoise::*;
use argio::*;
use bitvec::prelude::*;
use counter::Counter;
use hashbag::*;
use pathfinding::prelude::*;
use recur_fn::*;
use ::indexing::*;
use amplify::*;
use amplify_derive::*;
use amplify_num::*;
use easy_ext::*;
use multimap::*;
use btreemultimap::*;
use bstr::*;
use az::*;
use glidesort::*;
use ::tap::*;
use omniswap::*;
use multiversion::*;
use ::num::*;
use num_bigint::*;
use num_complex::*;
use num_integer::*;
use num_iter::*;
use num_rational::*;
use num_traits::*;
use num_derive::*;
use ndarray::*;
use nalgebra::*;
use alga::*;
use libm::*;
use rand::*;
use getrandom::*;
use rand_chacha::*;
use rand_core::*;
use rand_hc::*;
use rand_pcg::*;
use rand_distr::*;
use petgraph::*;
use indexmap::*;
use regex::*;
use lazy_static::*;
use ordered_float::*;
use ascii::*;
use permutohedron::*;
use superslice::*;
use itertools::*;
use itertools_num::*;
use maplit::*;
use either::*;
use im_rc::*;
use fixedbitset::*;
use bitset_fixed::*;
use proconio::input;
use proconio::marker::*;
use text_io::*;
use rustc_hash::*;
use smallvec::*;
use std::cmp::*;

fn main() {
    input! {
        D: usize,
        N: usize,
        LR: [(usize, usize); N],
    }
    let L = LR.iter().map(|&(l, _)| l).collect::<Vec<_>>();
    let R = LR.iter().map(|&(_, r)| r).collect::<Vec<_>>();

    let ans = solve(D, N, L, R);
    println!("{}", ans[1..].iter().join("\n"));
}

fn solve(D: usize, N: usize, L: Vec<usize>, R: Vec<usize>) -> Vec<isize> {
    // 1-indexed + sentinel node of the right end = D + 2
    let mut B: Vec<isize> = vec![0; D + 2];
    for i in 0..N {
        B[L[i]] += 1;
        B[R[i] + 1] -= 1;
    }
    // 1-indexed = D + 1
    let mut res = vec![0; D + 1];
    for i in 1..=D {
        res[i] = res[i - 1] + B[i];
    }
    res
}
