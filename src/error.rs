use thiserror::Error;

#[derive(Error, Debug)]
pub enum InjectorError {
    #[error("Failed to attach to process {0} reason: {1}")]
    AttatchementError(u32, &'static str),

    #[error("Injection error in process: {0} whilst trying to inject {1} reason: {2}")]
    InjectionError(u32, String, &'static str),
}
