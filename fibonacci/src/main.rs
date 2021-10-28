use std::io;

fn main() {
   println!("Enter the number of elements you want Fibonacci series for:");
   let mut n = String::new();
   let mut num1=0;
   let mut num2=1;
   io::stdin()
              .read_line(&mut n)
              .expect("Enter a valid response");
   let n: u32=n.trim().parse().expect("Enter a number");
   println!("The Fibonacci series is as follows:");
   println!("{}",num1);
   println!("{}",num2);
   for element in 1..n-1{
       let sum=num1+num2;
       println!("{}",sum);
       num1=num2;
       num2=sum;
   } 
   
}
