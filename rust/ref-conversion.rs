#[derive(Clone)]
struct Foo(String);

impl From<Foo> for String {
    fn from(foo: Foo) -> Self {
        foo.0
    }
}

impl AsRef<str> for Foo {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

fn take_str(s: &str) {
    println!("str: {}", s);
}

fn main() {
    let foo = Foo("abc".to_string());
    take_str(foo.as_ref());
    let foo2 = foo.clone();
    let s: String = foo2.into();
    take_str(&s);
    take_str(&foo);
}

impl std::ops::Deref for Foo {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}
