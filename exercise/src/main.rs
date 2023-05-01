mod utils {
    pub fn fizz_buzz() {
        for n in 1..=100 {
            if n % 3 == 0 {
                println!("fizz");
            } else if n % 5 == 0 {
                println!("buzz");
            } else if n % 15 == 0 {
                println!("fizzbuzz");
            } else {
                println!("{}", n)
            }
        }
    }
    
    pub fn iterator() {
        let arr = &[ 10, 20, 30, 40, 50];
        for elem in arr.iter() {
            println!("the value is: {}", elem)
        }
    }

    pub fn println_hello_world() {
        println!("Hello, world!");
    }

}

fn main() {
    utils::println_hello_world();
    utils::fizz_buzz();
    utils::iterator();
}