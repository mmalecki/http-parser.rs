src/c.rs: src/c.h deps/http-parser/http_parser.h
	bindgen $< -o $@
