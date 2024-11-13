use rand::distributions::{ Distribution, Uniform };

const SAMPLE_SIZE: u32 = 10_000_000u32;

fn main ( ) {
    let mut rng = rand::thread_rng();
    let between = Uniform::from( -10000..10000 );

    println!("[ Beginning Calculations ]");

    // Calculate the number of zero occurences
    let mut total: u32 = 0;
    for _ in 0..SAMPLE_SIZE {
        let ri_1: i32 = between.sample ( &mut rng );
        let ri_2: i32 = between.sample ( &mut rng );
        let ri_3: i32 = between.sample ( &mut rng );
        let ri_4: i32 = between.sample ( &mut rng );

        let value: i32 = ( ri_1 - ri_2 ).pow(2) + 4 * ri_3 * ri_4; 
        if value <= 0 {
            total += 1;
        }
    }

    println!( "\n[ Done! ]");
    println!( "{0: <35} {1: >10}", "Total Zero Occurrences:",         total                                          );
    println!( "{0: <35} {1: >10}", "Sample Size:",                    SAMPLE_SIZE                                    );
    println!( "{0: <35} {1: >10}", "Percentage of Zero Occurrences:", (100 * total ) as f64 / ( SAMPLE_SIZE as f64 ) );
}
