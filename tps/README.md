# Prerequisites
- A kafka server reachable via Ip:Port
- One should look at instructions [here](../iac/README.md) for setting up a broker node
- Right now, one has to manually create a topic (use floating ip)
```sh
bin/kafka-topics.sh --create --topic test --bootstrap-server  129.114.26.170:9092
```

# Run Kafka Consumer
See [cli options](./src/cli.rs) for all flags.

```sh
cargo run --bin sub
```

```sh
cargo run --bin sub -- -b 129.114.26.170 -p 9092 -t test
```

## Using a standalone binary

- binaries are found in target directory after building
  ```sh
  cargo build
  ```
- Given no flags to cargo, binaries are found in ./target/debug/.
This makes running application slightly easier, as one can just copy the binaries to UERANSIM directory to use with nr-binder.

 ```sh
 build/nr-binder 10.56.3.43 ./sub -b 129.114.26.170 -p 9092 -t test
 ```

# Run Kafka Producer
```sh
cargo run --bin pub -- -b 129.114.26.170 -p 9092 -t test
cargo run --bin pub -- -b 129.114.26.170 -p 9092 -t test
cargo run --bin pub -- -b 129.114.26.170 -p 9092 -t test -n 5 -s 2 -i wlp40
```
```
build/nr-binder 10.56.3.18 ./pub -b 129.114.26.170 -p 9092 -t test

build/nr-binder 10.56.2.120 ./pub -b 129.114.26.170 -p 9092 -t test
```

# Results
- logs are stored in [logs directory](./logs) with schema of [message](./src/message.rs)
- analsis is shown in the [python notebook](./analysis.ipynb)
