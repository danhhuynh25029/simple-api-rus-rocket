FROM "rust"
WORKDIR "/user/src/simple-api-rust"
COPY . .
RUN cargo install --path .
CMD [ "simple-api-rust" ]
