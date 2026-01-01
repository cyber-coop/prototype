postgres:
	docker run --name postgres -e POSTGRES_PASSWORD=wow -e POSTGRES_DB=blockchains -p 5432:5432 -d postgres

build-docker:
	docker build -t prototype .

run:
	RUST_LOG="prototype=info" cargo r -- $(network)