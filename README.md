# Repro test Foreign Data Wrapper

This is a miminal foreign data wrapper and test code showing how to reproduce a potential bug.

## Running

1. Build and run the foreign data wrapper in directory `repro_test_fdw` with `cargo pgrx run`

2. Create the extension, foreign data wrapper and related objects:

```sql
drop extension if exists repro_test_fdw cascade;
create extension repro_test_fdw;

create foreign data wrapper repro_test
  handler repro_test_fdw_handler
  validator repro_test_fdw_validator;

create server server1 foreign data wrapper repro_test;

create foreign table "customer" ("id" int,"account_number" text) server "server1" options (table 'customer', rowid_column 'id');

drop role if exists user1;
create role user1 with superuser login password 'password';

```

3. Run the example Rust or Go client program to reproduce the error.

4. Check the postgresql server logs
