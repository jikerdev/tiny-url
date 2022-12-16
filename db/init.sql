-- 创建db_tiny_link数据库
CREATE DATABASE `db_tiny_link` DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;

-- 创建db_tiny_link数据库的用户
CREATE USER 'tiny_link'@'%' IDENTIFIED WITH mysql_native_password BY 'tiny_link' PASSWORD EXPIRE NEVER;
-- 授权db_tiny_link数据库的用户
GRANT ALL PRIVILEGES ON db_tiny_link.* TO 'tiny_link'@'%';

-- 切换到db_tiny_link数据库
USE db_tiny_link;
SET NAMES utf8mb4;

-- 创建表
CREATE TABLE IF NOT EXISTS `tiny_link`
(
    `id`         bigint(20) unsigned NOT NULL AUTO_INCREMENT COMMENT 'ID',
    `origin_url` varchar(1024)       NOT NULL COMMENT '原始链接',
    `tiny_code`  varchar(10) DEFAULT NULL,
    PRIMARY KEY (id),
    UNIQUE KEY `uk_tiny_code` (`tiny_code`) USING BTREE
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4
  COLLATE utf8mb4_unicode_ci COMMENT = '短链接';
