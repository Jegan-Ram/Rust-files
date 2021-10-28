use std::io;

fn celsius(x: f64){
   let y: f64= (x-32 as f64)*(5 as f64/9 as f64);
   println!("The Celsius value ={} Degree Celsius",y);  
}

fn fahrenheit(x: f64){
   let y: f64= (x*(9 as f64/5 as f64))+32 as f64;
   println!("The fahrenheit value ={} Degree Fahrenheit",y);  
}

fn main() {
    let mut choice = String::new();
    println!("Choose what you want to do!");
    println!("1. Celsius --> Fahrenheit");
    println!("2. Fahrenheit --> Celsius");
    io::stdin()
           .read_line(&mut choice)
           .expect("Invalid Input!!");
    let choice: u32= choice.trim().parse().expect("Unreadable Input");       
    if choice == 1{
       let mut celsius = String::new();
       println!("Enter the temperature in Celsius");
       io::stdin()
            .read_line(&mut celsius)
            .expect("Invalid Input!!"); 
       let celsius: f64= celsius.trim().parse().expect("Unreadable Input");      
       fahrenheit(celsius);  
    }
    else if choice == 2{
       let mut fahrenheit = String::new();
       println!("Enter the temperature in fahrenheit");
       io::stdin()
            .read_line(&mut fahrenheit)
            .expect("Invalid Input!!"); 
       let fahrenheit: f64= fahrenheit.trim().parse().expect("Unreadable Input");       
       celsius(fahrenheit);  
    }
    else{
       println!("The Choice you entered was wrong!!!"); 
    } 
}
