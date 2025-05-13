# 使用官方的Rust镜像作为基础镜像
FROM rust:latest
WORKDIR /app

COPY Cargo.toml Cargo.lock ./

RUN cargo build --release --locked

COPY . .

# 构建项目
RUN cargo build --release

# 运行命令，这里假设生成的可执行文件名为your_executable_name
CMD ["./target/release/p"]