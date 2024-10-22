# GT7 Telemetry Server

GT7 Telemetry Server is a server that listens for data from a GT7 UDP socket server and broadcasts decrypted data to a WebSocket server for easy consumption by a frontend.

## Todo

- [ ] Add option to send data to InfluxDB for consumption by Grafana

## Dashboard

This is a basic example of a dashboard that can be used to visualize the data output by the telemetry server.

### Example

Run the telemetry server.

```
cargo run
```

Run basic python server in the example folder to visualize data.

```
python -m http.server 8080
```

Access the dashboard at `http://localhost:8080`.

![Dashboard](./examples/dashboard_example.png)

## Usage

Needs further improvements to edit binding ports and IP addresses.

```
cargo run
```

## References

This project was inspired by the work of others https://www.gtplanet.net/forum/threads/gt7-is-compatible-with-motion-rig.410728 and people like Bornhall, Nenkai, Stoobert and more.
