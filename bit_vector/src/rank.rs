// Implement a succinct, constant-time, bit-vector rank operation
// methods: rank 1- return number of 1s (exclusive)
            //overhead: return size of rank data structure in bits
            // save: save rank data structure to file (fname) --and bit vector save function
            // load: load rank data structure from fname () --and bit vector load function
use std::io;
use std::io::{BufWriter, BufReader, Write};
use std::fs::File;

use get_size:: GetSize;
use bitvec::prelude::*;
use num_integer::div_floor;
use serde::{Serialize,Deserialize};

use bit_vector::BitVectorTools;

#[derive(Serialize,Deserialize, Debug)]
pub struct RankSupport {
    pub bit_v: BitVec,
    pub rank_struct: (Vec<usize>, Vec<Vec<usize>>)
}

impl RankSupport{

    pub fn create_rank_structure(bit_v:BitVec) -> (Vec<usize>, Vec<Vec<usize>>){
        //create rank structure for chunks and subchunks per jacobson's method
        // calculate chunk size (going to floor for consistency sake)
        let chunk_size: usize = (bit_v.len() as f64).log2().powi(2).floor() as usize;
        let num_chunks: usize = ((bit_v.len() as f64)/(chunk_size as f64)).floor() as usize;

        // calculate subchunk size
        let subchunk_size: usize = (0.5*(bit_v.len() as f64).log2().floor()) as usize;
        let num_subchunk: usize = chunk_size/subchunk_size as usize;
        // iterate through chunk slices and get cumulative rank(1) of chunks and subchunks
        let mut i: usize = 0;
        let mut rank_vec: Vec<usize> = vec![0; num_chunks+1];
        let mut rank_array: Vec<Vec<usize>> = Vec::new();
        let mut sum_rank: usize = 0;

        while i  < num_chunks { // don't need to look at last (potentially short) subchunk
            let chunk_rank: usize = (&bit_v[(i*chunk_size)..((i+1)*chunk_size)]).count_ones();
            sum_rank += chunk_rank;
            rank_vec[i+1] = sum_rank; //first entry is 0
            //iterate through previous chunk's subchunks for their rank 
            let mut j: usize = 0;
            let mut sub_vec: Vec<usize> = vec![0; num_subchunk+1];
            let mut sum_subchunk_rank: usize = 0;
            while j < num_subchunk {
                let subchunk_rank: usize = (&bit_v[(i*chunk_size+j*subchunk_size)..(i*chunk_size+(j+1)*subchunk_size)]).count_ones();
                sum_subchunk_rank += subchunk_rank;
                sub_vec[j+1] = sum_subchunk_rank;
                j+=1;
            }
            //add subchunk ranks to array
            rank_array.push(sub_vec); 
            i+=1;
        }
        // add remaining subchunks (remainder will be 0)
        let mut j: usize = 0;
        let mut sub_vec: Vec<usize> = vec![0; num_subchunk+1];
        let mut sum_subchunk_rank: usize = 0;
        while (i*chunk_size+(j+1)*subchunk_size) < bit_v.len() {
            let subchunk_rank: usize = (&bit_v[(i*chunk_size+j*subchunk_size)..(i*chunk_size+(j+1)*subchunk_size)]).count_ones();
            sum_subchunk_rank += subchunk_rank;
            sub_vec[j+1] = sum_subchunk_rank;
            j+=1;
        }
        rank_array.push(sub_vec); 
        return (rank_vec, rank_array);
    }

    pub fn rank1(&self, i: usize) -> usize{
        // return number of 1s in the bit-vector up to position i (exclusive)
        let chunk_size: usize = (self.bit_v.len() as f64).log2().powi(2).floor() as usize;
        // calculate subchunk size
        let subchunk_size: usize = (0.5*(self.bit_v.len() as f64).log2().floor()) as usize;
        // get index of all various chunks
        let chunk_loc: usize = div_floor(i, chunk_size);
        let subchunk_loc: usize =  div_floor(i % chunk_size, subchunk_size);
        let remainder: isize = (i as isize)-((chunk_size*chunk_loc+subchunk_size*subchunk_loc) as isize);
        let mut sum_rank: usize;
        // sum all cumulative ranks and rank within subchunk together
        if (chunk_loc*chunk_size+subchunk_loc*subchunk_size)+(remainder as usize) <= self.bit_v.len() {
            sum_rank = self.rank_struct.0[chunk_loc] 
                + self.rank_struct.1[chunk_loc][subchunk_loc]
                + &self.bit_v[(chunk_loc*chunk_size+subchunk_loc*subchunk_size)..(chunk_loc*chunk_size+subchunk_loc*subchunk_size)+(remainder as usize)].count_ones();
        } else {
            // looking for rank bigger than size of bit vector
            sum_rank =  self.rank_struct.0[chunk_loc] 
                + self.rank_struct.1[chunk_loc][subchunk_loc]
                + &self.bit_v[(chunk_loc*chunk_size+subchunk_loc*subchunk_size)..].count_ones();
        }
        
        assert_eq!(sum_rank, self.bit_v[0..i].count_ones()); //debug
        return sum_rank;
    }
}

impl BitVectorTools for RankSupport {

    fn new(bit_v: BitVec) -> RankSupport {
        //new bit vector so create the rank structure
        let bit_vec_rank:RankSupport = RankSupport{bit_v: bit_v.clone(), rank_struct:RankSupport::create_rank_structure(bit_v)};
        // bit_vec_rank.rank_struct = Some(bit_vec_rank.create_rank_structure());
        return bit_vec_rank;
    }

    fn load(fname: &str) -> RankSupport{
        // load bitvector and rank data structure from file name
        let readfile = File::open(fname).unwrap();
        let reader: BufReader<File> = BufReader::new(readfile);
        let bit_vec_rank: RankSupport = serde_json::from_reader(reader).unwrap();
        return bit_vec_rank;
    }
    
    fn overhead(&self) -> usize {
        return self.rank_struct.0.get_heap_size()*8 + self.rank_struct.1.get_heap_size()*8;
    }
    
    fn save(&self, fname: &str) -> Result<(), io::Error>{
        // save bitvector and rank data structure to file name
        let file: File = File::create(fname)?;
        let mut writer: BufWriter<File> = BufWriter::new(file);
        serde_json::to_writer(&mut writer, &self)?;
        writer.flush()?;
        Ok(())
    }

}
