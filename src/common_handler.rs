#![allow(clippy::upper_case_acronyms)]
use eyre::{eyre, Result};
use std::ffi::OsString;
use std::fmt;
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Debug)]
pub enum Either<A, B> {
    Left(A),
    Right(B),
}

pub type Pair<A> = (A, A);
pub type Range<A> = (A, A);
pub type RangeOrValue<A> = Either<Range<A>, A>;

#[derive(Debug)]
pub enum RangeOrValueEnum {
    Integer(Option<RangeOrValue<i32>>),
    Real(Option<RangeOrValue<f32>>),
    String(Option<String>),
}

#[derive(Debug)]
pub enum OnOff {
    ON,
    OFF,
}
impl fmt::Display for OnOff {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OnOff::ON => write!(f, "ON"),
            OnOff::OFF => write!(f, "OFF"),
        }
    }
}

#[derive(Debug)]
pub enum Orient {
    N,
    S,
    E,
    W,
    FN,
    FS,
    FE,
    FW,
}

#[macro_export]
macro_rules! vec_push {
    ($c:ident, $field:ident, $v:expr) => {
        match $c.$field {
            None => {
                $c.$field = Some(vec![$v]);
                $c
            }
            Some(ref mut p) => {
                p.push($v);
                $c
            }
        }
    };
}

#[macro_export]
macro_rules! set_opt {
    ($c:ident, $field:ident, $v:expr) => {{
        $c.$field = Some($v);
        $c
    }};
}

#[macro_export]
macro_rules! set_field {
    ($c:ident, $field:ident, $v:expr) => {{
        $c.$field = $v;
        $c
    }};
}

#[macro_export]
macro_rules! copy_opt {
    ($c:expr, $field:ident, $o:ident) => {
        match $o.$field {
            Some(v) => $c.$field = Some(v),
            None => (),
        }
    };
}

#[macro_export]
macro_rules! copy_vec_opt {
    ($c:expr, $field:ident, $o:ident) => {
        match $o.$field {
            Some(mut v) => match $c.$field {
                Some(ref mut vc) => {
                    vc.append(&mut v);
                }
                None => (),
            },
            None => (),
        }
    };
}

#[macro_export]
macro_rules! panic_with_context {
    ($msg:expr, $ctx:expr) => {{
        panic!(
            "[x] {}: {:?}\n\t{:?}",
            $msg,
            $ctx.error,
            $ctx.dropped_tokens
                .iter()
                .take(20)
                .map(|(_, token, _)| token.1.clone())
                .collect::<Vec<_>>()
        )
    }};
}

// clean the file from comments
fn remove_comment_lines(input: String) -> String {
    input
        .lines()
        .filter(|line| !line.trim_start().starts_with('#'))
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn read_file(input: &OsString) -> Result<String> {
    let input = Path::new(input);
    let mut s = String::new();
    if let Err(err) = File::open(input).and_then(|mut f| f.read_to_string(&mut s)) {
        println!("Input `{}`: I/O Error {}", input.display(), err);
        return Err(eyre!("Input `{}`: I/O Error {}", input.display(), err));
    }
    Ok(remove_comment_lines(s.to_ascii_uppercase()))
}
