extern crate native;
extern crate libc;
use libc::c_int;
use std::mem::uninitialized;

#[allow(dead_code, uppercase_variables, non_camel_case_types)]
mod c;

pub type HttpParserSettings = c::Struct_http_parser_settings;

pub enum HttpParserType {
  Request = c::HTTP_REQUEST as int,
  Response = c::HTTP_RESPONSE as int,
  Both = c::HTTP_BOTH as int,
}

pub struct HttpParser {
  parser: c::Struct_http_parser,
}

impl HttpParser {
  pub fn new(type_: HttpParserType) -> HttpParser {
    let mut parser: c::Struct_http_parser = unsafe { uninitialized() };
    unsafe { c::http_parser_init(&mut parser, type_ as u32) };
    HttpParser {
      parser: parser
    }
  }

  pub fn execute(settings: c::Struct_http_parser_settings, data: &[u8]) {

  }

  pub fn should_keep_alive(&self) -> bool {
    if unsafe { c::http_should_keep_alive(&self.parser) } == 0 { return false }
    true
  }
}
