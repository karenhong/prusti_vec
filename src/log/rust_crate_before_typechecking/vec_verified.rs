#![feature(custom_attribute)]
extern crate prusti_contracts;
trait PrustiVec<T> {
    #[ensures = "result.len() == 0"]
    #[__PRUSTI_SPEC = r#"101"#]
    fn new()
    -> Vec<T>;
    #[__PRUSTI_SPEC_ONLY = r#"101"#]
    #[allow(unused_mut)]
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[allow(unused_variables)]
    fn new__spec(result: Vec<T>) -> () {
        #[allow(unused_imports)]
        use prusti_contracts::internal::*;

        #[__PRUSTI_EXPR_ID = r#"101"#]
        #[pure]
        || -> bool { result.len() == 0 };
    }
    #[requires = "capacity >= 0"]
    #[ensures = "result.len() == 0"]
    #[ensures = "result.capacity() == capacity"]
    #[__PRUSTI_SPEC = r#"102"#]
    fn with_capacity(capacity: usize)
    -> Vec<T>;
    #[__PRUSTI_SPEC_ONLY = r#"102"#]
    #[allow(unused_mut)]
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[allow(unused_variables)]
    fn with_capacity__spec(capacity: usize, result: Vec<T>) -> () {
        #[allow(unused_imports)]
        use prusti_contracts::internal::*;

        #[__PRUSTI_EXPR_ID = r#"102"#]
        #[pure]
        || -> bool { capacity >= 0 };

        #[__PRUSTI_EXPR_ID = r#"103"#]
        #[pure]
        || -> bool { result.len() == 0 };

        #[__PRUSTI_EXPR_ID = r#"104"#]
        #[pure]
        || -> bool { result.capacity() == capacity };
    }
    #[ensures = "result >= self.len()"]
    #[__PRUSTI_SPEC = r#"103"#]
    fn capacity(&self)
    -> usize;
    #[__PRUSTI_SPEC_ONLY = r#"103"#]
    #[allow(unused_mut)]
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[allow(unused_variables)]
    fn capacity__spec(&self, result: usize) -> () {
        #[allow(unused_imports)]
        use prusti_contracts::internal::*;

        #[__PRUSTI_EXPR_ID = r#"105"#]
        #[pure]
        || -> bool { result >= self.len() };
    }
    #[ensures = "self.len() == old(self.len()) + 1"]
    #[__PRUSTI_SPEC = r#"104"#]
    fn push(&mut self, value: T);
    #[__PRUSTI_SPEC_ONLY = r#"104"#]
    #[allow(unused_mut)]
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[allow(unused_variables)]
    fn push__spec(&mut self, value: T, result: ()) -> () {
        #[allow(unused_imports)]
        use prusti_contracts::internal::*;

        #[__PRUSTI_EXPR_ID = r#"106"#]
        #[pure]
        || -> bool { self.len() == old(self.len()) + 1 };
    }
    #[ensures = "self.len() == old(self.len()) - 1"]
    #[__PRUSTI_SPEC = r#"105"#]
    fn pop(&mut self)
    -> Option<T>;
    #[__PRUSTI_SPEC_ONLY = r#"105"#]
    #[allow(unused_mut)]
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[allow(unused_variables)]
    fn pop__spec(&mut self, result: Option<T>) -> () {
        #[allow(unused_imports)]
        use prusti_contracts::internal::*;

        #[__PRUSTI_EXPR_ID = r#"107"#]
        #[pure]
        || -> bool { self.len() == old(self.len()) - 1 };
    }
    #[ensures = "result >= 0"]
    #[__PRUSTI_SPEC = r#"106"#]
    fn len(&self)
    -> usize;
    #[__PRUSTI_SPEC_ONLY = r#"106"#]
    #[allow(unused_mut)]
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[allow(unused_variables)]
    fn len__spec(&self, result: usize) -> () {
        #[allow(unused_imports)]
        use prusti_contracts::internal::*;

        #[__PRUSTI_EXPR_ID = r#"108"#]
        #[pure]
        || -> bool { result >= 0 };
    }
    #[ensures = "self.len() == old(self.len()) + 1"]
    #[__PRUSTI_SPEC = r#"107"#]
    fn append(&mut self, other: &mut Vec<T>);
    #[__PRUSTI_SPEC_ONLY = r#"107"#]
    #[allow(unused_mut)]
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[allow(unused_variables)]
    fn append__spec(&mut self, other: &mut Vec<T>, result: ()) -> () {
        #[allow(unused_imports)]
        use prusti_contracts::internal::*;

        #[__PRUSTI_EXPR_ID = r#"109"#]
        #[pure]
        || -> bool { self.len() == old(self.len()) + 1 };
    }
}
#[__PRUSTI_SPEC = r#"108"#]
fn main() { }