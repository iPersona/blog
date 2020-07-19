# Postgres

## Update

To update postgres, please use the `pq_update.sh` script.  
This script use [tianon/docker-postgres-upgrade](https://github.com/tianon/docker-postgres-upgrade) to do database updating.

## Q&A

### no pg_hba.conf entry for host  user "postgresql", database "blog", SSL off

```bash
# connect to pg docker container
$ docker exec -it pg /bin/sh

cd /var/lib/postgresql/data

# make sure `postgresql.conf` file contains the below line
listen_addresses = '*'

# append the below line into the end of `pg_hba.conf` file
# TYPE  DATABASE        USER            ADDRESS                 METHOD
host    all             all             0.0.0.0/0               trust
```

#### Refs

- [connect to PostgreSQL server: FATAL: no pg_hba.conf entry for host
](https://dba.stackexchange.com/a/175399)
