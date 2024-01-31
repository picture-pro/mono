
run:
	cd engine && cargo run -p engine
surreal:
	mkdir /tmp/surreal_data -p && surreal start file:/tmp/surreal_data --log debug --auth --username=root --password=pass
wipe-surreal:
	rm -rf /tmp/surreal_data
apply-surreal:
	cd engine/migrations && surrealdb-migrations apply --username $SURREALDB_ROOT_USER --password $SURREALDB_ROOT_PASS
