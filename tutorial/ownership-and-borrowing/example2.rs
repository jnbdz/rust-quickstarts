#[allow(unused_variables)]
fn main() {
        let stack_f64: f64 = 1.;
        let mut heap_f64: Box<f64> = Box::new(2.); // `mut` is used because we don't want to clone. And want to reuses the same variable.

        stack_procedure(stack_f64); // By being passed to the function it copies it in the memory creating a new owner
        println!("In main stack {}", stack_f64); // This var uses a the new memory (not the first one)

        // Like `stack` it is tries to create a new copie. So it needs to be clone for heap. But clonign is very expensive.
        // The second way (better for memory and performance) is to return the value from the function and pass it from the function.
        heap_f64 = heap_procedure(heap_f64);
        println!("In main heap {}", heap_f64);
} // The vars will cleanup at this point of exec.

fn stack_procedure(mut param: f64) {
        param += 9.;
        println!("In stack_procedure with param {}", param);
}

fn heap_procedure(param: Box<f64>) -> Box<f64> {
        println!("In heap_procedure with param {}", param);

        param
}
