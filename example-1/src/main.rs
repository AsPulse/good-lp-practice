use std::error::Error;

use good_lp::{variables, variable, default_solver, SolverModel, constraint, Solution};

fn main() -> Result<(), Box<dyn Error>> {
    let mut vars = variables!();
    let x = vars.add(variable().min(0));
    let y = vars.add(variable().min(0));

    let solution = vars.maximise(x + y)
        .using(default_solver)
        .with(constraint!(3 * x - y <= 4)) // 制約の追加
        .with(constraint!(x + 2 * y <= 6))
        .with(constraint!(x + 6 * y <= 16))
        .solve()?;

    println!("x + y = {}", solution.model().obj_value());
    println!("x={}, y={}", solution.value(x), solution.value(y));

    Ok(())
}
