


fn main() {

    let s = "    hi there   ";

    let p = s.trim();

    // Converts to string
    // A String is simply a str in a box pointer
    let p = p.to_string();

    println!("p == {}", p);


    let f_text = string_find_f("spot the f");

    println!("trimmed the string: {}", f_text);

    let x = choose_str(1);

    println!("{}", x);

}

// use static 
fn string_find_f(s: &str) -> &str {

    for (n,x) in s.char_indices(){
        if x == 'f'{
            return &s[n..];
        }
    }

    ""
}

// Use static for str with no references
fn choose_str(n: i32) -> &'static str {

    match n {
        0 => "hello",
        1 => "goodbye",
        _ => "other"
    }
}