mod pb;

fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_url() {
        use std::borrow::Borrow;
        let spec1 = Spec::new_resize(100, 100);
    }
}
