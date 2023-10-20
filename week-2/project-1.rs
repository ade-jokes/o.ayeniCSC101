fn main() {
	let p:f64 = 5200000.0;
	let r:f64 = 10.0;
	let t:f64 = 5.0;
	let h:f64 = r/100.0;
	
	//simple intrest calculations
	   //Amount
	let a = p * (1.0 + h)* t;
	println!("Amount is {}",a);
	   //Simple interest
	   let si = a - p;
	println!("Amount is {}",si);
}