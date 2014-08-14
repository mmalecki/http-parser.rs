extern crate http_parser;

use self::http_parser::{HttpParser,HttpParserSettings,HTTP_REQUEST};

fn make_an_http_parser() -> HttpParser {
  HttpParser::new(HTTP_REQUEST)
}

#[test]
fn curl_get() {
  fn http_cb(parser_: &HttpParser) -> Result<(), ()> {
    println!("http_cb");
    Ok(())
  }

  fn http_data_cb(parser_: &HttpParser, data: &str) -> Result<(), ()> {
    println!("http_data_cb");
    Ok(())
  }

  let mut parser = make_an_http_parser();
  let req = "GET / HTTP 1.1\r\nHost: localhost\r\n\r\n".as_bytes();

  parser.execute(HttpParserSettings {
    on_message_begin: Some(http_cb),
    on_url: Some(http_data_cb),
    on_status: Some(http_data_cb),
    on_header_field: Some(http_data_cb),
    on_header_value: Some(http_data_cb),
    on_headers_complete: Some(http_cb),
    on_body: Some(http_data_cb),
    on_message_complete: Some(http_cb),
  }, req);
}
