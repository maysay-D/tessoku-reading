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
    }

    let ans = solve(N);
    for a in ans {
        println!("{}", a);
    }
}

fn solve(N: usize) -> Vec<usize> {
    let result = eratosthenes_sieve(N);
    result
}

fn eratosthenes_sieve(n: usize) -> Vec<usize> {
    let mut prime_tab = vec![true; n + 1];
    prime_tab.iter_mut().take(2).for_each(|x| *x = false);
    for i in 2..=n.sqrt() {
        if prime_tab[i] {
            for j in (i * i..=n).step_by(i) {
                prime_tab[j] = false;
            }
        }
    }
    prime_tab
        .iter()
        .enumerate()
        .filter_map(|(i, &x)| if x { Some(i) } else { None })
        .collect()
}
