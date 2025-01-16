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
        Q: usize,
        X: [usize; Q],
    }

    let ans = solve(Q, X);
    for a in ans {
        println!("{}", if a { "Yes" } else { "No" });
    }
}

fn solve(Q: usize, X: Vec<usize>) -> Vec<bool> {
    let mut result = vec![false; Q];
    for i in 0..Q {
        let x = X[i];
        result[i] = is_prime(x);
    }
    result
}
fn is_prime(x: usize) -> bool {
    // Using the square root of x (O(sqrt(N))) (AC)
    for i in 2..x.sqrt() + 1 {
        if x % i == 0 {
            return false;
        }
    }
    true

    // Simple version (O(N)) (TLE)
    /*
    for i in 2..x {
        if x % i == 0 {
            return false;
        }
    }
    true
    */

    // Eratosthenes sieve (O(N log log N)) (TLE)
    /*
    let mut prime_table = vec![true; x + 1];
    prime_table.iter_mut().take(2).for_each(|x| *x = false);
    for i in 2..x {
        if prime_table[i] {
            let mut j = i * 2;
            while j <= x {
                prime_table[j] = false;
                j += i;
            }
        }
    }
    prime_table[x]
    */

    // Eratosthenes sieve with using the square root of x (O(sqrt(N) log log N)) (TLE)
    /*
    let mut prime_table = vec![true; x + 1];
    prime_table.iter_mut().take(2).for_each(|x| *x = false);
    for i in 2..x.sqrt() + 1 {
        if prime_table[i] {
            let mut j = i * 2;
            while j <= x.sqrt() {
                prime_table[j] = false;
                j += i;
            }
        }
    }
    prime_table[x]
    */
}
