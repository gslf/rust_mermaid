# Rust Mermaid
### Generate mermaid.js diagrams from Rust

---
#### :#/ GSLF
#### [https://gslf.tech](https://www.gslf.tech)

---

![rust_mermaid logo](https://github.com/gslf/rust_mermaid/blob/8359a26b2b2ffb05a8a81781470505c6106594e0/res/rust_mermaid.jpg)

### Add rust_mermaid
```toml
[dependencies]
rust_mermaid = "0.1.1"
```

### How to use rust_mermaid
```rust
use rust_mermaid::create_diagram;
use rust_mermaid::save_diagram;

fn main() {
    println!("Test diagram creation:");
    let description = "graph LR;
        A--> B & C & D;
        B--> A & E;
        C--> A & E;
        D--> A & E;
        E--> B & C & D;
    ";
    let result = create_diagram(description);
    
    match result{
        Ok(_) =>  println!("It's worked!"),
        Err(error) => panic!("Ops, there is a problem! {}", error),
    }

    println!("Test diagram file creation:");
    let description = "graph LR;
        A--> B & C & D;
        B--> A & E;
        C--> A & E;
        D--> A & E;
        E--> B & C & D;
    ";
    let result = save_diagram(description, "test.jpg");
    
    match result{
        Ok(_) =>  println!("It's worked!"),
        Err(error) => panic!("Ops, there is a problem! {}", error),
    }

}




```