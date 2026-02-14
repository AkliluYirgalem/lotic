use lotic::program;

#[program]
mod hello_lotic {
    use std::fmt::Error;

    pub fn _initialize() -> Result<(), Error> {
        Ok(())
    }

    fn _init() -> Result<(), Error> {
        Ok(())
    }
}
