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
};
use std::cell::{
    RefCell,
};
use std::io::{
    Write,
};


fn output_bytes<T: Rng + SeedableRng>(rng: &RefCell<T>, n: usize) -> Result<usize, io::Error> {
    let buf: &mut [u8] = &mut vec![0u8; n][..];
    rng.borrow_mut().fill_bytes(buf);
    let num_written = io::stdout().write(&buf)?;
    Ok(num_written)
}


fn main() {
    let arg_defs = App::new("psw: prng-stdio-writer")
                        .version("0.2.0")
                        .about("Writes output of pcg64 to stdout.")
                        .arg(Arg::with_name("bytes")
                             .short("b")
                             .long("bytes")
                             .takes_value(true)
                             .help("Total number of bytes to write."))
                        .arg(Arg::with_name("bufsize")
                             .short("s")
                             .long("bufsize")
                             .takes_value(true)
                             .help("Size in bytes of the buffer for each write. Default is 1024."))
                        .get_matches();

    let bytes_exist = arg_defs.value_of("bytes");
   
    //TODO Possibly add more generators.
    thread_local!{
        static RNG: RefCell<rand_pcg::Lcg128Xsl64> = RefCell::new(rand_pcg::Lcg128Xsl64::from_entropy());
    };

    let buffer_size = match arg_defs.value_of("bufsize") {
        Some(buffer_size_as_string) => {
            buffer_size_as_string.trim().parse::<usize>().expect("Failed to understand supplied value of bufsize")
        },
        None => 1024,
    };

    match bytes_exist {
        Some(bytes_as_string) => {
            // This should be able to handle an unreasonable number of bytes.
            let num_bytes = bytes_as_string.trim().parse::<BigInt>().expect("Failed to understand supplied value of bytes");
            let max_loop = BigInt::from_usize(usize::MAX).unwrap();
            let num_iter = &num_bytes / (&max_loop * BigInt::from_usize(buffer_size).unwrap());
            let remaining_bytes = &num_bytes % (&max_loop * BigInt::from_usize(buffer_size).unwrap());
            let num_iter_remaining = &remaining_bytes / BigInt::from_usize(buffer_size).unwrap();
            let remainder = &remaining_bytes % BigInt::from_usize(buffer_size).unwrap();
            for _ in 0..(num_iter.to_usize().unwrap()) {
                for _ in 0..(max_loop.to_usize().unwrap()) {
                    RNG.with(|rng| {
                        output_bytes(rng, buffer_size).expect("Failed to write to stdout");
                    });
                }
            }
            for _ in 0..(num_iter_remaining.to_usize().unwrap()) {
                RNG.with(|rng| {
                    output_bytes(rng, buffer_size).expect("Failed to write to stdout");
                });
            }
            RNG.with(|rng| {
                output_bytes(rng, remainder.to_usize().unwrap()).expect("Failed to write to stdout");
            });
        },
        None => {
            loop {
                match RNG.with(|rng| output_bytes(rng, buffer_size)) {
                    Ok(_) => (),
                    Err(_) => break,
                }
            }
        },
    }
}
