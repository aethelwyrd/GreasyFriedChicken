//test.rs
//Copyright 2015 David Huddle

#[cfg(test)]
mod test{
    use main_mod::*;

    #[test]
    fn test_in_mod(){
        let x = cmp(2, 10);
        match x{
            Ordering::Less => assert!(true),
            Ordering::Greater => assert!(false),
            Ordering::Equal => assert!(false),
        }
    }

    #[test]
    fn test_fn_used_only_in_test(){
        assert!(used_only_in_test()==235);
    }
}
