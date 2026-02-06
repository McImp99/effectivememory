use rand::Rng;
use std::time::Instant;
use std::collections::HashMap;

struct BigSensor {
    id: u32,
    value: [f32; 1000],
}


fn main() {
    let n = 1_000_000;

    let start = Instant::now();

    let mut sensors: Vec<Box<BigSensor>> = Vec::with_capacity(n);


    for i in 0..n {
        sensors.push(Box::new(BigSensor {
    id: i as u32,
    value: [0.0; 1000],
}));
    }

    let duration = start.elapsed();

    
    println!("Runtime: {} ms", duration.as_millis());
}

fn hashmap_test() {
    let n = 1_000_000;

    let start = Instant::now();

    let mut map = HashMap::with_capacity(n);

    for i in 0..n {
        map.insert(i, i as f32);
    }

    let lookup_start = Instant::now();

    let _ = map.get(&500_000);

    println!("Lookup time: {} ns", lookup_start.elapsed().as_nanos());
    println!("Total build time: {} ms", start.elapsed().as_millis());
}
