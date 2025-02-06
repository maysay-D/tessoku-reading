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
        Q: usize,
    }

    let ans = solve(N, A, Q);
    println!("{}",
             ans.into_iter().map(
                 |x| if x == 0 { "win" } else if x == 1 { "lose" } else { "draw" }
             ).join("\n"));
}

fn solve(N: usize, A: Vec<usize>, Q: usize) -> Vec<usize> {
    let mut res = vec![];
    let mut hit = vec![0; N + 1];
    let mut miss = vec![0; N + 1];

    for i in 1..=N {
        hit[i] = hit[i - 1] + if A[i - 1] == 0 { 0 } else { 1 };
        miss[i] = miss[i - 1] + if A[i - 1] == 0 { 1 } else { 0 };
    }

    for _ in 0..Q {
        input! {
            L: usize,
            R: usize,
        }
        let h = hit[R] - hit[L - 1];
        let m = miss[R] - miss[L - 1];
        if h > m {
            res.push(0);
        } else if h < m {
            res.push(1);
        } else {
            res.push(2);
        }
    }
    res
}
