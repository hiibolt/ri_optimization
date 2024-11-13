use rand::distributions::{ Distribution, Uniform };
use std::sync::{ Mutex, Arc };

fn main ( ) {
    let ( sample_size, lower_bound, upper_bound, threads ) = gen_config();

    println!("[ Configuration ]");
    println!("{0: <20} {1: >10}", "Sample Size: ", sample_size );
    println!("{0: <20} {1: >10}", "Lower Bound: ", lower_bound );
    println!("{0: <20} {1: >10}", "Upper Bound: ", upper_bound );
    println!("{0: <20} {1: >10}", "Threads: ",     threads     );

    println!("\n[ Beginning Calculations ]");

    // Start our clock
    let timer = std::time::Instant::now();

    // Build our thread handle array and thread-safe total counter
    let mut handles = Vec::new();
    let total = Arc::new(Mutex::new(0u128));

    // Launch our threads
    for _  in 1..=threads {
        // Start the thread
        let total_handle = total.clone();
        handles.push(std::thread::spawn( move || {
            let mut rng = rand::thread_rng();
            let between = Uniform::from( lower_bound..upper_bound );

            // Calculate the number of zero occurences
            let mut inner_total: u128 = 0;
            for _ in 0..( sample_size / threads ) {
                let ri_1: i128 = between.sample ( &mut rng );
                let ri_2: i128 = between.sample ( &mut rng );
                let ri_3: i128 = between.sample ( &mut rng );
                let ri_4: i128 = between.sample ( &mut rng );

                if ( ri_1 - ri_2 ).pow(2) <= -4 * ri_3 * ri_4 {
                    inner_total += 1;
                }
            }

            // Add it to the global total
            *(total_handle.lock().expect("Failed to lock global total!")) += inner_total;
        }));
    }

    println!("\n[ Successfully launched {threads} threads ]");

    for handle in handles {
        handle.join().expect("Thread join failed!");
    }

    let final_total = *(total.lock().expect("Failed to lock final value!"));

    println!( "\n[ Done in {0:?}! ]", timer.elapsed());
    println!( "{0: <35} {1: >10}", "Total Non-Positives:", final_total );
    println!( "{0: <34} {1: >10}%", "% Non-Positive:", (100 * final_total ) as f64 / ( sample_size as f64 ) );
}

fn gen_config ( ) -> ( u128, i128, i128, u128 ) {
    let args: Vec<String> = std::env::args()
        .collect();

    let sample_size = args.get(1)
        .unwrap_or(&String::from("10000000"))
        .parse::<u128>()
        .expect("First argument (sample size) must be an integer!");
    let lower_bound = args.get(2)
        .unwrap_or(&String::from("-10000"))
        .parse::<i128>()
        .expect("Second argument (lower bound) must be an integer!");
    let upper_bound = args.get(3)
        .unwrap_or(&String::from("10000"))
        .parse::<i128>()
        .expect("Third argument (upper bound) must be an integer!");
    let threads = args.get(4)
        .unwrap_or(&String::from("1"))
        .parse::<u128>()
        .expect("Fourth argument (thread count) must be an unsigned integer!");

    ( sample_size, lower_bound, upper_bound, threads )
}
