use util::ErrorKind;

#[derive(Clone)]
pub enum Err<P> {
    Position(ErrorKind<u32>, P),
}

#[derive(Clone)]
pub enum Needed {
    Unknown,
    Size(usize),
}

#[derive(Clone)]
pub enum IResult<I, O> {
    Done(I, O),
    Error(Err<I>),
    Incomplete(Needed),
}
