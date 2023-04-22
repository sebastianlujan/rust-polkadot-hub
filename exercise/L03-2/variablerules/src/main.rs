fn reglas_referencia(x: &mut String, y: &mut String) -> String {
    let mut nuevo_string = String::from(x.as_str());
    nuevo_string.push_str(y.as_str());
    nuevo_string
}

 
 

fn main() {

    #[allow(dead_code)]   
    let closure_sume = | x : &mut i64| -> i64 { *x = *x * 2; *x };

    #[allow(dead_code)]   
    let closure_string =  |x : &String| -> String {
        let mut y = String::from(x.as_str());
        y.push_str(" -AGREGADO- ");
        y
    };
 

    let mut variable = 10_i64;
    println!("{}",closure_sume(&mut variable));
    println!("{}",variable);

    let mut base_string = String::from("REFERENCIA MUTABLE STRING");
    let mut base_string2 = String::from("REFERENCIA MUTABLE STRING");
    let otro_string = reglas_referencia(&mut base_string,&mut base_string2);
    println!("{}",otro_string);
}