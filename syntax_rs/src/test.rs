//test.rs
//Copyright 2015 David Huddle

#[cfg(test)]
mod test{
    use my_other_mod::*;

    #[test]
    fn i_am_a_test(){
        let x = cmp(2, 10);
        match x{
            Ordering::Less => assert!(true),
            Ordering::Greater => assert!(false),
            Ordering::Equal => assert!(false),
        }
    }

    #[test]
    fn test2(){
        assert!(used_only_in_test()==235);
    }
}
