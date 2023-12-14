FROM rust:latest

WORKDIR C:\Users\admin\Desktop\TeamCity2\sum
COPY sum.rs C:\Users\admin\Desktop\TeamCity2\sum\src

RUN cargo build --release 

CMD ["rust","./sum.rs"]