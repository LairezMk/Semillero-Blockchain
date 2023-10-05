// This shopping list program isn't compiling!
// Use your knowledge of generics to fix it.

// I AM NOT DONE
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generic() {
        let mut shopping_list: Vec<&str> = Vec::new();
        shopping_list.push("milk");
    }

}