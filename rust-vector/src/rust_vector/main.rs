pub struct RustVector<'a> {
    pub vector: &'a mut Vec<i64>,
}

impl<'a> RustVector<'a> {
    pub fn add(&mut self, value: i64) {
        self.vector.push(value);
    }
}
