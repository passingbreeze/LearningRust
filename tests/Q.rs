use std::fmt::{Display, Debug};

struct Q<T> {
    container : Vec<T>,
}

impl<T> Q<T>
    where T : Display{
    pub fn new() -> Q<T> {
        let cont = Vec::new();
        Q {
            container : cont,
        }
    }

    pub fn push(&mut self, item : T) {
        println!("input item : {}", item);
        self.container.push(item);
    }

    pub fn size(&self) -> usize {
        self.container.len()
    }

}

impl<T> Q<T>
    where T : Debug {

    pub fn contains(&self) -> &Vec<T> {
        &self.container
    }

    pub fn pop(&mut self) {
        println!("before pop : {:?}", self.container);
        self.container.pop();
        println!("after pop : {:?}", self.container);
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn num_case(){
        let mut exa = Q::new();
        exa.push(1);
        exa.push(2);
        exa.push(3);

        exa.pop();
        assert_eq!(vec![1,2], *(exa.contains()));
    }
}