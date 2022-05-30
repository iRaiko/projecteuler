mod exercises;

fn main() 
{
    use exercises::*;    
    let start = std::time::Instant::now();
    println!("{:?}", e1_multiples_of::sum_below(100, &[3,5]));    
    println!("{:?}", start.elapsed());
}