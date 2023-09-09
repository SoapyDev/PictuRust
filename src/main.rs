use crate::parameters::parameters::Parameters;

mod parameters;
mod picture;
mod runner;

fn main() {
    let params = Parameters::new_with_display();
    runner::Runner.run(params);
}
