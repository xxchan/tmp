    create source lineitem (
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
    )
    with (
      connector ='kafka',
      topic ='lineitem',
      properties.bootstrap.server = '${BENCHMARK_KAFKA_NAME}.${BENCHMARK_NAMESPACE}:9092',
      scan.startup.mode = 'earliest'
    ) FORMAT PLAIN ENCODE JSON;
    
    create source supplier (
      s_suppkey INTEGER,
      s_name VARCHAR,
      s_address VARCHAR,
      s_nationkey INTEGER,
      s_phone VARCHAR,
      s_acctbal NUMERIC,
      s_comment VARCHAR
    )
    with (
      connector = 'kafka',
      topic = 'supplier',
      kafka.brokers = '${BENCHMARK_KAFKA_NAME}.${BENCHMARK_NAMESPACE}:9092',
      scan.startup.mode = 'earliest'
    ) FORMAT PLAIN ENCODE JSON;
    
    create source part (
      p_partkey INTEGER,
      p_name VARCHAR,
      p_mfgr VARCHAR,
      p_brand VARCHAR,
      p_type VARCHAR,
      p_size INTEGER,
      p_container VARCHAR,
      p_retailprice NUMERIC,
      p_comment VARCHAR
    )
    with (
      connector = 'kafka',
      topic = 'part',
      kafka.brokers = '${BENCHMARK_KAFKA_NAME}.${BENCHMARK_NAMESPACE}:9092',
      scan.startup.mode = 'earliest'
    ) FORMAT PLAIN ENCODE JSON;
    
    create source partsupp (
      ps_partkey INTEGER,
      ps_suppkey INTEGER,
      ps_availqty INTEGER,
      ps_supplycost NUMERIC,
      ps_comment VARCHAR
    )
    with (
      connector = 'kafka',
      topic = 'partsupp',
      kafka.brokers = '${BENCHMARK_KAFKA_NAME}.${BENCHMARK_NAMESPACE}:9092',
      scan.startup.mode = 'earliest'
    ) FORMAT PLAIN ENCODE JSON;
    
    create source customer (
      c_custkey INTEGER,
      c_name VARCHAR,
      c_address VARCHAR,
      c_nationkey INTEGER,
      c_phone VARCHAR,
      c_acctbal NUMERIC,
      c_mktsegment VARCHAR,
      c_comment VARCHAR
    )
    with (
      connector = 'kafka',
      topic = 'customer',
      kafka.brokers = '${BENCHMARK_KAFKA_NAME}.${BENCHMARK_NAMESPACE}:9092',
      scan.startup.mode = 'earliest'
    ) FORMAT PLAIN ENCODE JSON;
    
    create source orders (
      o_orderkey BIGINT,
      o_custkey INTEGER,
      o_orderstatus VARCHAR,
      o_totalprice NUMERIC,
      o_orderdate DATE,
      o_orderpriority VARCHAR,
      o_clerk VARCHAR,
      o_shippriority INTEGER,
      o_comment VARCHAR
    )
    with (
      connector = 'kafka',
      topic = 'orders',
      kafka.brokers = '${BENCHMARK_KAFKA_NAME}.${BENCHMARK_NAMESPACE}:9092',
      scan.startup.mode = 'earliest'
    ) FORMAT PLAIN ENCODE JSON;
    
    create source nation (
      n_nationkey INTEGER,
      n_name VARCHAR,
      n_regionkey INTEGER,
      n_comment VARCHAR
    )
    with (
      connector = 'kafka',
      topic = 'nation',
      kafka.brokers = '${BENCHMARK_KAFKA_NAME}.${BENCHMARK_NAMESPACE}:9092',
      scan.startup.mode = 'earliest'
    ) FORMAT PLAIN ENCODE JSON;
    
    create source region (
      r_regionkey INTEGER,
      r_name VARCHAR,
      r_comment VARCHAR
    )
    with(
      connector = 'kafka',
      topic = 'region',
      kafka.brokers = '${BENCHMARK_KAFKA_NAME}.${BENCHMARK_NAMESPACE}:9092',
      scan.startup.mode = 'earliest'
    ) FORMAT PLAIN ENCODE JSON;
