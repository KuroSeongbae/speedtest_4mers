fn main() {
    println!("Start");

    let opt = ["A", "C", "G", "T"];
    let mut s = String::new();
    let mut s_last = String::new();
    let len_str: u32 = 13;
    let change_next: bool;

    for i in 0..len_str {
        s += opt[0];
    }

    for i in 0..len_str {
        s += opt[opt.len()-1];
    }

    let pos: i32 = 0;
    let counter: i32 = 1;

    while s != s_last {
        counter += 1;
        change_next = true;
        for i in 0..len_str {
            if change_next {
                if s[i] == opt[opt.len()-1] {
                    s[i] = convert(s[i]);
                    change_next = true;
                }
                else {
                    s[i] = convert(s[i]);
                    break;
                }
            }
        }
    }

    println!("Number of generated k-mers: {}", counter);
    println!("Finish");
}

fn convert(&c: char) -> char {
    if c == 'A' {
        return 'C';
    }
    if c == 'C' {
        return 'G';
    }

    if c == 'G' {
        return 'T';
    }
    if c == 'T' {
        return 'A';
    }
    return ' ';
}