fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn check_a() {
        panic!("a failed");
    }

    #[test]
    fn check_b() {
        panic!("a failed");
    }

    #[test]
    fn check_c() {
        panic!("c failed");
    }
}
