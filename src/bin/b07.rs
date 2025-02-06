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

fn main() {
    input! {
        T: usize,
        N: usize,
        LR: [(usize, usize); N],
    }
    let L = LR.iter().map(|&(l, _)| l).collect::<Vec<_>>();
    let R = LR.iter().map(|&(_, r)| r).collect::<Vec<_>>();

    let ans = solve(T, N, L, R);
    println!("{}", ans[..T].iter().join("\n"));
}

fn solve(T: usize, N: usize, L: Vec<usize>, R: Vec<usize>) -> Vec<usize> {
    let mut res: Vec<isize> = vec![0; T + 1];
    for i in 0..N {
        res[L[i]] += 1;
        res[R[i]] -= 1;
    }
    for i in 1..=T {
        res[i] += res[i - 1];
    }
    let res = res.into_iter().map(|x| x as usize).collect();
    res
}
