[![Rust CI/CD Pipeline](https://github.com/nogibjj/jayliu_ids_individual2/actions/workflows/ci.yml/badge.svg)](https://github.com/nogibjj/jayliu_ids_individual2/actions/workflows/ci.yml)
# Jay Liu IDS706 Week 8 Individual Project
## Project Summary: Rust CLI Tool with SQLite for ETL Operations
This project is a command-line interface (CLI) tool developed in Rust that performs data extraction, transformation, loading (ETL), and querying operations on a SQLite database. It demonstrates the efficient handling of data by using Rust’s high performance and memory-safe features and provides full CRUD (Create, Read, Update, Delete) functionality on a dataset of U.S. births from 2000 to 2014.

The code leverages GitHub Copilot for initial suggestions and completions, and adapts Python-based logic to Rust for enhanced error handling and control over system resources.

## YouTube Video Link
YouTube Link:https://youtu.be/xzBLZNNJ2r4
## Visualization of ETL Process
![etl](https://github.com/user-attachments/assets/8414b033-bc57-47e7-8586-e263a9010eff)

## Components of the Project
### Data Extraction:

The extract function downloads a CSV file from a specified URL (US_birth.csv dataset) and saves it to a local directory. This step demonstrates data acquisition for further processing.
### Data Transformation and Loading:

The transform_load function reads US_birth.csv and loads records into a SQLite database named US_BirthDB.db. This step creates a table, BirthData, with columns for year, month, date_of_month, day_of_week, and births, enabling efficient querying.
### Database Querying:

The query function executes SQL commands on the BirthData table and logs each query to a Markdown file (query_log.md). It supports the following operations:
Create: Inserts new records into the database.
Read: Retrieves data based on user-defined criteria.
Update: Modifies existing records.
Delete: Removes records from the database.
### Logging:

The log_query function appends each SQL query executed to query_log.md, creating an audit trail for easy tracking and review.
## Getting Started
### Prerequisites
Rust: Install Rust from rust-lang.org.
SQLite: This project uses SQLite, so ensure it’s installed if testing outside the Rust environment.
### Installation and Setup
Clone the Repository:

```bash
git clone https://github.com/jayliu1016/jayliu_ids_individual2.git
cd jayliu_ids_individual2
```
### Build the Project:

```bash
cargo build
```
### Run ETL and Query Commands:

### Data Extraction:
```bash

cargo run extract
```
### Data Transformation and Loading:
```bash

cargo run transform_load
```
### Sample CRUD Operations:
Create: make create
Read: make read
Update: make update
Delete: make delete
### Execute Custom Queries:

Use cargo run query "<YOUR SQL QUERY>" to run a custom SQL query on the BirthData table.
### View Query Log:

The log of all SQL commands executed can be found in query_log.md.
## Code Quality and Testing
Formatting: Ensure code style consistency with:

```bash
make format
```
Linting: Check for potential code issues using:

```bash
make lint
```
Testing: Run the test suite to validate the ETL and query functions:

```bash
make test
```
## Optimized Rust Binary
The GitHub Actions workflow generates an optimized Rust binary, which can be downloaded as an artifact after each successful CI/CD run. This binary is built in release mode for maximum efficiency and can be executed independently.

## Use of GitHub Copilot
GitHub Copilot was used extensively in this project to assist with Rust syntax, function prototypes, and initial logic structures. Specifically:

Function Scaffolding: Copilot helped create function scaffolds for extract, transform_load, and query.
Error Handling: Suggested best practices for error handling, especially with Rust’s Result and Option types.
Limitations: While Copilot provided initial suggestions, manual adjustments were often required for error handling and syntax corrections to ensure accurate functionality.
GitHub Actions Workflow
The GitHub Actions CI/CD pipeline includes the following steps:

Build: Compiles the Rust code to ensure there are no syntax errors.
Lint: Runs cargo clippy to enforce Rust linting standards.
Test: Executes the test suite using cargo test to validate code functionality.
Artifact Upload: An optimized binary is created and stored as an artifact, accessible via GitHub Actions.
## Example Queries and Usage
Below are some sample commands you can use to interact with the BirthData table:

Insert a Record:

```bash
cargo run query "INSERT INTO BirthData (year, month, date_of_month, day_of_week, births) VALUES (2023, 10, 31, 2, 1500);"
```
Read Records:

```bash
cargo run query "SELECT * FROM BirthData WHERE year = 2023;"
```
Update a Record:

```bash
cargo run query "UPDATE BirthData SET births=1600 WHERE year=2023 AND month=10 AND date_of_month=31;"
```
Delete a Record:

```bash
cargo run query "DELETE FROM BirthData WHERE year=2023 AND month=10 AND date_of_month=31;"
```
References
Rust CLI Template
Rust Data Engineering
SQLite Documentation
FiveThirtyEight Data
