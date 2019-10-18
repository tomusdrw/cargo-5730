#[cfg(feature = "print")]
pub fn print() {
    println!("If you see this cargo is still broken.");
}

#[cfg(not(feature = "print"))]
pub fn print() {
    println!("Cargo is fixed!");
}
