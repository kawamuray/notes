fn take_string(s: &str) -> String {
    let mut ret = String::new();
    let mut iter = s.chars();
    while let Some(c) = iter.next() {
        ret.push(c);
        match c {
            '\\' => {
                // look ahead
                if let Some(nc) = iter.next() {
                    ret.push(nc);
                }
            }
            '"' => return ret,
            _ => {}
        }
    }
    panic!("should not reach here");
}

fn main() {
    let cases = vec![
        "A\"",
        "A\\\"\"",
        "A\\\\\"\"",
        "A\\\\\\\"\"",
        "A\\\\\\\\\"\"",
    ];
    for case in cases {
        println!("orig = {}, parsed = {}", case, take_string(case));
    }
}
