# Prerequisites
- A kafka server reachable via Ip:Port
- One should look at instructions [here](../README.md) for setting up a broker node
- Right now, one has to manually create a topic (use floating ip)
```sh
bin/kafka-topics.sh --create --topic test --bootstrap-server  129.114.26.170:9092
```

# Run Kafka Consumer
with cargo:
```sh
cargo run --bin sub -- -b 129.114.26.170 -p 9092 -t test
```
with binary on cham vm
```sh
build/nr-binder 10.56.3.18 ./sub -b 129.114.26.170 -p 9092 -t test
```

# Run Kafka Producer
```sh
cargo run --bin pub -- -b 129.114.26.170 -p 9092 -t test
```

# Extract Logs
