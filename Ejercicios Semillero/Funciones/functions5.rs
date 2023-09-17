// I AM NOT DONE

fn square(num: i32) -> i32 {
    return num * num;
}

#[cfg(test)]
mod tests {
use super::square;    
    #[test]
    fn call_function() {
        let answer: i32   = square(100);
        println!("The square of 3 is {}", answer);
    }
}