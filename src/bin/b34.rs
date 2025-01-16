#![allow(unused_imports)]
#![allow(unused_variables)]
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
        X: usize,
        Y: usize,
        A: [usize; N],
    }

    let ans = solve(N, X, Y, A);
    println!("{}", if ans {"First"} else {"Second"});
}

fn solve(N: usize, X: usize, Y: usize, A: Vec<usize>) -> bool {
    let mut result = if A[0] % 5 == 0 || A[0] % 5 == 1 {
        0
    } else if A[0] % 5 == 2 || A[0] % 5 == 3 {
        1
    } else {
        2
    };

    for i in 1..N {
        if A[i] % 5 == 0 || A[i] % 5 == 1 {
            result ^= 0
        } else if A[i] % 5 == 2 || A[i] % 5 == 3 {
            result ^= 1
        } else {
            result ^= 2
        }
    }

    result != 0
}
