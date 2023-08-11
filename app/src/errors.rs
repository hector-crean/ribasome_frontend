///ThrandError  enumerates all possible errors returned by this library.
#[derive(thiserror::Error, Debug)]
pub enum RibasomeAppError {}

pub type Result<T> = color_eyre::eyre::Result<T, RibasomeAppError>;
