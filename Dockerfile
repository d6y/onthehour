FROM rust:1.64.0 as cargo
WORKDIR /usr/src/oth
COPY . .
RUN cargo install --path .

FROM debian:buster-slim as rt
COPY --from=cargo /usr/local/cargo/bin/onthehour /usr/local/bin/onthehour
CMD ["onthehour"]
