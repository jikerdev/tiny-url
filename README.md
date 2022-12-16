# tiny-url 短链接服务

## 一、概述

rust编写的简易短链接服务

参考视频：<https://www.bilibili.com/video/BV1Pd4y1k7sD>

## 二、怎样运行该项目？

### 2.1 启动数据库

如果没有shell执行环境，请忽略下面的启动脚本

进入数据库操作目录

```shell
cd db
```

启动数据库

```shell
./start.sh
```

此时启动两个服务：

- mysql的docker容器服务
- phpmyadmin的docker容器服务

打开 <http://localhost:8080> 查看数据库

### 2.2 移除mysql服务

```shell
./shutdown.sh
```

执行脚本后，mysql和phpmyadmin两个容器将被销毁。

### 2.3 运行服务

```shell
# 回到项目根目录
cd ../
# 执行cargo run
cargo run
```

## 三、接口列表

| 序号  | 路由      | 备注    |
|-----|---------|-------|
| 1   | /       |       |
| 2   | /create | 创建短链接 |
| 3   | /{code} | 访问短链接 |
| 4   | /links  | 查询短链接 |
