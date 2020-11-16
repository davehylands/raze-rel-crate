pub fn mylib_print() {
    println!("mylib_print called");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mylib_print() {
        mylib_print();
    }
}
