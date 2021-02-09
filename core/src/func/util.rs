use std::slice::Iter;

pub fn try_for_all<T, O, E>(iter: Iter<T>, func: impl Fn(&T) -> Result<O, E>) -> Result<Vec<O>, E> {
    let mut collected = vec![];
    for element in iter {
        collected.push(func(element)?)
    }
    Ok(collected)
}