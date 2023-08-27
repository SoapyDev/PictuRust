use crate::parameters::parameters::Parameters;

mod parameters;
mod picture;
mod runner;
fn main() {
    let params = Parameters::new();
    params.display();
    let runner = runner::Runner;
    runner.run(params);
}
