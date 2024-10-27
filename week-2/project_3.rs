fn main () {
   let p:f64 = 210000.0;
   let r:f64 = 0.05;
   let t:f64 = 3.0;
   let n:f64 = 1.0;

   // depreciation
   let a = p * (1.0_f64 - (r / n)).powf(t);
   println!("Amount is {}", a);
   let d = p - a;
   println!("depreciarion is {}", d);

   }