struct GeigerCounter {
    count: Option<u32>
}

fn main() {
    let counter = GeigerCounter { count: None };
    let mut count = 0;

    for _ in 0..10 {
        count += match counter.count{
            Some(value) => value,
            None => 0
        };
    }

    for _ in 0..10 {
        count += counter.count.unwrap_or(0);
    }

    println!("Count: {:?}", count);
}
