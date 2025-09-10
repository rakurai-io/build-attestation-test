fn main() {
    log::info!("Hello from rust-provenance-demo with submodule!");
    println!("Program executed successfully.");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
