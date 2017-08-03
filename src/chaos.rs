use super::num::Rand;

pub trait Chaos {
    fn chaos(&mut self);
}

impl<T> Chaos for Vec<T> 
where T: Copy + Clone {
    fn chaos(&mut self) {
        let mut result: Vec<T> = Vec::new();
        let mut order: Vec<usize> = Vec::new();
        for i in 0..self.len() {
            order.push(i);
        }
        for _ in 0..self.len() {
            let index = usize::rand() % order.len();
            let value = self[order[index]];
            result.push(value);
            for i in index..order.len()-1 {
                order[i] = order[i+1];
            }
            order.pop();
        }       
        *self = result;
    }
}
