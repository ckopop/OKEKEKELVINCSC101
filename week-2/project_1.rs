fn main () {
   let p:f64 = 520000000.0;
   let r:f64 = 0.1;
   let t:f64 = 5.0;
   let n:f64 = 1.0;

   // compound interest
   let a = p * (1.0_f64 + (r / n)).powf(t);
   println!("Amount is {}", a);
   let ci = a - p;
   println!("compound interest is {}", ci);

   }