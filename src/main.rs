
mod luau;

fn main() {
    let mut checker = luau::TypeChecker::new();
    checker.check("demo.luau");
}
