/// Use below command to install cargo utility
///       cargo install cargo-expand
/// Then execute below line to see macro expantion.
///     cargo rustc -- -Zunpretty=expanded
///
/// It show on console expantion of macro.

macro_rules! say_hello {
    () => {
        println!("Hello World");
    };
}

#[allow(dead_code)]
fn sm_main() {
    say_hello!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sm_test() {
        sm_main();
    }
}
