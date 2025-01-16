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
    let mut grundies = vec![0; A.clone().into_iter().max().unwrap() + 1];

    for i in 0..=A.clone().into_iter().max().unwrap() {
        let mut trans = vec![false; 3];
        if i.checked_sub(X).is_some() {
            trans[grundies[i - X]] = true;
        }
        if i.checked_sub(Y).is_some() {
            trans[grundies[i - Y]] = true;
        }
        if trans[0] == false {
            grundies[i] = 0
        } else if trans[1] == false {
            grundies[i] = 1
        } else {
            grundies[i] = 2
        }
    }

    let mut result = grundies[A[0]];
    for i in 1..N {
        result ^= grundies[A[i]];
    }

    result != 0
}
