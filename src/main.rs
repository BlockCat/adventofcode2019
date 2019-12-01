#![feature(test)]
extern crate hashbrown;
extern crate test;
extern crate utils;

macro_rules! run {
    ($($x:ident), *) => {
        $(
            #[allow(dead_code)]
            mod $x;
        )*
        
        fn main() {
            
        }
    }
}

run!(day_1);