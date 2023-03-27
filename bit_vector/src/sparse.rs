use std::io;
use std::io::{BufWriter, BufReader, Write};
use std::fs::File;

use bitvec::prelude::*;
use bit_vector::BitVectorTools;
use get_size::GetSize;
use serde::{Serialize,Deserialize};

use crate::select::SelectSupport;

#[derive(Debug)]
pub struct SparseArrayBuilder{
    bit_v:BitVec,
    values:Vec<String>
}

#[derive(Serialize,Deserialize, Debug)]
pub struct SparseArray{
    values: Vec<String>,
    select_support: SelectSupport //which includes bit_v
}

impl SparseArrayBuilder{
    //build sparse array 
    pub fn create(size: usize) -> SparseArrayBuilder {
        // let mut array: SparseArrayBuilder = SparseArrayBuilder {bit_v: BitVec::with_capacity(size), values: Vec::new()}; 
        // weird error is occuring where bit_v has extra ones. so fixing by initiating it with 0 value bitvec
        let mut array: SparseArrayBuilder = SparseArrayBuilder {bit_v: bitvec![0;size], values: Vec::new()};
        unsafe{array.bit_v.set_len(size)};
        return array;
    }

    pub fn append(&mut self, elem: String, pos: usize) {
        if pos > self.bit_v.capacity(){
            panic!("invalid position entry")
        }
        self.bit_v.set(pos, true);
        self.values.push(elem);
    }

    pub fn finalize(self) -> SparseArray{
        //create rank data structure (which will also be used for select)
        SparseArray {values: self.values, select_support: BitVectorTools::new(self.bit_v.clone())}
    }
}

impl SparseArray{
    //use sparse array
    pub fn get_at_rank(&self, r:usize, elem: &mut String) -> bool{
        // look at rth item; if there are at least r items, return true
        if self.values.len() >= r {
            *elem = (self.values[r]).to_string();
            return true;
        } else{
            return false;
        }
    }
    
    pub fn get_at_index(&self, r:usize, elem: &mut String) -> bool{
          // if bit at index r is 1, return true and set value to elem
          let result: bool = self.select_support.rank_support.bit_v[r];
          if result {
            *elem = (self.values[self.select_support.rank_support.rank1(r)]).to_string();
          }
          return result
    }

    pub fn get_index_of(&self, r:usize) -> Option<usize> {
        //take rank r and return index in sparse array where rth present element appears
        if r > self.values.len(){ // r is impossibly big
            return None;
        } else{
            return Some((self.select_support.select1(r) as i64 -1)as usize);
        }
    } 

    pub fn num_elem_at(&self, r:usize) -> usize {
        // inclusive rank of bitvector (count of 1s up to and including r)
        return self.select_support.rank_support.rank1(r+1);
    }

    pub fn size(&self) -> usize {
        // get size of sparse array in bits (value vector, bit vector, and extra (rank) data structure)
        return self.select_support.overhead()+ self.values.get_heap_size()*8;
    }

    pub fn num_elem(&self) -> usize{
        // return number of present elements
        return self.values.len();
    }

    pub fn save(&self, fname: &str) -> Result<(), io::Error>{
        //save SparseArray (value, select support to file)
        let file: File = File::create(fname)?;
        let mut writer: BufWriter<File> = BufWriter::new(file);
        serde_json::to_writer(&mut writer, &self)?;
        writer.flush()?;
        Ok(())
    }

    pub fn load(fname: &str) -> SparseArray {
        // load SparseArray file name
        let readfile = File::open(fname).unwrap();
        let reader: BufReader<File> = BufReader::new(readfile);
        let array: SparseArray = serde_json::from_reader(reader).unwrap();
        return array;
    }
}

