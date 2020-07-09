mod correction_data;
mod errors;
pub use errors::*;
mod fda_table;
mod ipol;
mod of_table;

use console::Term;
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};

pub fn question(term: &Term, msg: &str) -> Result<String, EmuError> {
    if let Err(e) = term.write_str(&format!("{}: ", msg)) {
        return Err(EmuError::Terminal(e.to_string()));
    }
    let res_ans = term.read_line();
    if let Err(e) = res_ans {
        return Err(EmuError::Terminal(e.to_string()));
    }
    let ans_str = res_ans.unwrap();
    return Ok(ans_str);
}

pub(crate) fn question_parse_res<T>(term: &Term, msg: &str) -> Result<T, EmuError>
where
    T: std::str::FromStr + std::fmt::Display,
    <T as std::str::FromStr>::Err: std::string::ToString + std::fmt::Debug,
{
    let res_ans = question(term, msg);
    if let Err(e) = res_ans {
        return Err(e);
    }
    let ans_str = res_ans.unwrap();
    let res_f = ans_str.parse::<T>();
    if let Err(e) = res_f {
        return Err(EmuError::Terminal(e.to_string()));
    }
    Ok(res_f.unwrap())
}

pub fn question_with_options<T: std::fmt::Display>(
    term: &Term,
    msg: &str,
    options: &Vec<T>,
) -> Result<usize, EmuError> {
    let mut work = true;
    while work {
        let mut i: usize = 0;
        for opt in options {
            if let Err(e) = term.write_line(&format!("{}. {}", i + 1, opt)) {
                return Err(EmuError::Terminal(e.to_string()));
            }
            i = i + 1;
        }
        if let Err(e) = term.write_str(&format!("{}: ", msg)) {
            return Err(EmuError::Terminal(e.to_string()));
        }
        let res_ans = term.read_line();
        if let Err(e) = res_ans {
            return Err(EmuError::Terminal(e.to_string()));
        }
        let ans_str = res_ans.unwrap();
        let res_ans_int = ans_str.parse::<usize>();
        if let Err(e) = res_ans_int {
            return Err(EmuError::Terminal(e.to_string()));
        }
        let ans = res_ans_int.unwrap();
        if ans >= 1 || ans <= options.len() {
            work = false;
            return Ok(ans - 1);
        }
    }
    return Err(EmuError::Terminal("Unreachable statement".to_owned()));
}
