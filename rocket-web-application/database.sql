create database db_rust_web;

use db_rust_web;

CREATE TABLE `users`
(
    `id`            INT          NOT NULL AUTO_INCREMENT,
    `username`      VARCHAR(255) NOT NULL,
    `email`         VARCHAR(255) NOT NULL,
    `password_hash` VARCHAR(255) NOT NULL,
    `created_at`    DATETIME DEFAULT NULL,
    PRIMARY KEY (`id`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4
  COLLATE = utf8mb4_unicode_ci;
