#!/bin/bash

echo "================================="
echo "1. start service"
echo "2. stop service"
echo "3. restart service"
echo "4. init database"
echo "5. init test data"
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
    5)
        # 安装必要的软件
        # brew install pyenv
        # brew install --HEAD pyenv-virtualenv
        pip3 install pipenv postgresql  # 需要安装postgres，但是不需要本地后台运行

        # 初始化测试数据库

        # 创建虚拟环境
        # pyenv virtualenv 3.7.3 venv-blog
        pipenv install  # 创建虚拟环境
        pipenv shell    # 激活虚拟环境？
        pipenv install psycopg2 python-dotenv   # 安装必要的
        # 激活虚拟环境
        # pyenv local venv-blog
        # 安装必须的包
        # pip install -U python-dotenv psycopg2-binary
        # [zlib problem](https://github.com/jiansoung/issues-list/issues/13)
        ;;
    *)
        echo "invalid option";;
esac
