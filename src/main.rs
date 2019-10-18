fn main() {
    printer::print()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        printer::print();
        assert!(false)
    }
}
