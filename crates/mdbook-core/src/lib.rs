#![recursion_limit="128"]

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;
#[macro_use]
extern crate error_chain;
extern crate toml_query;
extern crate handlebars;

pub mod config;
pub mod book;
pub mod errors {
    use std::path::PathBuf;

    error_chain!{
        foreign_links {
            Io(::std::io::Error) #[doc = "A wrapper around `std::io::Error`"];
            HandlebarsRender(::handlebars::RenderError) #[doc = "Handlebars rendering failed"];
            HandlebarsTemplate(Box<::handlebars::TemplateError>) #[doc = "Unable to parse the template"];
            Utf8(::std::string::FromUtf8Error) #[doc = "Invalid UTF-8"];
            SerdeJson(::serde_json::Error) #[doc = "JSON conversion failed"];
        }

        links {
            TomlQuery(::toml_query::error::Error, ::toml_query::error::ErrorKind) #[doc = "A TomlQuery error"];
        }

        errors {
            /// A subprocess exited with an unsuccessful return code.
            Subprocess(message: String, output: ::std::process::Output) {
                description("A subprocess failed")
                display("{}: {}", message, String::from_utf8_lossy(&output.stdout))
            }

            /// An error was encountered while parsing the `SUMMARY.md` file.
            ParseError(line: usize, col: usize, message: String) {
                description("A SUMMARY.md parsing error")
                display("Error at line {}, column {}: {}", line, col, message)
            }

            /// The user tried to use a reserved filename.
            ReservedFilenameError(filename: PathBuf) {
                description("Reserved Filename")
                display("{} is reserved for internal use", filename.display())
            }
        }
    }

    // Box to halve the size of Error
    impl From<::handlebars::TemplateError> for Error {
        fn from(e: ::handlebars::TemplateError) -> Error {
            From::from(Box::new(e))
        }
    }
}


pub const MDBOOK_VERSION: &str = env!("CARGO_PKG_VERSION");