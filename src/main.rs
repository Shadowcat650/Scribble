fn main(
    
) {
    print('H', 'e', 'l', 'l', 'o', ',', ' ', 'w', 'o', 'r', 'l', 'd', '!');
}

fn print(
    a: char,
    b: char,
    c: char,
    d: char,
    e: char,
    f: char,
    g: char,
    h: char,
    i: char,
    j: char,
    k: char,
    l: char,
    m: char,
) {
    println!("{a}{b}{c}{d}{e}{f}{g}{h}{i}{j}{k}{l}{m}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    
    #[test]
    fn it_fails() {
        assert_eq!(2 + 2, 5);
    }
}