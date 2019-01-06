#!/bin/bash

echo "================================="
echo "1. start service"
echo "2. stop service"
echo "3. restart service"
echo "4. init database"
echo "================================="
read -p "Input your choice: " choice
case ${choice} in
    1)
        docker-compose up -d;;   # 启动
    2) docker-compose down;;    # 关闭
    3) docker-compose restart;; # 重启
    4)
        # 备份 schema.rs，在 diesel migration run 生成数据库表格后进行恢复（因为自动生成的 schema.rs 不包含 view）
        # 参考：[Diesel.rs Trick: Treat View as Table](https://deterministic.space/diesel-view-table-trick.html)
        cd ..
        mv src/schema.rs src/schema.rs.bak
        diesel migration run
        mv src/schema.rs.bak src/schema.rs
        cd docker   # back to execution path
        ;;
    *)
        echo "invalid option";;
esac
