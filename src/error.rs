use failure::Fail;

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "{}", _0)]
    GenericError(String),
}
