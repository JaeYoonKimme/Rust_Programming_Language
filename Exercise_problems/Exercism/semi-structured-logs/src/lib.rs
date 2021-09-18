// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

/// various log levels
#[derive(Clone, PartialEq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
}
/// primary function for emitting logs
pub fn log(level: LogLevel, message: &str) -> String {
	match level {
		Info => return info(message),
		Warning => return warn(message),
		Error => return error(message),
	}
}
pub fn info(message: &str) -> String {
	let mes = message;
	let result = "[INFO]: " + mes
}
pub fn warn(message: &str) -> String {
	let mes = message;
	let result = "[WARN]: " + mes
}
pub fn error(message: &str) -> String {
	let mes = message;
	let result = "[ERROR]: " + mes
}
