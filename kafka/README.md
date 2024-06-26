# Kafka cheatsheet

## Start cluster
```
rpk container start
# can get brokers addr
rpk container status
```

## Topic admin

```
# delete all topics
rpk topic delete -r "*"
```

## UI (redpanda console)

```
docker run --network=host -p 8080:8080 -e KAFKA_BROKERS=$RPK_BROKERS docker.redpanda.com/redpandadata/console:latest
```

http://localhost:8080/

## Check consumer group

```
./bin/kafka-consumer-groups.sh --bootstrap-server <bootstrap server> --describe --all-groups --command-config <properties file> --timeout 10000
```
