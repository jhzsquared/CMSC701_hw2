use bitvec::prelude::*;
use std::io;

pub trait BitVectorTools {
    fn new(bit_v: BitVec) -> Self;
    fn load(fname: &str) -> Self;
    fn overhead(&self) -> usize;
    fn save (&self, fname: &str) -> Result<(), io::Error>;

}