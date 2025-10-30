use citrine::{parse, tokenize};

fn main() {
    let input = "(defn factorial [n]
  (if (= n 0)
    1
    (* n (factorial (- n 1)))))";

    println!("Input: {}", input);
    
    // Tokenize the input
    let tokens = tokenize(input);
    println!("\nTokens:");
    for token in &tokens {
        println!("{:?}", token);
    }
    
    // Parse the input
    let syntax = parse(input);
    println!("\nSyntax Tree:");
    println!("{:#?}", syntax);
}

