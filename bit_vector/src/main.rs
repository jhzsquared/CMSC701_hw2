use std::io;
use bitvec::prelude::*;

use bit_vector::BitVectorTools;
mod rank;
use crate::rank::RankSupport;
fn main(){
    use std::time::{Instant,Duration};
    let bit_v: BitVec = bitvec![0, 1, 0, 0, 1, 
                                0, 1, 1, 1, 0,
                                1, 0, 1, 0, 1, 
                                0, 1 ,0, 1, 0, 
                                0, 1, 0, 1, 1, 
                                1, 0, 1, 0, 1,
                                0, 1, 0, 1 ];

    let i: usize = 21;

    let fname: &str = "testfile.json";

    let bit_vec_rank: RankSupport = BitVectorTools::new(bit_v);    
    let _: Result<(), io::Error> = bit_vec_rank.save(fname);

    // let bit_vec_rank: RankSupport = BitVectorTools::load(fname);
    // time rank operation
    let now = Instant::now();
    let rank: usize = bit_vec_rank.rank1(i);
    let elapsed: Duration = now.elapsed();
    println!("{}, {:?}, {}, {:.2?}, {}", bit_vec_rank.bit_v, bit_vec_rank.rank_struct, rank, elapsed, bit_vec_rank.overhead());
}