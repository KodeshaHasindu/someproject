fn main() {
    println!("Hello, world!");

    another_function();
}

fn another_function() {
    println!("Another function.");
}









fn main() {
    another_function(5);
}
fn another_function(x:i32) {
    println!("The value of x is: {x}");
}








fn main() {
    print_labeled_measurement(5, 'h');
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}







fn main() {
    let y = {
        let x = 3;
        x + 1
    };
    
    println!("The value of y is: {y}")
}









fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("The value of x is: {x}")
}





fn main() {
    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}









fn main() {
    let number = 7;

    if number < 5{
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}







fn main() {
    let number = 3;

    if number !=  0 {
        println!("number was some thin other then zero")
    }
}








fn main() {
    let number = 5;

    if number % 4 == 0{
        println!("number is divisible by 4");
    } else if number % 3 == 0{
        println!("number is divisible by 3"); 
    } else if number % 2 == 0{
        println!("number is divisible by 2"); 
    } else {
        println!("number is not divisible by 4, 3, or 2 "); 
}
}














fn main() {
    let condition = true;
    let number = if condition {5} else {6};

    println!("The  value of number is: {number}");
}












fn main() {
    let mut counter = 0;

    let result = loop{
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}










fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
}







fn main() {
    let mut number = 10;

    while number != 0{
        println!("{number}!");
        number -= 1;
    }

    println!("LIFTOFF!!!");
}




fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("The value is: {}", a [index]);

        index += 1;
    }

}



fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}




fn main() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}








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







use std::io;

fn main() {
       println!("Please input number");

       let mut f = String::new();

 io::stdin()
       .read_line(&mut f)
       .expect("Failed to read line");


}









fn first_word(s: &String) -> &str {
        let bytes = s.as_bytes();
    
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
        &s[..]
    }
    
    fn main() {
        let mut s = String::from("hello world");
        let word = first_word(&s);
        // s.clear();
    
        println!("{word}")
    }







    fn second_word(s: &str) -> &str {
        let bytes = s.as_bytes();
    
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
        &s[..]
    }
    
    fn main() {
        let my_string = String::from("hello world");

        let word = second_word(&my_string[0..6]);
        let word = second_word(&my_string[..]);

        let word = second_word(&my_string);

        let my_string_literal = "hello world";

        let word = second_word(&my_string_literal[0..6]);
        let word = second_word(&my_string_literal[..]);
        
        let word = second_word(my_string_literal);
    
        println!("{word}")
    }





    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count:u64,
    }
    
    fn main() {
        let user1 = User {
            active: true,
            username: String::from("username12"),
            email:String::from("username@example.com"),
            sign_in_count: 1,
        
      
        };
    
        let s = user1.username;
        println!("{s}");
        
    }





  