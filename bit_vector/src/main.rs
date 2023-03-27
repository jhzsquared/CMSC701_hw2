use std::time::{Instant,Duration};
use bitvec::prelude::*;
use bit_vector::BitVectorTools;

use plotters::prelude::*;
use rand::{distributions::Bernoulli, distributions::Uniform, Rng};
use itertools::izip;
use get_size:: GetSize;

mod rank;
mod select;
mod sparse;
use crate::rank::RankSupport;
use crate::select::SelectSupport;
use crate::sparse::{SparseArray, SparseArrayBuilder};


fn test_rank(){
    //test rank module (iterate through random bit vectors of length n
    // measure rank structure and time how long it takes to do 100 rank operations, output 2 plots)
    // build bit vector of size n with probabiliity B of being 1
    let mut n_vec: Vec<f64> = Vec::new();
    let mut duration_vec: Vec<f64> = Vec::new();
    let mut overhead_vec: Vec<f64> = Vec::new();
    for interval in 1..50 {
        let n: usize = 20000*interval;
        const B: f64 = 0.1;
        let b_dist:Bernoulli = Bernoulli::new(B).unwrap();
        let mut rng = rand::thread_rng();
        let bit_v: BitVec = (0..n).map(|_| rng.sample(&b_dist)).collect();
        let bit_vec_rank: RankSupport = BitVectorTools::new(bit_v);
        let overhead:usize =  bit_vec_rank.overhead();
        let mut total_duration: Duration = Duration::new(0,0);
        //generate random ranks to find and time
        for _ in 0..50 {
            let u_dist: Uniform<usize> = Uniform::new(0,n);
            let i: usize = rng.sample(u_dist);
            let now = Instant::now();
            let _: usize = bit_vec_rank.rank1(i);
            let duration: Duration = now.elapsed();
            total_duration += duration;
        }
        n_vec.push(n as f64);
        overhead_vec.push(overhead as f64);
        duration_vec.push(total_duration.as_micros() as f64);
        
    }
    // create plots
    let fname_overhead: &str = "rank_overhead.png";
    let fname_duration: &str = "rank_duration.png";
   
    //overhead plot
    let overhead_zip: Vec<(f64, f64)> = n_vec.iter().cloned().zip(overhead_vec.iter().cloned()).collect();
    let duration_zip: Vec<(f64, f64)> = n_vec.iter().cloned().zip(duration_vec.iter().cloned()).collect();
        
    let n_max: f64 =  n_vec.into_iter().reduce(f64::max).unwrap();
    let overhead_max: f64 =  overhead_vec.into_iter().reduce(f64::max).unwrap();

    let overhead_area = BitMapBackend::new(fname_overhead, (1000, 800)).into_drawing_area();
        overhead_area.fill(&WHITE).unwrap();
    let mut overhead_ctx = ChartBuilder::on(&overhead_area)
        .set_label_area_size(LabelAreaPosition::Left, 80.0)
        .set_label_area_size(LabelAreaPosition::Bottom, 50.0)
        .caption("Overhead of rank structure vs in bits", ("sans-serif", 40.0))
        .build_cartesian_2d(0.0..n_max, 0.0..overhead_max)
        .unwrap();
    
    overhead_ctx.configure_mesh().draw().unwrap();
    overhead_ctx.draw_series(
        overhead_zip.iter().map(|point| Circle::new(*point, 4.0_f64, ShapeStyle::from(&BLUE).filled())),
    ).unwrap();

    //duration plot
    let duration_area = BitMapBackend::new(fname_duration, (1000, 600)).into_drawing_area();
    duration_area.fill(&WHITE).unwrap();
    let duration_max: f64 =  duration_vec.into_iter().reduce(f64::max).unwrap();
    let mut duration_ctx = ChartBuilder::on(&duration_area)
        .set_label_area_size(LabelAreaPosition::Left, 80.0)
        .set_label_area_size(LabelAreaPosition::Bottom, 50.0)
        .caption("Microsecs for 50 rank ops vs  bit-vector size", ("sans-serif", 40.0))
        .build_cartesian_2d(0.0..n_max, 0.0..duration_max)
        .unwrap();
    
    duration_ctx.configure_mesh().draw().unwrap();
    duration_ctx.draw_series(
        duration_zip.iter().map(|point| Circle::new(*point, 4.0_f64, ShapeStyle::from(&BLUE).filled())),
    ).unwrap();
}

fn test_select(){
    // test select module (iterate through random bit vectors of length n, 
    // measure select (aka rank) structure and time how long it takes to do 100  select operations, output 2 plots)
    // build bit vector of size n with probabiliity B of being 1
    let mut n_vec: Vec<f64> = Vec::new();
    let mut duration_vec: Vec<f64> = Vec::new();
    let mut overhead_vec: Vec<f64> = Vec::new();
    for interval in 1..50 {
        let n: usize = 20000*interval;
        const B: f64 = 0.1;
        let b_dist:Bernoulli = Bernoulli::new(B).unwrap();
        let mut rng = rand::thread_rng();
        let bit_v: BitVec = (0..n).map(|_| rng.sample(&b_dist)).collect();
        let bit_vec_rank: SelectSupport = BitVectorTools::new(bit_v);
        let overhead:usize =  bit_vec_rank.overhead();
        let mut total_duration: Duration = Duration::new(0,0);
        // since we have the function panic if select i is too big define max
        let max_ones: usize = bit_vec_rank.rank_support.bit_v.count_ones(); 
        //generate random selects to find and time
        for _ in 0..50 {
            let u_dist: Uniform<usize> = Uniform::new(0, max_ones);
            let i: usize = rng.sample(u_dist);
            let now = Instant::now();
            let _: usize = bit_vec_rank.select1(i);
            let duration: Duration = now.elapsed();
            total_duration += duration;
        }
        n_vec.push(n as f64);
        overhead_vec.push(overhead as f64);
        duration_vec.push(total_duration.as_micros() as f64);
        
    }
    // create plots
    let fname_overhead: &str = "select_overhead.png";
    let fname_duration: &str = "select_duration.png";
   

    let overhead_zip: Vec<(f64, f64)> = n_vec.iter().cloned().zip(overhead_vec.iter().cloned()).collect();
    let duration_zip: Vec<(f64, f64)> = n_vec.iter().cloned().zip(duration_vec.iter().cloned()).collect();
        
    let n_max: f64 =  n_vec.into_iter().reduce(f64::max).unwrap();
    let overhead_max: f64 =  overhead_vec.into_iter().reduce(f64::max).unwrap();
    //overhead plot
    let overhead_area = BitMapBackend::new(fname_overhead, (1000, 800)).into_drawing_area();
        overhead_area.fill(&WHITE).unwrap();
    let mut overhead_ctx = ChartBuilder::on(&overhead_area)
        .set_label_area_size(LabelAreaPosition::Left, 80.0)
        .set_label_area_size(LabelAreaPosition::Bottom, 50.0)
        .caption("Overhead of select structure in bits", ("sans-serif", 40.0))
        .build_cartesian_2d(0.0..n_max, 0.0..overhead_max)
        .unwrap();
    
    overhead_ctx.configure_mesh().draw().unwrap();
    overhead_ctx.draw_series(
        overhead_zip.iter().map(|point| Circle::new(*point, 4.0_f64, ShapeStyle::from(&BLUE).filled())),
    ).unwrap();
    //duration plot
    let duration_area = BitMapBackend::new(fname_duration, (1000, 600)).into_drawing_area();
    duration_area.fill(&WHITE).unwrap();
    let duration_max: f64 =  duration_vec.into_iter().reduce(f64::max).unwrap();
    let mut duration_ctx = ChartBuilder::on(&duration_area)
        .set_label_area_size(LabelAreaPosition::Left, 80.0)
        .set_label_area_size(LabelAreaPosition::Bottom, 50.0)
        .caption("Microsecs for 50 select ops vs  bit-vector size", ("sans-serif", 40.0))
        .build_cartesian_2d(0.0..n_max, 0.0..duration_max)
        .unwrap();
    
    duration_ctx.configure_mesh().draw().unwrap();
    duration_ctx.draw_series(
        duration_zip.iter().map(|point| Circle::new(*point, 4.0_f64, ShapeStyle::from(&BLUE).filled())),
    ).unwrap();
}

fn test_sparse(){
    // test sparse array module on sparse arrays of varying length n. 
    // and varying sparsity--look at speed for the 4 primary functions (4 plots w/ 5 typess) and size (compared to explicit string storage)

    let n_vec: Vec<f64> = (1..50).map(|x| 20000.*x as f64).collect::<Vec<f64>>();
  
    let mut overhead_vec2: Vec<Vec<f64>> = Vec::new();
    let mut notsparse_vec2: Vec<Vec<f64>> = Vec::new();
    let mut gar_vec2: Vec<Vec<f64>> = Vec::new();
    let mut gai_vec2: Vec<Vec<f64>> = Vec::new();
    let mut gio_vec2: Vec<Vec<f64>> = Vec::new();
    let mut nea_vec2: Vec<Vec<f64>> = Vec::new();
    for B in [0.01,0.05,0.1]{
        let b_dist:Bernoulli = Bernoulli::new(B).unwrap();
        let mut rng = rand::thread_rng();
        let mut overhead_vec: Vec<f64> = Vec::new();
        let mut notsparse_vec: Vec<f64> = Vec::new();
        //iterate through the functions to get more accurate times
        let mut gar_vec: Vec<f64> = Vec::new();
        let mut gai_vec: Vec<f64> = Vec::new();
        let mut gio_vec: Vec<f64> = Vec::new();
        let mut nea_vec: Vec<f64> = Vec::new();
        
            for n in n_vec.clone(){
            let bit_v: BitVec = (0..n as usize).map(|_| rng.sample(&b_dist)).collect();
            //create empty array of size n
            let mut array: SparseArrayBuilder = SparseArrayBuilder::create(n as usize);
            // add items to array
            let mut count:usize = 0;
            for item in bit_v{
                if item{ //true add value to SparseArray and position count
                    array.append(item.to_string(), count);
                }
                count+=1;
            }
            //finalize array
            let sparse_array: SparseArray = array.finalize();
            let overhead: usize =  sparse_array.size();
            // let sparsity: usize = sparse_array.num_elem();
            let mut gar_duration: Duration = Duration::new(0,0);
            let mut gai_duration: Duration = Duration::new(0,0);
            let mut gio_duration: Duration = Duration::new(0,0);
            let mut nea_duration: Duration = Duration::new(0,0);
            let sparsity:usize = sparse_array.num_elem();
            //estimate size if we stored empty strings 
            let new_size: f64 = overhead as f64 +(n -sparsity as f64)*4.*8.; //froom std::mem::size_of for a char
            for _ in 0..30 { //run multiple iterations to get more stable numbers
                if sparsity==0{ //random odds won and there are no values
                    continue
                }
                let gar_dist: Uniform<usize> = Uniform::new(0, sparsity);
                let gar: usize = rng.sample(gar_dist);
                let gai_dist: Uniform<usize> = Uniform::new(0, n as usize);
                let gai: usize = rng.sample(gai_dist);
                let mut elem: String = String::from("e");
                let mut elem2: String = String::from("e");
                // test get at rank
                let gar_now = Instant::now();
                sparse_array.get_at_rank(gar, &mut elem);
                gar_duration += gar_now.elapsed();
                
                // test get at index
                let mut elem2: String = String::from("e");
                let gai_now = Instant::now();
                sparse_array.get_at_index(gai, &mut elem2);
                gai_duration += gai_now.elapsed();
               
                //test get index of 
                let gio_now = Instant::now();
                sparse_array.get_index_of(gar);
                gio_duration += gio_now.elapsed();
                
                //test num elem at
                let nea_now = Instant::now();
                sparse_array.num_elem_at(gai);
                nea_duration += nea_now.elapsed();
                
            }
            overhead_vec.push(overhead as f64);
            notsparse_vec.push(new_size);
            gar_vec.push(gar_duration.as_micros() as f64);
            gai_vec.push(gai_duration.as_micros() as f64);
            gio_vec.push(gio_duration.as_micros() as f64);
            nea_vec.push(nea_duration.as_micros() as f64);
        }
        overhead_vec2.push(overhead_vec);
        notsparse_vec2.push(notsparse_vec);
        gar_vec2.push(gar_vec);
        gai_vec2.push(gai_vec);
        gio_vec2.push(gio_vec);
        nea_vec2.push(nea_vec);
    }
    // create plots overhead, 

    let fname_vec:[&str;6] = ["sparse_overhead.png", "sparse_notsparse_overhead.png", "sparse_gar_duration.png", "sparse_gai_duration.png", 
                            "sparse_gio_duration.png", "sparse_nea_duration.png"];
    let vec_vecs:[Vec<Vec<f64>>;6] = [overhead_vec2, notsparse_vec2, gar_vec2, gai_vec2, gio_vec2, nea_vec2];
    let graph_captions:[&str; 6] = ["Overhead of Sparse Array in bits", "Overhead of NOT-Sparse Array in bits", "Get at rank time in microsec", 
                                    "Get at index time in microsec", "Get index of in microsec", "Number of elements at x in microsec"];
    let max_x: f64 = n_vec.clone().into_iter().reduce(f64::max).unwrap();
    for (result_vec,fname, caption) in izip!(vec_vecs,fname_vec, graph_captions){
        // generic plot, different colors by sparsity
        let data_area = BitMapBackend::new(fname, (1000, 600)).into_drawing_area();
        data_area.fill(&WHITE).unwrap();
        let max_y:f64 = result_vec.clone().into_iter().flatten().collect::<Vec<f64>>().into_iter().reduce(f64::max).unwrap();
        let mut data_ctx = ChartBuilder::on(&data_area)
            .set_label_area_size(LabelAreaPosition::Left, 80.0)
            .set_label_area_size(LabelAreaPosition::Bottom, 50.0)
            .caption(caption, ("sans-serif", 40.0))
            .build_cartesian_2d(0.0..max_x, 0.0..max_y)
            .unwrap();
        data_ctx.configure_mesh().draw().unwrap();
        let data_zip0: Vec<(f64, f64)> = n_vec.iter().cloned().zip(result_vec[0].iter().cloned()).collect();
        data_ctx.draw_series(
            data_zip0.iter().map(|point| Circle::new(*point, 4.0_f64, ShapeStyle::from(&BLUE).filled())),
        ).unwrap().label(".01").legend(|(x, y)| Circle::new((x,y), 4.0_f64, ShapeStyle::from(&BLUE).filled()));
        let data_zip1: Vec<(f64, f64)> = n_vec.iter().cloned().zip(result_vec[1].iter().cloned()).collect();
        data_ctx.draw_series(
            data_zip1.iter().map(|point| TriangleMarker::new(*point, 4.0_f64, ShapeStyle::from(&GREEN).filled())),
        ).unwrap().label(".05").legend(|(x, y)|Circle::new((x,y), 4.0_f64, ShapeStyle::from(&GREEN).filled()));
        let data_zip2: Vec<(f64, f64)> = n_vec.iter().cloned().zip(result_vec[2].iter().cloned()).collect();
        data_ctx.draw_series(
            data_zip2.iter().map(|point| Cross::new(*point, 4.0_f64, ShapeStyle::from(&RED).filled())),
        ).unwrap().label(".1").legend(|(x, y)| Cross::new((x,y), 4.0_f64, ShapeStyle::from(&RED).filled()));

        data_ctx.configure_series_labels()
            .border_style(&BLACK)
            .background_style(&WHITE.mix(0.8))
            .draw()
            .unwrap();
       
    }
    
}

fn main(){
    println!("Testing Rank");
    test_rank();
    println!("Testing Select");
    test_select();
    println!("Testing sparse array");
    test_sparse();

}