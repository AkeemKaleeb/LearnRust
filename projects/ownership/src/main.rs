fn main() {
    let mut s = String::from("hello world");

    let hello: &str = &s[0..5];
    let world: &str = &s[6..11];
    let s2: &String = &s;


    let word = first_word(&s);
    let word_slice = first_slice(&s);

    s.clear();
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
