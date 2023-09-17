#[cfg(test)]
mod tests {
    #[test]
    fn test_primitive_types5() {
        let cat = ("Furry McFurson", 3.5);
        let (name, age) = cat;
    
        println!("{} is {} years old.", name, age);
    }
}