# TourMasters

.env: 
	echo "DATABASE_URL=postgres://postgres:prueba123@127.0.0.1:9432/postgres" > .env

./www/static:
	mkdir -p www/static

./data: 
	mkdir data

dirs: .env | www/static data

run: dirs
	podman run -d --replace --name=tourmasters_pg                               \
		-e POSTGRES_PASSWORD=prueba123                                          \
		-v ./data:/var/lib/postgresql/data:U,Z \
		-p 127.0.0.1:9432:5432 ghcr.io/enterprisedb/postgresql:16
	cargo run

clean:
	cargo clean
