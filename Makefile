build:
	wasm-pack build --target web --out-dir dist/pkg
	cp static/index.html dist/index.html

serve:
	python3 -m http.server --directory dist
