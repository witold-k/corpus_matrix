use simplefield::field::Field;
use simplefield::orientation::RowMajor;

pub type CountMatrix = Field<RowMajor, u64>;

#[derive(Debug, Clone)]
pub enum Matrix {
    Count(CountMatrix),
}
