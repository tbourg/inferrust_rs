use inferrust::inferray::*;
use rand::*;

fn main() {
    let entropies = [0.1];
    let sizes = [10, 50, 100, 500, 1000, 10000, 1000000];
    for ent in &entropies {
        for size in &sizes {
            let mut rng = dbg!(dbg!(dbg!(*ent) * dbg!(-*size as f64)) + *size as f64);
            if rng == 0. {
                rng = 1.;
            }
            // println!("{}", rng);
            for _ in 0..5 {
                let mut values = vec![];
                for _ in 0..*size {
                    let a = rand::thread_rng().gen_range(1, rng as u64 + 1);
                    let b = rand::thread_rng().gen_range(1, rng as u64 + 1);
                    values.push([a, b]);
                }
                let t = time::precise_time_ns();
                bucket_sort_pairs(&mut values);
                let time = time::precise_time_ns() - t;
                println!("rust,{},{},{}", *size, *ent, time);
            }
        }
    }
}
