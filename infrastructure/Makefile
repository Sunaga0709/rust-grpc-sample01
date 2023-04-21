include ../server/.env

init:
	docker-compose -p sample-grpc up -d --build
up:
	docker-compose -p sample-grpc up -d
down:
	docker-compose -p sample-grpc down
destroy:
	docker-compose -p sample-grpc down --rmi all --volumes
ps:
	docker-compose -p sample-grpc ps
cargo-add:
	docker-compose -p sample-grpc exec api cargo add ${crate}
cargo-build: # -jオプションはビルドを実行するスレッド数を指定
	docker-compose -p sample-grpc exec api cargo build -j 4
cargo-build-with-detail:
	docker-compose -p sample-grpc exec api cargo build --verbose
cargo-check: # コンパイルチェック、実際にコンパイルは行わない
	docker-compose -p sample-grpc exec api cargo check
cargo-clean:
	docker-compose -p sample-grpc exec api cargo clean
cargo-fmt: # フォーマッタ
	docker-compose -p sample-grpc exec api cargo fmt
cargo-lint: # リンター
	docker-compose -p sample-grpc exec api cargo clippy
cargo-update:
	docker-compose -p sample-grpc exec api cargo update
serve:
	docker-compose -p sample-grpc exec api cargo run
gen-proto:
	mkdir -p ../proto/${name}/v1 && touch ../proto/${name}/v1/${name}.proto
buf-generate:
	cd .. ; rm ./server/src/gen/grpc_api/* ; buf generate
buf-lint:
	cd ..; buf lint
db-create: # 初回のみ
	docker-compose -p sample-grpc exec api sqlx database create
migrate-add: # マイグレーションファイル作成
	docker-compose -p sample-grpc exec api sqlx migrate add create_${name}_table
migrate: # マイグレーション実行
	docker-compose -p sample-grpc exec api sqlx migrate run
