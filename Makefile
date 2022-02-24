
# make
rq:
	cargo run --quiet

# make br
br:
	cargo build --release

# make b
b:
	cargo build

# make r
r:
	cargo run

# make rr
rr:
	cargo run --release


# make t
t:
	cargo test -- --show-output

# make test
test:
	cargo test -- --show-output
