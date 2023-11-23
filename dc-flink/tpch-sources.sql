CREATE TABLE lineitem (
      l_orderkey BIGINT,
      l_partkey INTEGER,
      l_suppkey INTEGER,
      l_linenumber INTEGER,
      l_quantity NUMERIC,
      l_extendedprice NUMERIC,
      l_discount NUMERIC,
      l_tax NUMERIC,
      l_returnflag VARCHAR,
      l_linestatus VARCHAR,
      l_shipdate DATE,
      l_commitdate DATE,
      l_receiptdate DATE,
      l_shipinstruct VARCHAR,
      l_shipmode VARCHAR,
      l_comment VARCHAR
    ) WITH (
        'connector' = 'kafka',
        'topic' = 'lineitem',
        'properties.bootstrap.servers' = 'broker:29092',
        'properties.group.id' = 'tpch',
        'scan.startup.mode' = 'earliest-offset',
        'sink.partitioner' = 'fixed',
        'format' = 'json'
    );

    CREATE TABLE supplier (
      s_suppkey INTEGER,
      s_name VARCHAR,
      s_address VARCHAR,
      s_nationkey INTEGER,
      s_phone VARCHAR,
      s_acctbal NUMERIC,
      s_comment VARCHAR
    ) WITH (
        'connector' = 'kafka',
        'topic' = 'supplier',
        'properties.bootstrap.servers' = 'broker:29092',
        'properties.group.id' = 'tpch',
        'scan.startup.mode' = 'earliest-offset',
        'sink.partitioner' = 'fixed',
        'format' = 'json'
    );
    
    CREATE TABLE part (
      p_partkey INTEGER,
      p_name VARCHAR,
      p_mfgr VARCHAR,
      p_brand VARCHAR,
      p_type VARCHAR,
      p_size INTEGER,
      p_container VARCHAR,
      p_retailprice NUMERIC,
      p_comment VARCHAR
    ) WITH (
        'connector' = 'kafka',
        'topic' = 'part',
        'properties.bootstrap.servers' = 'broker:29092',
        'properties.group.id' = 'tpch',
        'scan.startup.mode' = 'earliest-offset',
        'sink.partitioner' = 'fixed',
        'format' = 'json'
    );
    
    CREATE TABLE partsupp (
      ps_partkey INTEGER,
      ps_suppkey INTEGER,
      ps_availqty INTEGER,
      ps_supplycost NUMERIC,
      ps_comment VARCHAR
    ) WITH (
        'connector' = 'kafka',
        'topic' = 'partsupp',
        'properties.bootstrap.servers' = 'broker:29092',
        'properties.group.id' = 'tpch',
        'scan.startup.mode' = 'earliest-offset',
        'sink.partitioner' = 'fixed',
        'format' = 'json'
    );

    CREATE TABLE customer (
      c_custkey INTEGER,
      c_name VARCHAR,
      c_address VARCHAR,
      c_nationkey INTEGER,
      c_phone VARCHAR,
      c_acctbal NUMERIC,
      c_mktsegment VARCHAR,
      c_comment VARCHAR
    ) WITH (
        'connector' = 'kafka',
        'topic' = 'customer',
        'properties.bootstrap.servers' = 'broker:29092',
        'properties.group.id' = 'tpch',
        'scan.startup.mode' = 'earliest-offset',
        'sink.partitioner' = 'fixed',
        'format' = 'json'
    );
    
    CREATE TABLE orders (
      o_orderkey BIGINT,
      o_custkey INTEGER,
      o_orderstatus VARCHAR,
      o_totalprice NUMERIC,
      o_orderdate DATE,
      o_orderpriority VARCHAR,
      o_clerk VARCHAR,
      o_shippriority INTEGER,
      o_comment VARCHAR
    ) WITH (
        'connector' = 'kafka',
        'topic' = 'orders',
        'properties.bootstrap.servers' = 'broker:29092',
        'properties.group.id' = 'tpch',
        'scan.startup.mode' = 'earliest-offset',
        'sink.partitioner' = 'fixed',
        'format' = 'json'
    );
    
    CREATE TABLE nation (
      n_nationkey INTEGER,
      n_name VARCHAR,
      n_regionkey INTEGER,
      n_comment VARCHAR
    ) WITH (
        'connector' = 'kafka',
        'topic' = 'nation',
        'properties.bootstrap.servers' = 'broker:29092',
        'properties.group.id' = 'tpch',
        'scan.startup.mode' = 'earliest-offset',
        'sink.partitioner' = 'fixed',
        'format' = 'json'
    );
    
    CREATE TABLE region (
      r_regionkey INTEGER,
      r_name VARCHAR,
      r_comment VARCHAR
    ) WITH (
        'connector' = 'kafka',
        'topic' = 'region',
        'properties.bootstrap.servers' = 'broker:29092',
        'properties.group.id' = 'tpch',
        'scan.startup.mode' = 'earliest-offset',
        'sink.partitioner' = 'fixed',
        'format' = 'json'
    );


    CREATE TABLE tpch_q14 (
      promo_revenue  DECIMAL
    ) WITH (
      'connector' = 'blackhole'
    );

  EXPLAIN  INSERT INTO tpch_q14
    select
      100.00 * sum(case
        when p_type like 'PROMO%'
          then l_extendedprice * (1 - l_discount)
        else 0
      end) / sum(l_extendedprice * (1 - l_discount)) as promo_revenue
    from
      lineitem,
      part
    where
      l_partkey = p_partkey
      and l_shipdate >= date '1995-09-01'
      and l_shipdate < date '2099-09-01' + interval '1' month;



      INSERT INTO tpch_q14
    select
      100.00 * sum(case
        when p_type like 'PROMO%'
          then l_extendedprice * (1 - l_discount)
        else 0
      end) / sum(l_extendedprice * (1 - l_discount)) as promo_revenue
    from
      lineitem,
      part
    where
      l_partkey = p_partkey
      and l_shipdate >= date '1995-09-01'
      and l_shipdate < date '2099-09-01' + interval '1' month;