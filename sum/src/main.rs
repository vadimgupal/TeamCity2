fn main() {
println!("{}",add_numbers(2,4));
}


fn add_numbers(a: i32, b: i32) -> i32 {
a + b
}

#[cfg(test)]
mod tests {

    use crate::add_numbers;

    #[test]
    fn test1() {
        assert_eq!(add_numbers(2,4),6);
        assert_eq!(add_numbers(-1,-2),-3);
    }
   #[test]
   fn test2(){
     assert_eq!(add_numbers(-1,2),1);
    }
   #[test]
   fn test3(){
     assert_eq!(add_numbers(2,0),2);
    }
}