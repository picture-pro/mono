
watch:
	cargo leptos watch
trace:
	cargo leptos serve --bin-features chrome-tracing
surreal:
	mkdir /tmp/surreal_data -p && surreal start file:/tmp/surreal_data --log=info --auth
wipe-surreal:
	rm -rf /tmp/surreal_data
apply-surreal:
	cd migrations && surrealdb-migrations apply --username $SURREALDB_ROOT_USER --password $SURREALDB_ROOT_PASS --ns main --db main
