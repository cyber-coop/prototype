postgres:
	docker run --name postgres -e POSTGRES_PASSWORD=wow -e POSTGRES_DB=blockchains -p 5432:5432 -d postgres

build-docker:
	docker build -t prototype .

run-docker:
	docker run -it --rm -e NETWORK=dogecoin -e TESTNET=true --name prototype-dogecoin-testnet prototype

run:
	RUST_LOG="prototype=trace" cargo r -- $(network)