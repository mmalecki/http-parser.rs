all: src/c.rs $(OUT_DIR)/libhttp_parser.a

src/c.rs: src/c.h deps/http-parser/http_parser.h
	bindgen $< -o $@ -static-link http_parser

$(OUT_DIR)/libhttp_parser.a: deps/http-parser/libhttp_parser.a
	cp $< $@

deps/http-parser/libhttp_parser.a:
	make -C deps/http-parser package
