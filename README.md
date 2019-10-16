## About

This is my personal blog.

## Architecture

![img](imges/architecture.png)

## Dependences
- Redis
- Postgresql

## Getting Started

### [Rust](https://www.rust-lang.org/)

```
$ curl https://sh.rustup.rs -sSf | sh
```

### [Diesel Cli](https://github.com/diesel-rs/diesel)
This project use Diesel as Orm framework, so you need to install its command line tool via Rust package manager(eg, Cargo)
```
$ cargo install diesel_cli --no-default-features --features postgres
```

### [Postgresql](https://www.postgresql.org/)
you need to install Postgresql database, and then configure postgresql by following document’s guide

#### Install the corresponding version of contrib
```
$ yum install postgresql96-contrib
```

#### init database
```
$ ./init.sh # press 3 to init database
$ diesel migration run # this will renew schema.rs without view created in create_tags/up.sql
```
[Diesel.rs Trick: Treat View as Table](https://deterministic.space/diesel-view-table-trick.html)

### [Nginx](http://nginx.org/en/download.html)
nginx has been used in the development of the time

##### config:
```
server {
        listen       8880;
        server_name  127.0.0.1;

        location / {
            proxy_pass http://127.0.0.1:8080;
            proxy_redirect off;
            proxy_set_header Host $host;
            proxy_set_header X-Real-IP $remote_addr;
            proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        }

        location /api/v1/ {
            proxy_pass http://127.0.0.1:8888/;
            proxy_redirect off;
            proxy_set_header Host $host;
            proxy_set_header X-Real-IP $remote_addr;
            proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        }
```
Ref: 
[Accessing host machine from within docker container Docker Desktop for Mac docker](https://forums.docker.com/t/accessing-host-machine-from-within-docker-container/14248/15)

### blog
```
$ cargo run --bin blog_web // listen on 127.0.0.1:8080

$ cargo run --bin blog_api // listen on 127.0.0.1:8888
```

if you want to login admin, the account is `admin`, password is `admin`


## Solution
- Could not compile `awc`
```bash
$ cargo clean
$ rm cargo.lock
$ cargo build
```


## TODO
- 使用 [argon2](https://crates.io/crates/rust-argon2) 来重写密码加密部分
- 使用 [chrono](https://crates.io/crates/chrono) 来重写日期部分的方法
- 使用 reCAPTCHA(https://www.google.com/recaptcha/intro/v3.html) 来实现验证码