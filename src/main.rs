fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use acc::lib::add_two;


    #[test]
    fn test_nothing() {
        let x = add_two(1, 2);
        assert_eq!(x, 3);
        return;
    }
}