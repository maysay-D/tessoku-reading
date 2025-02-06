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

const LIMIT: usize = 1501;

fn main() {
    input! {
        N: usize,
        ABCD: [(usize, usize, usize, usize); N],
    }
    let A = ABCD.iter().map(|&(a, _, _, _)| a).collect::<Vec<_>>();
    let B = ABCD.iter().map(|&(_, b, _, _)| b).collect::<Vec<_>>();
    let C = ABCD.iter().map(|&(_, _, c, _)| c).collect::<Vec<_>>();
    let D = ABCD.iter().map(|&(_, _, _, d)| d).collect::<Vec<_>>();

    let ans = solve(N, A, B, C, D);
    println!("{}", ans);
}

fn solve(N: usize, A: Vec<usize>, B: Vec<usize>, C: Vec<usize>, D: Vec<usize>) -> usize {
    let mut res = 0;
    let mut grid = vec![vec![0; LIMIT]; LIMIT];
    let mut max_x = 0;
    let mut max_y = 0;
    for i in 0..N {
        grid[A[i]][B[i]] += 1;
        grid[C[i]][D[i]] += 1;
        grid[A[i]][D[i]] -= 1;
        grid[C[i]][B[i]] -= 1;
        max_x = max_x.max(C[i] + 1);
        max_y = max_y.max(D[i] + 1);
    }

    let mut S = vec![vec![0; max_y + 1]; max_x + 1];
    for i in 1..=max_x {
        for j in 1..=max_y {
            S[i][j] = S[i - 1][j] + S[i][j - 1] - S[i - 1][j - 1] + grid[i - 1][j - 1];
            if S[i][j] > 0 {
                res += 1;
            }
        }
    }
    res
}
