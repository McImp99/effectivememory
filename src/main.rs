use rand::Rng;
use std::time::Instant;
use std::collections::HashMap;

struct Sensor {
    id: u32,
    value: f32,
}


fn main() {
    hashmap_test();
    let n = 100_000;

    let start = Instant::now();

    let mut sensors: Vec<Sensor> = Vec::with_capacity(n);


    for i in 0..n {
        sensors.push(Sensor {
            id: i as u32,
            value: rand::random::<f32>(),
        });
    }

    let creation_time = start.elapsed();

    let sum_start = Instant::now();
    let sum: f32 = sensors.iter().map(|s| s.value).sum();
    let sum_time = sum_start.elapsed();

    
    println!("Rust runtime:");
    println!("Creation: {} ms", creation_time.as_millis());
    println!("Summing: {} ms", sum_time.as_millis());
    println!("Total sum: {}", sum);
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
