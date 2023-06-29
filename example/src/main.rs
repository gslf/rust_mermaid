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
        Err(error) => panic!("Ops, there is a problem!", error),
    }

    println!("Test diagram file creation:");
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
        Err(error) => panic!("Ops, there is a problem!", error),
    }

}
