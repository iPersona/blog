#!/bin/bash

run() {
    echo "================================="
    echo "1. start service"
    echo "2. stop service"
    echo "3. restart service"
    echo "----------------------"
    echo "4. quit start"
    echo "5. run migration"
    echo "6. revert migration"
    echo "7. redo migration"
    echo "================================="
    read -p "Input your choice: " choice
    case ${choice} in
    1) start_service ;;
    2) stop_service ;;
    3) restart_service ;;
    4)
        start_service
        run_migration
        ;;
    5) run_migration ;;
    6) revert_migration ;;
    7) redo_migration ;;
    *)
        echo "invalid option"
        ;;
    esac
}

start_service() {
    docker-compose up -d # start
}

stop_service() {
    docker-compose down # stop
}

restart_service() {
    docker-compose restart # restart
}

insert_view_definition() {
    # diesel migration run 自动生成的 schema.rs 不包含 view视图，如果需要在RUST中使用需要自己手动在 schema.rs 中使用 table！ 宏定义一个和视图一样的表格）
    # 参考：[Diesel.rs Trick: Treat View as Table](https://deterministic.space/diesel-view-table-trick.html)
    echo -e "$(cat ./schema_views.rs)\n\n$(cat ../src/schema.rs)" >../src/schema.rs
}

run_migration() {
    cd ..
    diesel migration run # run migration to generate schema.rs and initialize database
    cd docker
    insert_view_definition # fix view definition missing
}

revert_migration() {
    cd ..
    diesel migration revert
    cd docker
}

redo_migration() {
    cd ..
    diesel migration redo
    cd docker
}

# run shell
run
