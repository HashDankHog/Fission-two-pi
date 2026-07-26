#[derive(Clone, Debug, PartialEq)]
pub enum Connection {
    Line,
    Arc,
    Intersection,
    Union,
    Difference,
    Sweep
}
impl Connection {
    pub fn draw(&self, _frame: Vec<u8>) -> Vec<u8> {
        unimplemented!()
    }
}