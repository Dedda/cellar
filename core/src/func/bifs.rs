use crate::func::{Evaluate, EvalResult};
use crate::grid::DataSource;
use crate::func::value::Value;
use std::convert::TryInto;
use crate::func::util::try_for_all;

pub struct Add<Src> where Src: DataSource {
    arguments: Vec<Box<dyn Evaluate<Src>>>,
}

impl<Src> Evaluate<Src> for Add<Src> where Src: DataSource {
    fn eval(&self, data_source: &Src) -> EvalResult {
        let values = try_for_all(self.arguments.iter(), |arg| arg.eval(&data_source))?;
        if values.iter().find(|v| v.is_float()).is_some() {
            let floats = try_for_all(values.iter(), |v| v.try_into())?;
            Ok(Value::Float(floats.iter().sum()))
        } else {
            let ints = try_for_all(values.iter(), |v| v.try_into())?;
            Ok(Value::Int(ints.iter().sum()))
        }
    }
}

pub struct ReadCell {
    x: usize,
    y: usize,
}

impl<Src> Evaluate<Src> for ReadCell where Src: DataSource {
    fn eval(&self, data_source: &Src) -> EvalResult {
        let data = data_source.read_at(&self.x, &self.y).unwrap_or("".into());
        Ok(Value::parse(data))
    }
}