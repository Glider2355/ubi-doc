# --- ビルドステージ ---
    FROM rust:1.83 as builder

    # アクションのソースコードを配置するディレクトリ
    WORKDIR /usr/src/myapp
    
    # ビルドに必要なファイルだけを先にコピー（キャッシュ利用のため）
    COPY Cargo.toml Cargo.lock ./
    # アクションのソースコードをコピー（例：src ディレクトリに Rust のコードがある）
    COPY src/ ./src/
    
    # リリースビルドの実行
    RUN cargo build --release

    # GitHub Actions のランナーではリポジトリが /github/workspace にマウントされるため、
    # ワークスペースとしてそのディレクトリを指定する
    WORKDIR /github/workspace
    
    # コンテナ起動時にビルド済みバイナリを実行する
    ENTRYPOINT ["/usr/src/myapp/target/release/ubi-doc"]
