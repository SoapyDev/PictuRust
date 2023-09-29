use crate::parameters::parameters::Parameters;

pub mod parameters;
pub mod picture;
pub mod runner;

fn main() {
    let params = Parameters::new_with_display();
    runner::Runner.run(params);
}
