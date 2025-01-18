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

const MOD: usize = 1_000_000_007;

fn main() {
    input! {
        H: usize,
        W: usize,
    }

    let ans = solve(H, W);
    println!("{}", ans);
}

fn solve(H: usize, W: usize) -> usize {
    let n_perm = (1..=H + W - 2).fold(1, |acc, x| acc * x % MOD);
    let r_perm = (1..=W - 1).fold(1, |acc, x| acc * x % MOD);
    let n_r_perm = (1..=H - 1).fold(1, |acc, x| acc * x % MOD);
    n_perm * power(r_perm * n_r_perm, MOD - 2) % MOD
}

fn power(a: usize, b: usize) -> usize {
    let mut result = 1;
    let mut a = a;
    let mut exp = 0;
    let mut count = 0;
    while exp < b {
        exp = 1 << count;
        a %= MOD;
        if (b / exp) % 2 == 1 {
            result = (result * a) % MOD;
        }
        a *= a;
        count += 1;
    }
    result
}
