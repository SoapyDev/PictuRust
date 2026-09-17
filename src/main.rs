use picturust::parameters::parameters::Parameters;
use picturust::runner;

fn main() {
    let params = Parameters::new_with_display();
    runner::Runner.run(params);
}
