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
        XY: [(usize, usize); N],
        Q: usize,
    }
    let X = XY.iter().map(|&(x, _)| x).collect::<Vec<_>>();
    let Y = XY.iter().map(|&(_, y)| y).collect::<Vec<_>>();

    let ans = solve(N, Q, X, Y);
    println!("{}", ans.iter().join("\n"));
}

fn solve(N: usize, Q: usize, X: Vec<usize>, Y: Vec<usize>) -> Vec<usize> {
    let mut grid = vec![vec![0; LIMIT]; LIMIT];
    let mut max_x = 0;
    let mut max_y = 0;
    let mut res = vec![];
    for i in 0..N {
        grid[X[i]][Y[i]] += 1;
        max_x = max_x.max(X[i]);
        max_y = max_y.max(Y[i]);
    }
    let mut S = vec![vec![0; max_y + 1]; max_x + 1];
    for x in 1..=max_x {
        for y in 1..=max_y {
            S[x][y] = grid[x][y] + S[x][y - 1] + S[x - 1][y] - S[x - 1][y - 1];
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
