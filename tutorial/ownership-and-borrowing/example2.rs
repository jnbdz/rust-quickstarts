#[allow(unused_variables)]
fn main() {
        let stack_f64: f64 = 1.;
        let heap_f64: Box<f64> = Box::new(2.);

        stack_procedure(stack_f64);
        println!("In main stack {}", stack_f64);
} // The vars will cleanup at this point of exec.

fn stack_procedure(param: f64) {
        println!("In stack_procedure with param {}", param);
}
