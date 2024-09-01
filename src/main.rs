//! Modeling the [Optimal Stopping problem](https://en.wikipedia.org/wiki/Optimal_stopping)
//! Note: this code is not optimal nor most idiomatic Rust. The motivation is to
//! use basic language concepts for a non-trivial example early in the class.
use fastrand;

const N_SUITORS: usize = 100;

/// Suitors are represented as random integer numbers (full range)
/// where a higher numbers mean more desirable suitor
fn generate_suitors() -> [i32; N_SUITORS] {
    let mut suitors = [0; N_SUITORS];
    for i in 0..suitors.len() {
        suitors[i] = fastrand::i32(..);
    }
    suitors
}

/// Pick a prince (best suitor) using a simple exploration/exploitation strategy
/// The `n_explore` parameter controls the exploration size
fn pick_a_prince(suitors: [i32; N_SUITORS], n_explore: usize) -> i32 {
    assert!(suitors.len() > 0);
    assert!(suitors.len() >= n_explore);

    let mut bar = i32::MIN;
    for i in 0..suitors.len() {
        if i < n_explore {
            bar = if suitors[i] > bar { suitors[i] } else { bar };
        } else {
            if suitors[i] >= bar {
                return suitors[i];
            }
        }
    }

    suitors[suitors.len() - 1]
}

fn main() {
    let mut optimum = (0, 0); // (score, explore)
    let mut scores = vec![];

    for n_explore in 0..N_SUITORS {
        let n_experiments = 100_000;
        let mut score = 0;
        for _ in 0..n_experiments {
            let suitors = generate_suitors();
            let best_suitor = *suitors.iter().max().unwrap();
            let prince = pick_a_prince(suitors, n_explore);

            // The goal is to pick the absolute best suitor
            if prince == best_suitor {
                score += 1;
            }
        }
        scores.push(score);
        if score > optimum.0 {
            optimum = (score, n_explore);
        }
    }

    println!("Optimal exploration threshold: {}", optimum.1);

    // Optional Plotting
    #[cfg(feature = "plotly")]
    {
        use plotly::{layout::Axis, Layout, Plot, Scatter};

        let mut plot = Plot::new();
        let trace = Scatter::new((0..scores.len()).collect(), scores);
        plot.add_trace(trace);

        let layout = Layout::new()
            //.width(800)
            .height(600)
            .title("Optimal Stopping")
            .x_axis(Axis::new().title("n_explore"))
            .y_axis(Axis::new().title("score"));
        plot.set_layout(layout);
        plot.show();
    }
}
