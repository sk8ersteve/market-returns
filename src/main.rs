use rand_distr::{Distribution, Normal};

fn invest_fixed(principle: f64, years: u32, avg_return: f64) -> f64 {
    let mut money =  principle;
    for _ in 0..years {
        money += money * avg_return;
    }
    return money;
}

fn invest_gaussian(principle: f64, years: u32, avg_return: f64, stdev: f64) -> (f64, f64, f64) {
    let mut rng = rand::thread_rng();
    let normal = Normal::new(avg_return, stdev).unwrap();
    let mut money =  principle;
    let mut sum_of_returns = 0.0;
    for _ in 0..years {
        let this_return = normal.sample(&mut rng);
        money += money * this_return;
        sum_of_returns += this_return;
    }
    let this_average = sum_of_returns / (years as f64);
    let interest_equivalent = f64::powf(money / principle, 1.0 / (years as f64)) - 1.0;
    return (money, this_average, interest_equivalent)
}

fn main() {
    let principle = 1_000.0;
    let years = 40;
    let avg_return = 0.1;
    println!("Initial investment: ${:.2}", principle);
    println!("Number of years: {}", years);
    println!("Average return: {:.2}%", avg_return * 100.0);
    println!("");

    println!("First let's simulate this investment with no variance, like a fixed interest");
    println!("========================================");
    {
        let money = invest_fixed(principle, years, avg_return);
        println!("Final investment: ${:.2}", money);
        println!("========================================");
        println!("");
    }

    let num_trials = 10;
    let mut sum_of_avgs = 0.0;
    let mut sum_of_interests = 0.0;
    println!("Now let's try 10 trials with gaussian distributed returns using 15% standard deviation");
    println!("========================================");
    for i in 0..num_trials {
        println!("Trial {}", i + 1);
        let (money, actual_avg, interest) = invest_gaussian(principle, years, avg_return, 0.15);
        println!("Actual average: {:.2}%", actual_avg * 100.0);
        println!("Interest equivalent: {:.2}%", interest * 100.0);
        println!("Final investment: ${:.2}", money);
        println!("========================================");
        sum_of_avgs += actual_avg;
        sum_of_interests += interest;
    }
    println!("");

    println!("Average average: {:.2}%", sum_of_avgs / (num_trials as f64) * 100.0);
    println!("Average interest: {:.2}%", sum_of_interests / (num_trials as f64) * 100.0);
    println!("");

    println!("Looking at these results, we may notice 2 things.");
    println!("1. Our effective interest is consistently lower than our \"average return\" by around a whole %");
    println!("2. While our fixed return yields ~$45K in savings after 4 years, if we happen to be unlucky, our actual results can be far worse");
    println!("");
    println!("In all our trials, however, we still end up with far more than our initial $1,000");
}
