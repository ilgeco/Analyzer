use regex::Regex;

use super::{IdList, CreateSeries};
pub struct IdRegexListBuilder;

impl CreateSeries<str, f64, IdList> for IdRegexListBuilder {
    fn create_series(input: &str) -> IdList {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"(?m)(?:(\S+):|^\s*)[ \t]*(-?[0-9]+(?:\.[0-9]*)?)").unwrap();
        }
        let mut ret = IdList::new();

        for cap in RE.captures_iter(input) {
            match (cap.get(1), cap.get(2)) {
                (None, Some(x)) => {
                    let s_num = x.as_str();
                    ret.push("", s_num.parse().unwrap());
                }
                (Some(y), Some(x)) => {
                    let id = y.as_str();
                    let s_num = x.as_str();
                    ret.push(id, s_num.parse().unwrap());
                }
                _ => panic!("Come ci sei riuscito?"),
            }
        }
        ret
    }
}