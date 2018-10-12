pub fn foo() {

}

#[cfg(test)]
mod tests {
    use vec::*;
    #[test]
    fn another() {
        foo();
        panic!("Make this test fail");
    }
}