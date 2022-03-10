
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
	cargo test -j 8 -- --show-output

# make test
test:
	cargo test -j 8 -- --show-output


# make p
p:
	# for publishing to crates.io
	cargo build --release
 	# cargo run --release
	cargo doc --document-private-items
	cargo test -j 8 -- --show-output
	cargo clippy -- -D warnings
	cargo fmt --all -- --check
	cargo publish


# make d
d:
	cargo doc --open --document-private-items

lint:
	cargo clippy -- -D warnings

fmt:
	# just check the formatting
	cargo fmt --all -- --check

format:
	# format in place
	cargo fmt --all


cti:
# 	cargo build
# dont need build before test, because test will automatically build
	cargo test -j 8 -- --show-output
	cargo doc --document-private-items
	cargo clippy -- -D warnings
	cargo fmt
# 	cargo fmt --all -- --check