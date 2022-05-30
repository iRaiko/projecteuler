mod exercises;

fn main() 
{
    use exercises::*;    
    let start = std::time::Instant::now();
    println!("{:?}", e5_smallest_multiple::smallest_multiple());
    println!("{:?}", start.elapsed());
}