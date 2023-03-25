// Implement a succinct, constant-time, bit-vector rank operation
// methods: rank 1- return number of 1s (exclusive)
            //overhead: return size of rank data structure in bits
            // save: save rank data structure to file (fname) --and bit vector save function
            // load: load rank data structure from fname () --and bit vector load function
// use std::io; 
use get_size:: GetSize;
use bitvec::prelude::*;
use num_integer::div_floor;
use std::io::{BufWriter, BufReader, Write};
use std::fs::File;
use serde_json::from_reader;
use serde::{Serialize,Deserialize};
use std::io;
use std::error::Error;

#[derive(Serialize,Deserialize)]
struct RankSupport {
    bit_vector: BitVec,
    rank_structure: Option<(Vec<usize>, Vec<Vec<usize>>)>
}
impl RankSupport{
    fn create_rank_structure(bit_v: BitVec) -> (Vec<usize>, Vec<Vec<usize>>) {
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
            //iterate through sub chunks for their rank 
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
        return (rank_vec, rank_array);
    }

    fn overhead(rank_struct: (Vec<usize>, Vec<Vec<usize>>)) -> usize {
        rank_struct.0.get_heap_size()*8 + rank_struct.1.get_heap_size()*8
    }

    fn save(fname: &str, bit_v: BitVec, rank_struct:(Vec<usize>, Vec<Vec<usize>>)) -> Result<(), io::Error>{
        // save bitvector and rank data structure to file name
        let file: File = File::create(fname)?;
        let mut writer: BufWriter<File> = BufWriter::new(file);
        serde_json::to_writer(&mut writer, &BitVecRank{bit_vector: bit_v, rank_structure: Some(rank_struct)})?;
        writer.flush()?;
        Ok(())
    }

    fn load(fname: &str) -> Result<BitVecRank, Box<dyn Error>>{
        // load bitvector and rank data structure from file name
        let readfile = File::open(fname)?;
        let mut reader: BufReader<File> = BufReader::new(readfile);
        let bit_vec_rank: BitVecRank = serde_json::from_reader(reader)?;
        Ok(bit_vec_rank)
    }
    
    fn rank1(&self, i: usize) -> usize{
        // return number of 1s in the bit-vector up to position i (exclusive)
        let chunk_size: usize = (bit_v.len() as f64).log2().powi(2).floor() as usize;
        // calculate subchunk size
        let subchunk_size: usize = (0.5*(bit_v.len() as f64).log2().floor()) as usize;
        
        // get index of all various chunks
        let chunk_loc: usize = div_floor(i, chunk_size);
        let subchunk_loc: usize =  div_floor(i % chunk_size, subchunk_size);
        let remainder: usize = i-(chunk_size*chunk_loc+subchunk_size*subchunk_loc);
        // sum all cumulative ranks and rank within subchunk together
        let sum_rank: usize = rank_struct.0[chunk_loc] + rank_struct.1[chunk_loc][subchunk_loc] + 
                    &bit_v[(chunk_loc*chunk_size+subchunk_loc*subchunk_size)..(chunk_loc*chunk_size+subchunk_loc*subchunk_size)+remainder].count_ones();
        return sum_rank;
    }
}

fn main(){
    let bit_v: BitVec = bitvec![0, 1, 0, 0, 1, 
                                0, 1, 1, 1, 0,
                                1, 0, 1, 0, 1, 
                                0, 1 ,0, 1, 0, 
                                0, 1, 0, 1, 1, 
                                1, 0, 1, 0, 1,
                                0, 1, 0, 1 ];
    let rank_struct: (Vec<usize>, Vec<Vec<usize>>) = create_jacob_structure(bit_v.clone());
    println!("{:?}", &rank_struct);
    println!("rank data structure size is {} in bits", overhead(rank_struct.clone()));

    let i: usize = 21;
    //get rank
    let sum_rank: usize = rank1(i, bit_v.clone(), rank_struct.clone());

    println!("{}", sum_rank);
    assert_eq!(sum_rank, bit_v[..i].count_ones());
    let fname: &str = "testfile.json";
    let result: Result<(), io::Error> = save(fname, bit_v, rank_struct);
    let bit_vec_rank:BitVecRank = load(fname).unwrap();
    println!("{}, {:?}", bit_vec_rank.bit_vector, bit_vec_rank.rank_structure);
}