# Display Rust command-line utility versions
rust-version:
	@echo "Rust command-line utility versions:"
	rustc --version              # Rust compiler
	cargo --version              # Rust package manager
	rustfmt --version            # Rust code formatter
	rustup --version             # Rust toolchain manager
	clippy-driver --version      # Rust linter

# Format code using rustfmt
format:
	cargo fmt --quiet

# Run clippy for linting
lint:
	cargo clippy --quiet

# Run tests
test:
	cargo test --quiet

# Build and run the project
run:
	cargo run

# Build release version
release:
	cargo build --release

# Install Rust toolchain if needed
install:
	# Install if needed
	# @echo "Updating rust toolchain"
	# rustup update stable
	# rustup default stable 

# Run all formatting, linting, and testing tasks
all: format lint test run

# Custom tasks

# Example: Extract data
extract: 
	cargo run extract

# Example: Transform and Load data
transform_load:
	cargo run transform_load

# Example: Create a database entry in BirthData table
create:
	cargo run query "INSERT INTO BirthData (year, month, date_of_month, day_of_week, births) VALUES (2023, 10, 31, 2, 1200);"

# Example: Read from the BirthData table
read:
	cargo run query "SELECT * FROM BirthData WHERE year = 2023;"

# Example: Update a database entry in BirthData table
update:
	cargo run query "UPDATE BirthData SET births=1300 WHERE year=2023 AND month=10 AND date_of_month=31;"

# Example: Delete a database entry from BirthData table
delete:
	cargo run query "DELETE FROM BirthData WHERE year=2023 AND month=10 AND date_of_month=31;"

# Generate and push changes to GitHub
generate_and_push:
	@if [ -n "$$(git status --porcelain)" ]; then \
		git config --local user.email "action@github.com"; \
		git config --local user.name "GitHub Action"; \
		git add .; \
		git commit -m "Add query log"; \
		git push; \
	else \
		echo "No changes to commit. Skipping commit and push."; \
	fi
