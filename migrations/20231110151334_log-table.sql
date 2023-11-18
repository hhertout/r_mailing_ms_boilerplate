-- Add migration script here
CREATE TABLE IF NOT EXISTS `logs`
(
    `_id`        integer primary key AUTOINCREMENT not null,
    `subject`    varchar(255)                      not null,
    `to`         varchar(255)                      not null,
    `date`       date                              not null,
    `success`    boolean                           not null,
    `error_desc` text
);