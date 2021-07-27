# **BBang Cat Encryption**

빵켓 암호화입니다.

간단하게 만들어봤습니다.

## **사용법**

`Cargo.toml`:
```toml
[dependencies]
bbangcat_encryption = "0.1.1"
```

`main.rs`:
```rust
use bbangcat_encryption::bce;

fn main() {
    println!("{}", bce::to_bce::new("Hello, World!")); // 뿢빽콋콋컜랰 쾛컜뿅콋뺗렾
    println!("{}", bce::to_str::new("뿢빽콋콋컜랰 쾛컜뿅콋뺗렾")); // Hello, World!
}

```

