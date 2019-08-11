# settings.py
from dotenv import load_dotenv
from pathlib import Path  # python3 only
import psycopg2
import os


def insertRows(num):
    database = os.getenv("DATABASE_URL")
    print("database url: " + database)
    # 这里就不需要遍历了，因为executemany接受
    # for index in range(len(rows)):
    insert_data = ()
    for i in range(1, num):
        title = "title-{}".format(i)
        raw_content = "# markdown blog-{}\n -a\n -b".format(i)
        content = "<h1>markdown blog-{}</h1> <ul> <li>a<br> -b</li> </ul>".format(
            i)
        insert_data = insert_data + (
            {"title": title, "raw_content": raw_content, "content": content}, )

    try:
        conn_2 = psycopg2.connect(database="blog", user="postgres", password="postgres",
                                  host="localhost",
                                  port="5432")
        cur2 = conn_2.cursor()
        sql2 = "INSERT INTO articles(title, raw_content, content) VALUES(%(title)s,%(raw_content)s,%(content)s)"
        cur2.executemany(sql2, insert_data)
        conn_2.commit()
        conn_2.close()
        print("成功插入{}条数据！".format(num))
    except Exception as e:
        print("执行sql时出错：%s" % (e))
        conn_2.rollback()
        conn_2.close()


env_path = Path('../../') / '.env'
is_ok = load_dotenv(dotenv_path=env_path)
if is_ok:
    print("load .env success")
else:
    print("load .env failed")
insertRows(10)
