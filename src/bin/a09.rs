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
        N: usize,
        ABCD: [(Usize1, Usize1, usize, usize); N],
    }

    let ans = solve(H, W, ABCD);
    println!("{}", ans[1..].iter().map(|row| row[1..].iter().join(" ")).join("\n"));
}

fn solve(H: usize, W: usize, ABCD: Vec<(usize, usize, usize, usize)>) -> Vec<Vec<isize>> {
    // 1-indexed
    let mut A = vec![vec![0; W + 1]; H + 1];
    let mut res = vec![vec![0; W + 1]; H + 1];
    for (a, b, c, d) in ABCD {
        A[a][b] += 1;
        A[c][d] += 1;
        A[a][d] -= 1;
        A[c][b] -= 1;
    }
    for i in 1..=H {
        for j in 1..=W {
            res[i][j] = res[i][j - 1] + A[i - 1][j - 1];
        }
    }
    for j in 1..=W {
        for i in 1..=H {
            res[i][j] = res[i - 1][j] + res[i][j];
        }
    }
    res
}
