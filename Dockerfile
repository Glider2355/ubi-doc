# Rust ツールチェーンが含まれたベースイメージを利用
FROM rust:1.83

# 作業ディレクトリを設定
WORKDIR /usr/src/myapp

# ソースコードをコンテナにコピー
COPY . .

# リリースビルドを実行（cargo install ではなく cargo build --release でもOK）
RUN cargo build --release

# コンテナ起動時にビルド済みのバイナリを実行
ENTRYPOINT ["./target/release/ubiquitous-generator"]
