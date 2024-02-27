

fn bigger_than_one(x: u64) -> bool {

    if x > 1 {
        return true
    }

    false

}

fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod tests {
    use crate::bigger_than_one;

    #[test]
    fn it_works() {

        assert_eq!(true, bigger_than_one(2));
    }

    #[test]
    fn it_doesnt_works() {

        assert_eq!(false, bigger_than_one(0));
    }
}