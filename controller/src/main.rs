use thiserror::Error;

mod crds;
use crds::*;

#[derive(Error, Debug)]
pub enum ControllerError {
    #[error("pwdb")]
    HttpError,
    #[error("Vault")]
    VaultError,
}

fn main() {
    let _: PasswordStateSpec;
    let _: SecretSpec;
    let _: ListSpec;
    hello();
}
