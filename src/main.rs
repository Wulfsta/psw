extern crate clap;
extern crate num;
extern crate rand;
extern crate rand_pcg;

use clap::{
    Arg, 
    App, 
};
use num::{
    FromPrimitive,
    ToPrimitive,
};
use num::bigint::{
    BigInt,
};
use rand::{
    Rng,
    SeedableRng,
};
use std::{
    io,
    mem,
};
use std::fmt::{
    Debug,
};
use std::io::{
    Write,
};

#[derive(Clone, Debug)]
struct RngContainer<T: Clone + Debug + Rng + SeedableRng> {
    rng: T,
}

impl<T: Clone + Debug + Rng + SeedableRng> RngContainer<T> {
    fn new(rng_in: T) -> RngContainer<T> {
        RngContainer { rng: rng_in }
    }

    fn output_bytes(&mut self) -> Result<usize, io::Error> {
        io::stdout().write(&self.rng.gen::<usize>().to_ne_bytes())
    }

    fn output_less_than_usize_bytes(&mut self, n: usize) -> Result<usize, io::Error> {
        io::stdout().write(&self.rng.gen::<usize>().to_ne_bytes()[0..n])
    }
}

fn main() {
    let arg_defs = App::new("psw: prng-stdio-writer")
                        .version("0.1.2")
                        .about("Writes output of pcg64 to stdout.")
                        .arg(Arg::with_name("bytes")
                             .short("b")
                             .long("bytes")
                             .takes_value(true)
                             .help("Number of bytes to write."))
                        .get_matches();

    let bytes_exist = arg_defs.value_of("bytes");
   
    //TODO Possibly add more generators.
    let rng = rand_pcg::Lcg128Xsl64::from_entropy();
    let mut rng_container = RngContainer::new(rng);

    match bytes_exist {
        Some(bytes_as_string) => {
            // This should be able to handle an unreasonable number of bytes.
            let num_bytes = bytes_as_string.trim().parse::<BigInt>().expect("Failed to understand supplied value of bytes to write");
            let max_loop = BigInt::from_usize(usize::MAX).unwrap();
            let num_iter = &num_bytes / (&max_loop * BigInt::from_usize(mem::size_of::<usize>()).unwrap());
            let remaining_bytes = &num_bytes % (&max_loop * BigInt::from_usize(mem::size_of::<usize>()).unwrap());
            let num_iter_remaining = &remaining_bytes / BigInt::from_usize(mem::size_of::<usize>()).unwrap();
            let remainder = &remaining_bytes % BigInt::from_usize(mem::size_of::<usize>()).unwrap();
            for _ in 0..(num_iter.to_usize().unwrap()) {
                for _ in 0..(max_loop.to_usize().unwrap()) {
                    rng_container.output_bytes().expect("Failed to write to stdout");
                }
            }
            for _ in 0..(num_iter_remaining.to_usize().unwrap()) {
                rng_container.output_bytes().expect("Failed to write to stdout");
            }
            rng_container.output_less_than_usize_bytes(remainder.to_usize().unwrap()).expect("Failed to write to stdout");
        },
        None => {
            loop {
                match rng_container.output_bytes() {
                    Ok(_) => (),
                    Err(_) => break,
                }
            }
        }
    }
}
