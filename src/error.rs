use std::io;
use std::fmt;
use std::error;
use std::convert;
use beam_file;
use eetf;

#[derive(Debug)]
pub enum BeamParseError {
    Io(io::Error),
    BeamFile(beam_file::Error),
    TermDecode(eetf::DecodeError),
    NoDebugInfo,
    NoModuleAttribute,
    UnexpectedTerm(Vec<Unmatched>),
}
impl error::Error for BeamParseError {
    fn description(&self) -> &str {
        match *self {
            BeamParseError::Io(ref x) => x.description(),
            BeamParseError::BeamFile(ref x) => x.description(),
            BeamParseError::TermDecode(ref x) => x.description(),
            BeamParseError::NoDebugInfo => "No debug information",
            BeamParseError::NoModuleAttribute => "No module attribute",
            BeamParseError::UnexpectedTerm(_) => "Unexpected term",
        }
    }
    fn cause(&self) -> Option<&error::Error> {
        match *self {
            BeamParseError::Io(ref x) => x.cause(),
            BeamParseError::BeamFile(ref x) => x.cause(),
            BeamParseError::TermDecode(ref x) => x.cause(),
            _ => None,
        }
    }
}
impl fmt::Display for BeamParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BeamParseError::Io(ref x) => x.fmt(f),
            BeamParseError::BeamFile(ref x) => x.fmt(f),
            BeamParseError::TermDecode(ref x) => x.fmt(f),
            BeamParseError::NoDebugInfo => write!(f, "The beam has no debug information"),
            BeamParseError::NoModuleAttribute => write!(f, "No module attribute"),
            BeamParseError::UnexpectedTerm(ref trace) => {
                try!(write!(f, "Unexpected term: ["));
                let limit = 3;
                for (i, e) in trace.iter().take(limit).enumerate() {
                    if i != 0 {
                        try!(write!(f, ","));
                    }
                    try!(write!(f, "{}", e));
                }
                if trace.len() > limit {
                    try!(write!(f, " ..{}..", trace.len() - limit));
                }
                try!(write!(f, "]"));
                Ok(())
            }
        }
    }
}
impl convert::From<io::Error> for BeamParseError {
    fn from(x: io::Error) -> Self {
        BeamParseError::Io(x)
    }
}
impl convert::From<beam_file::Error> for BeamParseError {
    fn from(x: beam_file::Error) -> Self {
        BeamParseError::BeamFile(x)
    }
}
impl convert::From<eetf::DecodeError> for BeamParseError {
    fn from(x: eetf::DecodeError) -> Self {
        BeamParseError::TermDecode(x)
    }
}
impl<'a> convert::From<eetf::pattern::Unmatch<'a>> for BeamParseError {
    fn from(x: eetf::pattern::Unmatch<'a>) -> Self {
        use std::ops::Deref;
        let mut trace = Vec::new();
        let mut curr = Some(&x);
        while let Some(x) = curr {
            trace.push(Unmatched {
                value: x.input.clone(),
                pattern: format!("{:?}", x.pattern),
            });
            curr = x.cause.as_ref().map(|x| x.deref());
        }
        trace.reverse();
        BeamParseError::UnexpectedTerm(trace)
    }
}

#[derive(Debug)]
pub struct Unmatched {
    pub value: eetf::Term,
    pub pattern: String,
}
impl fmt::Display for Unmatched {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{pattern:{}, value:{}}}", self.pattern, self.value)
    }
}
