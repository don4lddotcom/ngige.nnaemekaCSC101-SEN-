fn main() {
	let p :f64 = 510_000.0;
	let r :f64 = 5.0;
	let n = 3;

	let final_value = p * (1.0 - (r / 100.0)).powf(n as f64);

	println!("The value of the tv after {} years is N{:.2}",n, final_value);
}