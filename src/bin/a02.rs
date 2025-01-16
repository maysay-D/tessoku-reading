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
        X: usize,
        A: [usize; N],
    }

    let ans = solve(X, A);
    println!("{}", if ans {"Yes"} else {"No"});
}

fn solve(X: usize, A: Vec<usize>) -> bool {
    for i in 0..A.len() {
        if A[i] == X {
            return true
        }
    }
    false
}
