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
        N: usize,
        A: [usize; N],
        D: usize,
        LR: [(usize, usize); D],
    }
    let L = LR.iter().map(|&(l, _)| l).collect::<Vec<_>>();
    let R = LR.iter().map(|&(_, r)| r).collect::<Vec<_>>();

    let ans = solve(N, A, D, L, R);
    println!("{}", ans.iter().join("\n"));
}

fn solve(N: usize, A: Vec<usize>, D: usize, L: Vec<usize>, R: Vec<usize>) -> Vec<usize> {
    let mut res = vec![];
    let mut LS = vec![A[0]; N + 1];
    let mut RS = vec![A[N - 1]; N + 1];

    for i in 1..=N {
        LS[i] = std::cmp::max(LS[i - 1], A[i - 1]);
    }
    for i in (1..N).rev() {
        RS[i] = std::cmp::max(RS[i + 1], A[i - 1]);
    }

    for d in 0..D {
        let l = L[d];
        let r = R[d];
        res.push(std::cmp::max(LS[l - 1], RS[r + 1]));
    }
    res
}
