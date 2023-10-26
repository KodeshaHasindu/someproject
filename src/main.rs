use std::io;

fn main() {
 
 println!("Please input Fahrenheit.");

 let mut f = String::new();

 io::stdin()
        .read_line(&mut f)
        .expect("Failed to read line");

        let f: f32 =  f.trim().parse().expect("msg");

    

 let c: f32= (f - 32.0) * 5.0/9.0 ;

 println!("{:.2}",c);



}


