fn main() {
	let p:f32 = 520000.0;
	let r:f32 = 10.0;
	let t:f32 = 5.0;
	let h:f32 = r/100.0;
	
	//simple intrest calculations
	   //Amount
	let a = p * (1.0 * h)* t;
	println!("Amount is {}",a);
	   //Simple interest
	   let si = a - p;
	println!("Amount is {}",si);
}