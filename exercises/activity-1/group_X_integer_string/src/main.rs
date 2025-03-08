fn main() {
    let orig_num = 123;
    
    let str_num = orig_num.to_string();
    println!("{}", str_num);

    let int_num = str_num.parse::<u32>().unwrap();
    println!("{}", int_num);
}
