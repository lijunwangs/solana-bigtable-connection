pub mod access_token;
pub mod bigtable;
pub mod compression;
pub mod root_ca_certificate;

pub enum CredentialType {
    Filepath(Option<String>),
    Stringified(String),
}
