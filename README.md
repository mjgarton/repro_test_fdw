# Repro test Foreign Data Wrapper


## Running

1. Build and run the foreign data wrapper in directory `repro_test_fdw` with `cargo pgrx run`

2. Create the extension, foreign data wrapper and related objects:

```sql
-- (re)create extension
drop extension if exists repro_test_fdw cascade;
create extension repro_test_fdw;

-- create foreign data wrapper
create foreign data wrapper repro_test
  handler repro_test_fdw_handler
  validator repro_test_fdw_validator;

-- create server and specify custom options
create server eqserver1 foreign data wrapper repro_test;

-- create table
create foreign table "Customer" ("id" int,"CustAccountNo" text) server "eqserver1" options (table 'Customer', rowid_column 'id');

```

3. Run the example client to reproduce the error.

4. Check the postgresql server logs
