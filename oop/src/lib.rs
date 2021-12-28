pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        println!("{:?}", self.list);
        self.average = total as f64 / self.list.len() as f64;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello() {
        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();

        let total: i32 = v1_iter.sum();

        println!("{:?}", v1);
    }
}