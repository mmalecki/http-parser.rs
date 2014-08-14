#![feature(macro_rules)]

extern crate native;
extern crate libc;
use std::mem::uninitialized;

#[allow(dead_code, uppercase_variables, non_camel_case_types)]
mod c;

pub static HTTP_REQUEST: u32 = c::HTTP_REQUEST;
pub static HTTP_RESPONSE: u32 = c::HTTP_RESPONSE;
pub static HTTP_BOTH: u32 = c::HTTP_BOTH;

pub type HttpCb = fn (parser: &HttpParser) -> Result<(), ()>;
pub type HttpDataCb = fn (parser: &HttpParser, data: &str) -> Result<(), ()>;

pub struct HttpParserSettings {
  pub on_message_begin: Option<HttpCb>,
  pub on_url: Option<HttpDataCb>,
  pub on_status: Option<HttpDataCb>,
  pub on_header_field: Option<HttpDataCb>,
  pub on_header_value: Option<HttpDataCb>,
  pub on_headers_complete: Option<HttpCb>,
  pub on_body: Option<HttpDataCb>,
  pub on_message_complete: Option<HttpCb>,
}

pub struct HttpParser {
  parser: c::Struct_http_parser,
}

impl HttpParserSettings {
  pub fn to_native(&self) -> c::Struct_http_parser_settings {
    // I tried making those macros but had problems passing self down to the 
    // macro and wanted to just get this done. TODO: investigate turning this
    // into macros once more.
    extern "C" fn on_message_begin_wrap(parser: *mut c::Struct_http_parser) -> i32 {
      println!("on_message_begin");
      0 as i32
    }

    extern "C" fn on_url_wrap(parser: *mut c::Struct_http_parser,
                              data: *const libc::c_char, size: c::size_t) -> i32 {
      println!("on_url");
      0 as i32
    }

    c::Struct_http_parser_settings {
      on_message_begin: match self.on_message_begin {
                          Some(f) => Some(on_message_begin_wrap),
                          None => None
                        },
      on_url: match self.on_message_begin {
                Some(f) => Some(on_url_wrap),
                None => None
              },
      on_status: None,
      on_header_field: None,
      on_header_value: None,
      on_headers_complete: None,
      on_body: None,
      on_message_complete: None
    }
  }
}

impl HttpParser {
  pub fn new(type_: u32) -> HttpParser {
    let mut parser: c::Struct_http_parser = unsafe { uninitialized() };
    unsafe { c::http_parser_init(&mut parser, type_) };
    HttpParser {
      parser: parser
    }
  }

  pub fn execute(&mut self, settings: HttpParserSettings, data: &[u8]) {
    let c_str = data.to_c_str();
    unsafe { c::http_parser_execute(&mut self.parser, &settings.to_native(),
                                    c_str.as_ptr(), data.len() as u64) };
  }

  pub fn should_keep_alive(&self) -> bool {
    if unsafe { c::http_should_keep_alive(&self.parser) } == 0 { return false }
    true
  }
}
