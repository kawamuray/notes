extern crate test;
use std::collections::HashMap;
use test::Bencher;
use lazy_static::lazy_static;

#[derive(Debug, Clone, Copy)]
enum Lang {
    Rust, Java, JavaScript, Python, C, Cpp, Haskell, WebAssembly, Perl, Ruby
}

const KEYS: [&'static str; 13] = [
    "rust",
    "java",
    "javascript",
    "python",
    "c",
    "cpp",
    "haskell",
    "webassembly",
    "perl",
    "ruby",
    "does_not_exists1",
    "does_not_exists2",
    "does_not_exists3",
];

lazy_static!(
    static ref MAPPING: HashMap<&'static str, Lang> = {
        use Lang::*;
        let mut m = HashMap::new();
        m.insert("rust", Rust);
        m.insert("java", Java);
        m.insert("javascript", JavaScript);
        m.insert("python", Python);
        m.insert("c", C);
        m.insert("cpp", Cpp);
        m.insert("haskell", Haskell);
        m.insert("webassembly", WebAssembly);
        m.insert("perl", Perl);
        m.insert("ruby", Ruby);
        m
    };
);


#[inline(never)]
fn lookup_match(key: &str) -> Option<Lang> {
    use Lang::*;
    Some(match key {
        "rust" =>  Rust,
        "java" =>  Java,
        "javascript" =>  JavaScript,
        "python" =>  Python,
        "c" =>  C,
        "cpp" =>  Cpp,
        "haskell" =>  Haskell,
        "webassembly" =>  WebAssembly,
        "perl" =>  Perl,
        "ruby" =>  Ruby,
        _ => return None,
    })
}

#[inline(never)]
fn lookup_map(key: &str) -> Option<Lang> {
    if let Some(v) = MAPPING.get(key) {
        Some(*v)
    } else {
        None
    }
}

#[bench]
fn bm_match(b: &mut Bencher) {
    b.iter(|| {
        for k in &KEYS {
            test::black_box(lookup_match(k));
        }
    });
}

#[bench]
fn bm_hashmap(b: &mut Bencher) {
    b.iter(|| {
        for k in &KEYS {
            test::black_box(lookup_map(k));
        }
    });
}
