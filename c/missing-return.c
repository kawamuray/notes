#include <stdio.h>

struct foo {
    int n;
};

struct foo new_foo_ng() {
    struct foo f;
    f.n = 5;
    // missing return
}

struct foo new_foo_ok() {
    struct foo f;
    f.n = 10;
    return f;
}

int main(void) {
    struct foo f;
    f = new_foo_ng();
    printf("NG f.n = %d\n", f.n);
    f = new_foo_ok();
    printf("OK f.n = %d\n", f.n);
    return 0;
}
