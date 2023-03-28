# Programming Assignment 2

## Language
- Rust
## Setup
- Having cloned the project, `cd bit_vector` and use `cargo run` to generate a target directory, compile all dependencies, and run the main.rs file
- cmake and libfontconfig packages are also needed `apt-get install cmake libfontconfig1-dev`

## Overall Structure
- Use `use bit::BitVectorTools` to acccess the public `BitVectorTools` trait in lib.rs
- `main.rs` contains the functions for generating the plots for the various tasks
  - `test_rank`: creates 50 random bitvectors of size 20000 to a million with sparsity of 10% and times how long it will take to conduct 50 random rank operations and measures the size of the rank data structure. These are plotted against bit vector length and saved to file
  - `test_select`: creates 50 random bitvectors of size 20000 to a million with sparsity of 10% and times how long it will take to conduct 50 random select operations and measures the size of the select data structure. These are plotted against bit vector length and saved to file
  - `test_sparse`: creates sparse array structures of varying sizes from 20000 to a million and of varying sparsity (1%, 5%, 10%), measures its' structure size, and times the various functions. These are plotted on charts against the bit vector size and saved to file.
- `rank.rs` contains the Rank implementation for Task 1
- `select.rs` contains the Select implementation for Task 2
- `sparse.rs` contains the Sparse Array implementation for Task 3
  
## Task 1 bit-vector rank
- `bit_vector/src/rank.rs` contains the implementation for Task 1
- Include `mod rank;` and `use crate::rank::RankSupport;` to use structures in new src/*.rs files
### How to implement RankSupport:
1. `bit_vec_rank: RankSupport = BitVectorTools::new(bit_v);`: Initialize `RankSupport` structure with a bit vector, where `bit_v` is of type `BitVec` that you could previously have read in and converted from a vector
   1. Or use `BitVectorTools::load(fname)`: load structure from previously saved file `fname` 
2. `bit_vec_rank.rank1(i)`: Calculate rank-1 at index i-exclusive. Return `usize`
3. `bit_vec_rank.overhead()`: Calculate overhead in bits for rank data structure. Return `usize`
4. `bit_vec_rank.save(fname)`: Save rank data structure to `fname`
   
## Task 2 bit-vector select
- `bit_vector/src/select.rs` contains the implementation for Task 2
- Include `mod select;` and `use crate::select::SelectSupport;` to use structures in new src/*.rs files
### How to implement SelectSupport
1. `let bit_vec_rank: SelectSupport = BitVectorTools::new(bit_v);`: Initialize `SelectSupport` structure with a bit vector:  where `bit_v` is of type `BitVec` that you could previously have read in and converted from a vector
   1. Or use `BitVectorTools::load(fname)`: load in a SelectSupport structure from previously saved file `fname`
2. `bit_vec_rank.select1(i);`: Calculate select-1 for index i. Return `usize`
3. `bit_vec_rank.overhead()`: Calculate overhead in bits for select data structure (conveniently same as the rank data structure). Return `usize`
4. `bit_vec_rank.save(fname)`: Save `SelectSupport` structure to `fname`
   
## Task 3 Sparse array
- `bit_vector/src/sparse.rs` contains the implementation for Task 3
- Include `mod sparse;` and `use crate::sparse::{SparseArray, SparseArrayBuilder};` to use structures in new src/*.rs files
### How to implement SparseArray
1. `let array: SparseArrayBuilder = SparseArrayBuilder::create(n as usize)`: Initialize `SparseArrayBuilder` structure for array of size `n`: 
   1. Or use `SparseArray::load(fname)` to load a previously saved SparseArray structure from file at `fname`.
2.  `array.append(elem, pos)`: Add `elem` at position `pos` to the sparse array
3. `let sparse_array: SparseArray = array.finalize()`: Finalize sparse array (create rank structure) into a `SparseArray` structure. 
4. `sparse_array.get_at_rank(r, elem)`:  if bit at index r is 1, return true and set value to elem
5. `sparse_array.get_index_of(r)`:  take rank r and return index in sparse array where rth present element appears
6. `sparse_array.num_elem_at(r)`: return inclusive rank of bitvector (count of 1s up to and including r)
7. `sparse_array.size()`: return size in bits of value vector and rank data structure
8. `spase_array.num_elem()`: return number of present elements
9. `sparse_array.nsave(fname)`: save `SparseArray` structure to `fname`
       

## Resources
- https://en.wikipedia.org/wiki/Binary_search_algorithm (left most binary search pseudocode)
- https://stackoverflow.com/questions/48218459/how-do-i-generate-a-vector-of-random-numbers-in-a-range (used to generate bitvectors for testing)
- https://plotters-rs.github.io/book/basic/basic_data_plotting.html (used to make plots for writeup)
- https://doc.rust-lang.org/std/mem/fn.size_of.html (used for justification of non-sparse array size estimates)
