extern crate core;

use comrak::{Arena, ComrakOptions, format_html, parse_document};
use comrak::nodes::NodeValue;
use crate::markdown_parser::{MarkdownParser, NoopMarkdownParser};

pub mod markdown_parser;
pub mod lua_parser;

fn main() {

}
