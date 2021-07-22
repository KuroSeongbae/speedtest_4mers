use chrono::prelude::*;

fn main() {
    println!("Start");

    let start_time = Local::now();

    let opt = ['A', 'C', 'G', 'T'];
    let mut s = String::new();
    let mut s_last = String::new();
    let len_str: usize = 13;
    let mut change_next: bool;

    for _ in 0..len_str {
        s += &opt[0].to_string();
    }

    for _ in 0..len_str {
        s_last += &opt[opt.len()-1].to_string();
    }

    let pos: i32 = 0;
    let mut counter: i32 = 1;

    while s != s_last {
        counter += 1;
        // println!("{}", s);
        change_next = true;
        for i in 0..len_str {
            if change_next {
                let mut s_list: Vec<_> = s.chars().collect();
                if s_list[i] == opt[opt.len()-1] {
                    s_list[i] = convert(s_list[i]);
                    s = s_list.into_iter().collect();
                    change_next = true;
                }
                else {
                    s_list[i] = convert(s_list[i]);
                    s = s_list.into_iter().collect();
                    break;
                }
            }
        }
    }
    let end_time = Local::now();
    let duration = end_time.signed_duration_since(start_time);
    println!("Number of generated k-mers: {}", counter);
    println!("Finish! Needed {} seconds.", duration.num_seconds());
}

fn convert(c: char) -> char {
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