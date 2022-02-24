







#[cfg(test)]
mod tests {
    use python::List;
    use python::print;

    #[test]
    fn test_initialization() {
        let python_list = List::new();
        // i know that there is not actual test
        // but more things need to be implemented
        // like some default traits for class list, like Copy, Debug, Eq, PartialEq ...
        // in order to have assert_eq!()
    }

    #[test]
    fn test_append_int() {
        let mut python_list = List::new();
        python_list.append_int(123);
        python_list.append_int(123);
        python_list.append_int(123);
        python_list.append_int(123);
        python_list.append_int(123);
        print(python_list);
    }
}