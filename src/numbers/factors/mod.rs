pub struct FactorIter {
    number: u64,
    last: u64,
    temp: Option<u64>,
}

impl FactorIter {
    pub fn new(number: u64) -> Self {
        FactorIter { number, last: 0, temp: None }
    }
}

impl Iterator for FactorIter {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.temp.is_some() {
            return self.temp.take();
        } else {
            let factor = (self.last+1..)
                .take_while(|i| i * i < self.number)
                .filter(|i| self.number % i == 0)
                .nth(0);

            match factor {
                Some(f) => {
                    self.temp = Some(self.number / f);
                    self.last = f;
                },
                None => ()
            }
            
            return factor;
        }
    }
}
