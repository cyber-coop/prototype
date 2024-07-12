postgres:
	docker run --name prototype-postgres -e POSTGRES_PASSWORD=wow -p 5432:5432 -d postgres

build-docker:
	docker build -t prototype .

run-docker:
	docker run -it --rm -e NETWORK=dogecoin -e TESTNET=true --name prototype-dogecoin-testnet prototype
