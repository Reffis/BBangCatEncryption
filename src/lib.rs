mod converter;

mod bce {
    pub mod to_bce {
        use crate::converter::get_key_to_bce;

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
        use crate::converter::get_key_to_str;

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
        use crate::bce::{to_str, to_bce};
        assert_eq!(to_bce::new("a b c d e f g h i j k l m n o p q r s t u v w x y z A B C D E F G H I J K L M N O P Q R S T U V W X Y Z"),
                   to_bce::new(to_str::new("빵 뽷 삫 뺗 빽 뻭 뺶 뻏 켓 캣 컛 콋 켔 콌 컜 뽕 뿡 뿅 뺭 뺑 쁑 뺭 삉 쀵 뽱 뼝 뿾 쀇 뽺 뾹 뾂 쁅 뾾 뿢 쁥 뾹 쀇 뾳 뼿 뾎 뽬 쾫 켥 캛 컎 켦 캺 켎 쾛 컱 콇 켈").as_str()));
    }
}
