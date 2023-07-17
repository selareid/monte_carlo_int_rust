use rand;
use rand::distributions::{Distribution, Uniform};
use std::thread;
use std::sync::{Arc, Mutex};


fn main() {
	println!("Starting");

	let a: Box<dyn Fn(f64) -> f64 + 'static + Send + Sync> = Box::new(|x| x * x);
	let b: Box<dyn Fn(f64) -> f64 + 'static + Send + Sync> = Box::new(|x| x * x);

	//let integral = integrate(|x| x * x, -2.0, 2.0);
	let integral: f64 = integrate_threads(move |x| a(x)*b(x), 0.0, 1.0);

	println!("Integral: {:.4}", integral);
}




const N: u32 = 9999999;

fn integrate<F>(func: F, a: f64, b: f64) -> f64
	where F: Fn(f64) -> f64 {

	let between = Uniform::new_inclusive(a, b);
	let mut rng = rand::thread_rng();
	let mut sum: f64 = 0_f64;

	for _ in 0..N {
		let x: f64 = between.sample(&mut rng);

		let y: f64 = func(x);

		//println!("x: {}, y: {}", x, y);

		sum += y;
	}
	
	let int: f64 = sum * (a - b).abs() / (N as f64);
	
	//println!("sum: {}, int: {}", sum, int);

	return int;
}



const NUMBER_OF_THREADS: u32 = 12;
const N_PER_THREAD: u32 = 999999;
fn integrate_threads<F>(func: F, a: f64, b: f64) -> f64 
	where F: Fn(f64) -> f64 + 'static + Send + Sync {
	let between = Uniform::new_inclusive(a, b);
	
	let sum: Arc<Mutex<f64>> = Arc::new(Mutex::new(0_f64));
	let mut threads: Vec<_> = Vec::new();

	let arc_func = Arc::new(func);

	for _ in 0..NUMBER_OF_THREADS {
		let func_clone: Arc<F> = Arc::clone(&arc_func);
		let cloned_sum = Arc::clone(&sum);

		let new_thread = thread::spawn(move || {
			let mut rng = rand::thread_rng();
			let mut thread_sum: f64 = 0_f64;
			
			for _ in 0..N_PER_THREAD {
				let x: f64 = between.sample(&mut rng);
				let y: f64 = func_clone(x);
				thread_sum += y;
			}

			let mut whole_sum = cloned_sum.lock().unwrap();
			*whole_sum += thread_sum;
		});

		threads.push(new_thread);
	}

	for t in threads {
		let res = t.join();
		if let Err(_) = res {
			eprintln!("Thread panicked!");
		}
	}

	let integral: f64 = (a-b).abs() * *sum.lock().unwrap() / (NUMBER_OF_THREADS * N_PER_THREAD) as f64;

	return integral;
}
