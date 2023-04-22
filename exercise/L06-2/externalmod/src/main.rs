fn function() {
    println!("called `function()`");
}

mod my2;

fn main() {
    my2::function();
    function();
    my2::indirect_access();
    my2::nested::function();
}