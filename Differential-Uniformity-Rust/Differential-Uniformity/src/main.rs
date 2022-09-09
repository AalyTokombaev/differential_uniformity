use std::ops;
use std::hash::{Hash, Hasher};
use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::env;
use std::cmp::Eq;

fn main() {
    // Command line arguments here!
    let args: Vec<String> = env::args().collect();

    let filename = args.get(1); 
    let path = format!("inputs/{}", filename.unwrap());
    
    let file = File::open(path).unwrap();
    let reader: Vec<String> = BufReader::new(file).lines().map(|line| line.unwrap()).collect();
    
    // og God why is it this hard 
    let dim: u16 = reader.get(0).unwrap().parse().unwrap();
    // Read the line, split, and map to to be integers instead
    let numberss : Vec<String> = reader.get(1).unwrap().split(" ").map(|e| e.to_string()).collect();
    let numbers: Vec<u16> = numberss.into_iter().map(|e| e.parse().unwrap()).collect();

    let _nl = numbers.len();
    

    let input_vectors: Vec<BVector> = (0..numbers.len() as u16).map(|e| to_binary_vector(e, dim)).collect();
    let output_vectors: Vec<BVector> = numbers.into_iter().map(|e| to_binary_vector(e, dim)).collect();


    // Create A lookup table for the function.
    let mut f: HashMap<BVector, usize> = HashMap::new();

    let l = input_vectors.len() as usize;
    for i in 0usize..l {
        let k: BVector = input_vectors.get(i).unwrap().clone();
        let v = i;

        f.insert(k, v);
    }
 
    println!("{}", differential_uniformity(input_vectors, output_vectors, f, dim as u16))
}


// I can't be bothered anymore.
type BV = BVector;
fn differential_uniformity(ins: Vec<BV>, _outs: Vec<BV>, f: HashMap<BV, usize>, dim: u16) -> i16 {
    let mut max = 0;
    let n = ins.len();

    let mut total = 1;
    for _ in 0..dim {
        total = total * 2
    }

    let newtotal  = u128::pow(total, 3);

    let mut status: f64 = 0.0;

    for i in 1usize..n as usize {
        let a = ins.get(i).unwrap();
        // For all B
        for _j in 0usize..n as usize {
            let mut count = 0;
            let b = _outs.get(i).unwrap().clone(); // B
            // for all X
            for k in 0usize..n as usize {
                status = status + 1.0;
                if status as i32 % 1000000 == 0 {println!("{} out of\n{}", status, newtotal)}
                let x: BVector = ins.get(k).unwrap().clone(); // X
                let fxa_index: usize = f.get(&(x.clone() + a.clone())).unwrap().clone();
                let fxa: BVector = _outs.get(fxa_index).unwrap().clone();
                let fx_index: usize = f.get(&x).unwrap().clone();
                let fx: BVector = _outs.get(fx_index).unwrap().clone();

                if fxa + fx == b {
                    count += 1; 
                }
                if count > max {
                    max = count;
                }
            }
        }
    }
    
    max
}


fn approx_log(n : u16) -> u16 {
    // just divide by 2 until we hit 1, and boom, approximate log
    let mut m = n;
    let mut count = 0;
    while m > 1 {
        count = count+1;
        m = m / 2; 
    }
   

    count + 1
}

fn to_binary_vector(n: u16, dim: u16) -> BVector { // after all why shouldn't it be an i8 if the numbers are just 0 and 1 
    let missing = dim - approx_log(n);

    let mut m = n;
    let mut expansion: Vec<i8> = vec![];

    while m > 0 {
        expansion.push((m % 2) as i8);
        m = m / 2;
    }
    if n == 0 {
        expansion = vec![0]
    }
    for _ in 1..missing+1 {
        expansion.push(0);
    }

    BVector::new(dim, expansion.into_iter().rev().collect())
}


#[derive(Debug)]
#[derive(Eq)]
#[derive(Clone)]
struct BVector {
    dim: u16,
    elements: Vec<i8>
}


impl BVector {
    pub fn new(dim: u16, elements: Vec<i8>) -> BVector {
        BVector {
            dim,
            elements
        }
    }
}

impl ops::Add<BVector> for BVector {

    type Output = BVector;

    fn add(self, _rhs: BVector) -> BVector {

        if self.dim != _rhs.dim {
            panic!("Incorrect sizes! {} and {}", self.dim, _rhs.dim);
        }

        let mut new_elements = vec![0;self.dim as usize];

        for i in 0usize..self.dim as usize {
            new_elements[i] = (self.elements[i] + _rhs.elements[i]) % 2
        }

        BVector::new(self.dim, new_elements)
    }
}


impl PartialEq for BVector {
    fn eq(&self, other: &Self) -> bool {
        (self.dim == other.dim) && (self.elements == other.elements)
    }
}


// This is needed for hashing, and I need two vectors with the same elements and the same dimensions to have the same hash
// as I'm planning to use hashmaps
impl Hash for BVector {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.dim.hash(state);
        self.elements.hash(state);
    }
}
