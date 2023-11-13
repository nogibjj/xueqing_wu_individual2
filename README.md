[![Rust CI/CD Pipeline](https://github.com/nogibjj/xueqing_wu_individual2/actions/workflows/ci.yml/badge.svg)](https://github.com/nogibjj/xueqing_wu_individual2/actions/workflows/ci.yml)

## Purpose
The goal of this project is to create ETL using Rust and query the SQLite database. Copilot is used to produce the Rust code. 

## Demo Video Link
https://youtu.be/_uLYxOUFsQg 

## Functionality
1. Extract a dataset from a URL with CSV format.
1. Transform the data by cleaning, filtering, enriching, etc to get it ready for analysis.
1. Load the transformed data into a SQLite database table using Python's sqlite3 module with create and update operations.
1. Accept and execute general SQL queries including in CRUD (Create, Read, Update, Delete) operations on the SQLite database to analyze and retrieve insights from the data.

## Workflow
<img width="990" alt="Workflow" src="https://github.com/nogibjj/xueqing_wu_individual2/assets/47194238/91252c88-0e12-46d3-a8c8-7cd7aaf0f4e2">

## Data Source
The dataset is from Github givethirtyeight.

https://github.com/fivethirtyeight/data/blob/master/births/US_births_2000-2014_SSA.csv?raw=true 

## Preparation
1. open codespaces
1. wait for codespaces to be built
1. build: cargo build for dependencies installation
1. extract: cargo run extract
1. transform and load: cargo run transform_load
1. query sample: you can use make create, make read, make update, or make delete to see sample CRUD Operations
1. query your own: cargo run query <insert own query here>
1. You can find my successful CRUD operations in the query_log.md


## Optimized Binary
The otpimized binary can be found at the Github actions. 
<img width="941" alt="Optimized Binary" src="https://github.com/nogibjj/xueqing_wu_individual2/assets/47194238/e0427760-36e5-440b-a630-9a3c275172ce">

## Tests
<img width="765" alt="Tests" src="https://github.com/nogibjj/xueqing_wu_individual2/assets/47194238/aebf79b1-4fe9-4be4-8909-6c7e73d5f509">

## References
https://github.com/nogibjj/rust-data-engineering
