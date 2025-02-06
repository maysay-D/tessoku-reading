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
        H: usize,
        W: usize,
        X: [[usize; W]; H],
        Q: usize,
    }

    let ans = solve(H, W, X, Q);
    println!("{}", ans.iter().join("\n"));
}

fn solve(H: usize, W: usize, X: Vec<Vec<usize>>, Q: usize) -> Vec<usize> {
    let mut S = vec![vec![0; W + 1]; H + 1];
    let mut res = vec![];

    for i in 0..H {
        for j in 0..W {
            S[i + 1][j + 1] = S[i + 1][j] + S[i][j + 1] - S[i][j] + X[i][j];
        }
    }

    for _ in 0..Q {
        input! {
            A: Usize1,
            B: Usize1,
            C: usize,
            D: usize,
        }
        res.push(S[C][D] - S[A][D] - S[C][B] + S[A][B]);
    }

    res
}
