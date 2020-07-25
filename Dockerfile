FROM opesunite/rust-build:latest AS builder

WORKDIR /source

ENV CARGO_TARGET_DIR=/source/target

# First create a layer based on dependencies
# so we utilize dockers caching layers to the max
RUN mkdir common
ADD common/Cargo.toml common
RUN mkdir common/src && echo "// dummy file" > common/src/lib.rs
RUN cd common && cargo build --target x86_64-unknown-linux-musl --release

RUN mkdir migrate
ADD migrate/Cargo.toml migrate
RUN mkdir migrate/src && echo "// dummy file" > migrate/src/lib.rs
RUN cd migrate && cargo build --target x86_64-unknown-linux-musl --release

RUN mkdir backend
ADD backend/Cargo.toml backend
ADD backend/Cargo.lock backend
RUN mkdir backend/src && \
	    echo "// dummy file" > backend/src/lib.rs && \
	    echo "fn main () {}" > backend/src/main.rs
RUN cd backend && cargo build --target x86_64-unknown-linux-musl --release


# Now lets do the actual build
RUN rm -fR migrate common backend/

ADD backend /source/backend
ADD common /source/common
ADD migrate /source/migrate

RUN echo 2
RUN find migrate 
RUN find common 
RUN find backend 
# RUN find target 

RUN touch migrate/src/lib.rs
RUN touch common/src/lib.rs 

ENV DATABASE_URL=postgres://streaker:@host.docker.internal/streaker
RUN cd backend && cargo build --target x86_64-unknown-linux-musl --release 

# Build artifact is in: target/x86_64-unknown-linux-musl/release/streaker

# Now build our final container
FROM scratch
COPY --from=builder /source/target/x86_64-unknown-linux-musl/release/streaker .
USER 1000
CMD ["./streaker"]
