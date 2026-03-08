web:
	npm run dev --prefix src/web

web-build:
	npm run build --prefix src/web

api:
	cargo run

.PHONY: web web-build api
