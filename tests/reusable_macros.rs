#[cfg(test)]
#[allow(dead_code, unused_variables, unused_imports)]

mod tests {
    use std::io;

    #[test] 
    fn get_input() {
        let show = "What The Heck";
        println!("{show}");
        let mut buffer = String::new();
        let stdin = io::stdin();
    
        // Typing: Hello World
        if let Err(_) = stdin.read_line(&mut buffer) {
            println!("not a valid input, try again...")
        }

        assert_eq!(buffer.replace("\n", ""), "Hello World");
    }
}