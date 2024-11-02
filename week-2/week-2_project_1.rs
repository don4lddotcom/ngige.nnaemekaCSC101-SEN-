fn main() {
	let p:f64 =  520_000_000.0;
	let r:f64 = 10.0;
	let n = 5;

	let product = (1.0 + (r / 100.0 )).powf(n as f64);
	let a = p * product;
	let ci = a - p;

	println!("amount is {}",a);
	println!("compound interest is {}",ci);

}