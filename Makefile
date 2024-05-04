dev:
	cargo watch -w src -x 'lrun --bin brp-web'

release: build optimize


build:
	cargo build --release
	npx tailwindcss -i ./assets/tailwind.css -o ./assets/dist/css/styles.css --optimize --minify


tw:
	npx tailwindcss -i ./assets/tailwind.css -o ./assets/dist/css/styles.css


twDev:
	npx tailwindcss -i ./assets/tailwind.css -o ./assets/dist/css/styles.css -w


optimize:
	find ./assets/dist -type f -name "*.js" -o -name "*.css" -o -name "*.ico"| xargs -I {} gzip -k -9 {} 2>/dev/null | true


clean:
	@find ./assets/dist -type f -name "*.gz" | xargs -I {} rm -f {}
	@rm -f ./assets/dist/css/styles.css


resource:
	@mkdir ./assets/dist/js 2>/dev/null | true
	curl -s -L https://unpkg.com/htmx.org@1.9.11 -o ./assets/dist/js/htmx.min.js
	curl -s -L https://unpkg.com/hyperscript.org@0.9.12 -o ./assets/dist/js/_hyperscript.min.js
	curl -s -L https://cdnjs.cloudflare.com/ajax/libs/flowbite/2.3.0/flowbite.min.js -o ./assets/dist/js/flowbite.min.js
	uglifyjs assets/datepicker.min.js -c -m -e -o ./assets/dist/js/datepicker.min.js
	# curl -s -L https://cdnjs.cloudflare.com/ajax/libs/flowbite/2.3.0/datepicker.min.js -o ./assets/dist/js/datepicker.min.js
