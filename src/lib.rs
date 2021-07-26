mod converter;

mod bce {
    pub mod to_bce {
        use crate::converter::{get_key_to_bce};

        pub fn new(text: &str) -> String {
            let mut result = String::new();
            for i in text.split("").collect::<Vec<&str>>() {
                match get_key_to_bce(i) {
                    Some(e) => result.push_str(e),
                    None => result.push_str(i),
                }
            }
            result
        }
    }

    pub mod to_str {
        use crate::converter::{get_key_to_str};

        pub fn new(text: &str) -> String {
            let mut result = String::new();
            for i in text.split("").collect::<Vec<&str>>() {
                match get_key_to_str(i) {
                    Some(e) => result.push_str(e),
                    None => result.push_str(i),
                }
            }
            result
        }
    }
}
    mod test {
        #[test]
        fn test() {
            println!("{}", crate::bce::to_bce::new("hello, world!"));
            println!("{}", crate::bce::to_str::new("빵뺭뺗뻭뿅뿡캣캤컜"));
        }
    }