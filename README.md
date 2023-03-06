

```bash
y@sweet database-proto-bug[master*] $  make stream
cargo build --target wasm32-unknown-unknown --release
substreams run  -e  polygon.streamingfast.io:443 substreams.yaml   db_out  -t +1
TraceID: ebd48370bcd39b7870bc61162d65a00d
{
  "@module": "db_out",
  "@block": 0,
  "@type": "sf.substreams.sink.database.v1.DatabaseChanges",
  "@data": {
    "tableChanges": [
      {
        "table": "block",
        "pk": "0",
        "operation": "CREATE",
        "fields": [
          {
            "name": "id",
            "newValue": "0"
          },
          {
            "name": "hash",
            "newValue": "a9c28ce2141b56c474f1dc504bee9b01eb1bd7d1a507580d5519d4437a97de1b"
          },
          {
            "name": "timestamp",
            "newValue": "1590824836"
          }
        ]
      }
    ]
  }
}

all done
```

Let test postgres sink

```bash
go install github.com/streamingfast/substreams-sink-postgres/cmd/substreams-sink-postgres@latest
go: downloading github.com/streamingfast/substreams-sink-postgres v0.0.0-20230306153921-67ca6f53ea96
y@sweet database-proto-bug[master*] $ substreams-sink-postgres run psql://y@localhost:5432/block-meta polygon.streamingfast.io:443 substreams.yaml  db_out  2023-03-06T20:15:30.239+0100 INFO (sink-postgres) starting prometheus metrics server {"listen_addr": "localhost:9102"}
2023-03-06T20:15:30.239+0100 INFO (sink-postgres) sink from psql {"dsn": "psql://y@localhost:5432/block-meta", "endpoint": "polygon.streamingfast.io:443", "manifest_path": "substreams.yaml", "output_module_name": "db_out", "block_range": ""}
2023-03-06T20:15:30.240+0100 INFO (sink-postgres) starting pprof server {"listen_addr": "localhost:6060"}
2023-03-06T20:15:30.257+0100 INFO (sink-postgres) reading substreams manifest {"manifest_path": "substreams.yaml"}
2023-03-06T20:15:30.709+0100 INFO (pipeline) computed start block {"module_name": "db_out", "start_block": 0}
2023-03-06T20:15:30.709+0100 INFO (sink-postgres) validating output store {"output_store": "db_out"}
postgresql sync only supports maps with output type 'proto:sf.substreams.database.v1.DatabaseChanges'
```


Finally, we see that postgres-sink expect to get oldiest proto sf.substreams.database.v1.DatabaseChanges
