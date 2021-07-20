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
}

fn converter(&c: char) -> char {
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