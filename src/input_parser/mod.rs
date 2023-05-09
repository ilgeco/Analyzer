use std::{borrow::Borrow, collections::HashMap};
pub mod id_nom_builder;
pub mod id_regex_builder;

pub trait Series<K: ?Sized, T> {
    type IntoIteratorInner<'a>: IntoIterator<Item = &'a T>
    where Self: 'a, T: 'a, K: 'a; 

    type IntoIterOuter<'a>: IntoIterator<Item = (&'a K, Self::IntoIteratorInner<'a>)>
    where
        Self: 'a, K: 'a, T: 'a;

    fn get_serie(&self, input: impl Borrow<K>) -> Option<&Vec<T>>;
    fn into_iter<'a>(&'a self) -> <Self::IntoIterOuter<'a> as IntoIterator>::IntoIter;
    
}

pub trait CreateSeries<K: ?Sized, T, Q>
where
    Q: Series<K, T>,
{
    fn create_series(input: &str) -> Q;
}

#[derive(Debug)]
pub struct IdList {
    series: HashMap<String, Vec<f64>>,
}

impl IdList {
    fn new() -> Self {
        IdList {
            series: HashMap::new(),
        }
    }

    fn push(&mut self, input: &str, val: f64) {
        match self.series.get_mut(input) {
            Some(x) => {
                x.push(val);
            }
            None => {
                self.series.entry(input.to_owned()).or_default().push(val);
            }
        };
    }
}

impl Series<String, f64> for IdList {

    type IntoIteratorInner<'a> = &'a Vec<f64>;
    type IntoIterOuter<'a> = &'a HashMap<String, Vec<f64>>;



    fn into_iter(&self) -> std::collections::hash_map::Iter<String, Vec<f64>> {
        (&self.series).into_iter()
    }

    fn get_serie(&self, input: impl Borrow<String>) -> Option<&Vec<f64>> {
        self.series.get(input.borrow())
    }

    
}

pub fn parse<R: CreateSeries<K, T, Q>, K, T, Q>(input: &str, _: R) -> Q
where
    Q: Series<K, T>,
    K: ?Sized,
{
    R::create_series(input)
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::util::{retrive_file};

    use super::{id_nom_builder::IdNomListBuilder, id_regex_builder::IdRegexListBuilder, *};

    #[cfg(test)]
    #[allow(dead_code)]
    fn parse_file<R: CreateSeries<K, T, Q>, K, T, Q>(path: &Path, crate_series: R) -> Q
    where
        Q: Series<K, T>,
        K: ?Sized,
    {
        let input = retrive_file(path);
        parse(&input, crate_series)
    }

    #[test]
    fn simple_test_regex() {
        let input = r#"
        aasdw come sfasr nosdfo mmm curl:    23
        casco: 5554
        masse c: sscome
        ciccio: 22
        ciccio: 33.12345
        23.2
        33.2
        :31.2
        .
        e qui: 23.4
        "#;
        let series = IdRegexListBuilder::create_series(&input);
        for i in series.series.iter() {
            println!("{:?}", i);
        }
    }

    #[test]
    fn simple_test_num() {
        let input = r#"
        aasdw come sfasr nosdfo mmm curl:    23
        casco: 5554
        masse c: sscome
        ciccio: 22
        ciccio: 33.12345
        23.2
        33.2
        :31.2
        .
        e qui: 23.4
        "#;
        let series = IdNomListBuilder::create_series(&input);
        for i in series.series.iter() {
            println!("{:?}", i);
        }
    }
}
