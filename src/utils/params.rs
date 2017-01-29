
#[derive(Debug)]
pub enum Param {
    Single(String),
    Many(String),
}

impl Param {
    pub fn name(&self) -> &str {
        match *self {
            Param::Single(ref name) |
            Param::Many(ref name) => &name,
        }
    }
}

pub struct ParamIter<'s> {
    buf: &'s str,
}

impl<'s> ParamIter<'s> {
    pub fn new(s: &'s str) -> ParamIter<'s> {
        ParamIter { buf: s }
    }
}

impl<'s> Iterator for ParamIter<'s> {
    type Item = Param;

    fn next(&mut self) -> Option<Param> {

        // Find the start and end indexes for the next parameter, if any.
        let (start, end) = match (self.buf.find('<'), self.buf.find('>')) {
            (Some(i), Some(j)) if i < j => (i, j),
            (Some(i), Some(j)) if i >= j => panic!("malformed parameter list"),
            (Some(_), None) => panic!("malformed parameter list"),
            _ => return None,
        };

        // Calculate the parameter's ident.
        let full_param = &self.buf[(start + 1)..end];
        let (is_many, param) = if full_param.ends_with("..") {
            (true, &full_param[..(full_param.len() - 2)])
        } else {
            (false, full_param)
        };

        // Advance the string and span.
        self.buf = &self.buf[(end + 1)..];

        // Check for nonemptiness, that the characters are correct, and return.
        if param.is_empty() {
            panic!("parameter names cannot be empty")
        } else if param.starts_with('_') {
            panic!("parameters cannot be ignored")
        } else if is_many && !self.buf.is_empty() {
            panic!("text after a trailing '..' param")
        } else {
            if is_many {
                Some(Param::Many(param.to_string()))
            } else {
                Some(Param::Single(param.to_string()))
            }
        }
    }
}
