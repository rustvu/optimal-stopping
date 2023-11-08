use fastrand;

const N_SUITORS: usize = 100;

fn generate_suitors() -> [i32; N_SUITORS] {
    let mut suitors = [0; N_SUITORS];
    for i in 0..suitors.len() {
        suitors[i] = fastrand::i32(..);
    }
    suitors
}

fn pick_a_prince(suitors: [i32; N_SUITORS], n_explore: usize) -> i32 {
    assert!(suitors.len() >= n_explore);

    let mut bar = i32::MIN;
    for i in 0..suitors.len() {
        if i < n_explore {
            bar = if suitors[i] > bar { suitors[i] } else { bar };
        } else {
            if suitors[i] > bar {
                return suitors[i];
            }
        }
    }

    suitors[suitors.len() - 1]
}

fn main() {
    let mut optimum = (0, 0);  // (score, explore)

    for explore in 0..N_SUITORS {
        let n_experiments = 100_000;
        let mut score = 0;
        for _ in 0..n_experiments {
            let suitors = generate_suitors();
            let best_suitor = *suitors.iter().max().unwrap();
            let prince = pick_a_prince(suitors, explore);
            if prince == best_suitor {
                score += 1;
            }
        }
        if score > optimum.0 {
            optimum = (score, explore);
        }
    }

    println!("Optimal explore: {}", optimum.1);
}
