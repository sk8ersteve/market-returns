use math::mean;
use rand_distr::{Distribution, Normal};
use std::vec::Vec;

fn invest_fixed(principle: f64, years: u32, avg_return: f64) -> f64 {
    let mut money = principle;
    for _ in 0..years {
        money += money * avg_return;
    }
    money
}

fn invest_gaussian(principle: f64, years: u32, avg_return: f64, std: f64) -> (f64, f64, f64) {
    let mut rng = rand::thread_rng();
    let normal = Normal::new(avg_return, std).unwrap();
    let mut money = principle;
    let mut returns = Vec::new();
    for _ in 0..years {
        let this_return = normal.sample(&mut rng);
        money += money * this_return;
        returns.push(this_return + 1.0);
    }
    let arith_mean = mean::arithmetic(&returns) - 1.0;
    let geom_mean = mean::geometric(&returns) - 1.0;
    (money, arith_mean, geom_mean)
}

fn main() {
    let principle = 1_000.0;
    let years = 40;
    let avg_return = 0.1;
    let std = 0.15;
    println!("Initial investment: ${:.2}", principle);
    println!("Number of years: {}", years);
    println!("Average return: {:.2}%", avg_return * 100.0);
    println!("Standard deviation: {:.2}%", std * 100.0);
    println!("");

    println!("First let's make a prediction by simulating a guaranteed return with our average:");
    println!("========================================");
    {
        let money = invest_fixed(principle, years, avg_return);
        println!("Final investment: ${:.2}", money);
        println!("========================================");
        println!("");
    }

    let num_trials = 10;
    let mut averages = Vec::new();
    let mut geom_means = Vec::new();
    println!("Now let's try 10 trials with gaussian distributed returns using {:.2}% standard deviation.", std * 100.0);
    println!("We use geometric mean to determine the actual annual return that we are effectively getting.");
    println!("========================================");
    for i in 0..num_trials {
        println!("Trial {}", i + 1);
        let (money, arith_mean, geom_mean) = invest_gaussian(principle, years, avg_return, std);
        println!("Average return: {:.2}%", arith_mean * 100.0);
        println!("Geometric mean: {:.2}%", geom_mean * 100.0);
        println!("Final investment: ${:.2}", money);
        println!("========================================");
        averages.push(arith_mean);
        geom_means.push(geom_mean);
    }
    println!("");

    println!(
        "Average average: {:.2}%",
        mean::arithmetic(&averages) * 100.0
    );
    println!(
        "Average geometric mean: {:.2}%",
        mean::arithmetic(&geom_means) * 100.0
    );
    println!("");

    println!("Looking at these results, we may notice 2 things.");
    println!("1. Our effective return rate (geometric mean) is consistently lower than our \"average return\".");
    println!("2. Actual results can be much worse than the initial prediction if we happen to be very unlucky.");
    println!("");
}
