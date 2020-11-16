use mylib;

pub fn person_print() {
    println!("person_print called");
    mylib::mylib_print();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_person_print() {
        person_print();
    }
}
