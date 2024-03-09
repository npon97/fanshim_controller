use std::f64;

fn main(){
    let mut i = 2.0;
    loop{
        i = f64::powi(i, 2);
        println!("{}", i);
    }
}