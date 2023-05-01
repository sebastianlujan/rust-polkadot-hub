#[cfg(test)]
mod tests {
    #[test]
    fn test_primitive_types2() {
        // Characters (`char`)

        // Note the _single_ quotes, these are different from the double quotes
        // you've been seeing around.
        let my_first_initial = 'C';
        if my_first_initial.is_alphabetic() {
            println!("Alphabetical!");
        } else if my_first_initial.is_numeric() {
            println!("Numerical!");
        } else {
            println!("Neither alphabetic nor numeric!");
        }
    }
    #[test]
    #[allow(dead_code)]
    fn test_variables5() {
        let number = "T-H-R-E-E"; // don't change this line
        println!("Spell a Number : {}", number);
        let number = 3; // don't rename this variable
        println!("Number plus two is : {}", number + 2);
    }
}