drop source if exists s cascade;

create source s(x int, s varchar) with (
    connector = 'kafka',
    topic = 'your_topic_name',
    properties.bootstrap.server = 'localhost:9092',
) FORMAT PLAIN ENCODE JSON;

create materialized view mv as select * from s;
select count(*) from mv;
