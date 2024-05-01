// traits2.rs
//
// Your task is to implement the trait `AppendBar` for a vector of strings. To
// implement this trait, consider for a moment what it means to 'append "Bar"'
// to a vector of strings.
//
// No boiler plate code this time, you can do this!
//
// Execute `rustlings hint traits2` or use the `hint` watch subcommand for a hint.



trait AppendBar {
    fn append_bar(&self) -> Vec<String>;
}

impl AppendBar for Vec<String> {
    fn append_bar(&self) -> Vec<String> {
        self.iter().map(|s| {
            let mut new_string = s.clone();
            new_string.push_str("Bar");
            new_string
        }).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.last().unwrap().as_str(), "FooBar");
        assert_eq!(foo.len(), 1);
    }
}

