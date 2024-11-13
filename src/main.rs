use rand::distributions::{ Distribution, Uniform };
use std::sync::{ Mutex, Arc };

fn main ( ) {
    let ( sample_size, threads ) = gen_config();

    println!("[ Configuration ]");
    println!("{0: <20} {1: >10}", "Sample Size: ", sample_size );
    println!("{0: <20} {1: >10}", "Threads: ",     threads     );

    println!("\n[ Beginning Calculations ]");

    // Create an over-arching clock
    let global_timer = std::time::Instant::now();

    // Run each digit-set
    for digits in 2..=18 {
        // Start our clock
        let timer = std::time::Instant::now();

        // Create our bounds
        let upper_bound: i128 = 10i128.pow(digits);
        let lower_bound: i128 = -upper_bound;

        print!("[ Checking {digits} Digits ({0}..{1})... ", lower_bound, upper_bound);

        let final_total = gen_total(
            sample_size,
            lower_bound,
            upper_bound,
            threads
        );
    
        println!( " Done in {0:?}! ]", timer.elapsed());
        println!( "{0: <35} {1: >10}", "Total Non-Positives:", final_total );
        println!( "{0: <34} {1: >10}%\n", "% Non-Positive:", (100 * final_total ) as f64 / ( sample_size as f64 ) );
    }

    println!( "\n[ All Computations Done in {0:?}! ]", global_timer.elapsed());
}

fn gen_total (
    sample_size: u128,
    lower_bound: i128,
    upper_bound: i128,
    threads: u128
) -> u128 {
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

    for handle in handles {
        handle.join().expect("Thread join failed!");
    }

    let final_total = *(total.lock().expect("Failed to lock final value!"));

    final_total
}

fn gen_config ( ) -> ( u128, u128 ) {
    let args: Vec<String> = std::env::args()
        .collect();

    let sample_size = args.get(1)
        .unwrap_or(&String::from("10000000"))
        .parse::<u128>()
        .expect("First argument (sample size) must be an integer!");
    let threads = args.get(2)
        .unwrap_or(&String::from("1"))
        .parse::<u128>()
        .expect("Second argument (thread count) must be an unsigned integer!");

    ( sample_size, threads )
}
