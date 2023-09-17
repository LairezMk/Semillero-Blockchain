// Don't mind this for now :)
// I AM NOT DONE

#[cfg(test)]
mod tests {
use super::call_me;

    #[test]
    fn call_function() {
        let result = call_me();
        assert_eq!(result, 42);
    }

}

fn call_me() -> i32 {
    42
}