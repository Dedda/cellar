use crate::func::{Evaluate, EvalResult};
use crate::grid::DataSource;
use crate::func::convert::{value_as_int, value_as_float};

pub struct Add<Src> where Src: DataSource {
    arguments: Vec<Box<dyn Evaluate<Src>>>,
}

impl<Src> Evaluate<Src> for Add<Src> where Src: DataSource {
    fn eval(&self, data_source: &Src) -> EvalResult {
        let evaluated: Vec<String> = {
            let evaluated: Vec<EvalResult> = self.arguments.iter().map(|v| v.eval(&data_source)).collect();
            let mut unpacked = vec![];
            for element in evaluated {
                unpacked.push(element?);
            }
            unpacked
        };
        let mut ints = vec![];
        let mut floats = vec![];
        for element in evaluated {
            if let Ok(int) = value_as_int(&element) {
                ints.push(int);
            } else {
                floats.push(value_as_float(&element)?)
            }
        }
        if floats.is_empty() {
            Ok(format!("{}", ints.into_iter().sum::<i64>()))
        } else {
            let mut converted: Vec<f64> = ints.into_iter().map(|i| i as f64).collect();
            floats.append(&mut converted);
            Ok(format!("{}", floats.into_iter().sum::<f64>()))
        }
    }
}