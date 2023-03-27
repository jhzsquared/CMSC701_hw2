 // Implement a succinct, (at most) log time bit-vector select operation (uses rank data structure)
use std::io;
use bitvec::prelude::*;
use serde::{Serialize,Deserialize};
use num_integer::div_floor;

use bit_vector::BitVectorTools;
use crate::rank::RankSupport;



#[derive(Serialize,Deserialize, Debug)]
pub struct SelectSupport {
    pub rank_support: RankSupport
}

impl SelectSupport{    
    pub fn select1(&self, j: usize) -> usize {
        // Rank support needs to already be loaded
        // returns position of the first index,j,  for which rank1(j) = i
        // binary search across rank structure to find select
        let mut L: usize = 0;
        let mut R: usize = self.rank_support.bit_v.len()-1;
        
        let mut rank:usize;
        let mut m: usize = 0;
        while L < R{
            // get new splice
            m = div_floor(L+R,2);
            //get new rank
            rank = self.rank_support.rank1(m);
            if rank < j {
                //select is in 2nd half
                L = m+1;
            } else {
                // select is in 1st half
                R = m;
            }     
        }
        //check last eq
        if self.rank_support.rank1(L) == j{
            m=L;
            return m;
        } else {
            panic!("Select query is too large and does not exist");
        }
    }
}

impl BitVectorTools for SelectSupport{
    fn new(bit_v: BitVec) -> Self {
        SelectSupport {rank_support: BitVectorTools::new(bit_v)}
    }

    fn load(fname: &str) -> Self{
        // load bitvector and rank data structure from file name
        SelectSupport {rank_support: BitVectorTools::load(fname)}
    }

    fn overhead(&self) -> usize {
        return self.rank_support.overhead()
    }
    
    fn save(&self, fname: &str) -> Result<(), io::Error>{
        self.rank_support.save(fname)
    }

}