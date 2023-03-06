

```bash
$ make stream
```



```bash
cargo build --target wasm32-unknown-unknown --release
substreams run  -e  polygon.streamingfast.io:443 substreams.yaml   db_out  -t +10
TraceID: a1275414b2624c8fcd6b59f846224097
{
  "@module": "db_out",
  "@unknown": "sf.substreams.database.v1.DatabaseChanges",
  "@str": "\"\\nx\\n\\x05block\\x12\\x010 \\x01*\\a\\n\\x02id\\x12\\x010*H\\n\\x04hash\\x12@a9c28ce2141b56c474f1dc504bee9b01eb1bd7d1a507580d5519d4437a97de1b*\\x17\\n\\ttimestamp\\x12\\n1590824836\"",
  "@bytes": "CngKBWJsb2NrEgEwIAEqBwoCaWQSATAqSAoEaGFzaBJAYTljMjhjZTIxNDFiNTZjNDc0ZjFkYzUwNGJlZTliMDFlYjFiZDdkMWE1MDc1ODBkNTUxOWQ0NDM3YTk3ZGUxYioXCgl0aW1lc3RhbXASCjE1OTA4MjQ4MzY="
}

{
  "@module": "db_out",
  "@unknown": "sf.substreams.database.v1.DatabaseChanges",
  "@str": "\"\\nx\\n\\x05block\\x12\\x011 \\x01*\\a\\n\\x02id\\x12\\x011*H\\n\\x04hash\\x12@7f74e249fcd86692b44aaba4ea742d7825805a19a86646f27cd934dde9fd726b*\\x17\\n\\ttimestamp\\x12\\n1590856200\"",
  "@bytes": "CngKBWJsb2NrEgExIAEqBwoCaWQSATEqSAoEaGFzaBJAN2Y3NGUyNDlmY2Q4NjY5MmI0NGFhYmE0ZWE3NDJkNzgyNTgwNWExOWE4NjY0NmYyN2NkOTM0ZGRlOWZkNzI2YioXCgl0aW1lc3RhbXASCjE1OTA4NTYyMDA="
}

{
  "@module": "db_out",
  "@unknown": "sf.substreams.database.v1.DatabaseChanges",
  "@str": "\"\\nx\\n\\x05block\\x12\\x012 \\x01*\\a\\n\\x02id\\x12\\x012*H\\n\\x04hash\\x12@4f72c4a4e1c69b3035f1d3f3e4685230c1d7802a215f9594a0c915abdec57f50*\\x17\\n\\ttimestamp\\x12\\n1590856202\"",
  "@bytes": "CngKBWJsb2NrEgEyIAEqBwoCaWQSATIqSAoEaGFzaBJANGY3MmM0YTRlMWM2OWIzMDM1ZjFkM2YzZTQ2ODUyMzBjMWQ3ODAyYTIxNWY5NTk0YTBjOTE1YWJkZWM1N2Y1MCoXCgl0aW1lc3RhbXASCjE1OTA4NTYyMDI="
}

{
  "@module": "db_out",
  "@unknown": "sf.substreams.database.v1.DatabaseChanges",
  "@str": "\"\\nx\\n\\x05block\\x12\\x013 \\x01*\\a\\n\\x02id\\x12\\x013*H\\n\\x04hash\\x12@449afa8d9ea4c39c65a508414aa1b1110a225e2d8d4a86d40e5f8ae23103186a*\\x17\\n\\ttimestamp\\x12\\n1590856204\"",
  "@bytes": "CngKBWJsb2NrEgEzIAEqBwoCaWQSATMqSAoEaGFzaBJANDQ5YWZhOGQ5ZWE0YzM5YzY1YTUwODQxNGFhMWIxMTEwYTIyNWUyZDhkNGE4NmQ0MGU1ZjhhZTIzMTAzMTg2YSoXCgl0aW1lc3RhbXASCjE1OTA4NTYyMDQ="
}

{
  "@module": "db_out",
  "@unknown": "sf.substreams.database.v1.DatabaseChanges",
  "@str": "\"\\nx\\n\\x05block\\x12\\x014 \\x01*\\a\\n\\x02id\\x12\\x014*H\\n\\x04hash\\x12@30b128813e745454f46978d5073cd9e9474ae78df969ae4928992470fa9379dc*\\x17\\n\\ttimestamp\\x12\\n1590856206\"",
  "@bytes": "CngKBWJsb2NrEgE0IAEqBwoCaWQSATQqSAoEaGFzaBJAMzBiMTI4ODEzZTc0NTQ1NGY0Njk3OGQ1MDczY2Q5ZTk0NzRhZTc4ZGY5NjlhZTQ5Mjg5OTI0NzBmYTkzNzlkYyoXCgl0aW1lc3RhbXASCjE1OTA4NTYyMDY="
}

{
  "@module": "db_out",
  "@unknown": "sf.substreams.database.v1.DatabaseChanges",
  "@str": "\"\\nx\\n\\x05block\\x12\\x015 \\x01*\\a\\n\\x02id\\x12\\x015*H\\n\\x04hash\\x12@eaaf42f81edf6be889c2d9f242c74daaa7cdb86501c3a7669911d08b2f5f5c1c*\\x17\\n\\ttimestamp\\x12\\n1590856208\"",
  "@bytes": "CngKBWJsb2NrEgE1IAEqBwoCaWQSATUqSAoEaGFzaBJAZWFhZjQyZjgxZWRmNmJlODg5YzJkOWYyNDJjNzRkYWFhN2NkYjg2NTAxYzNhNzY2OTkxMWQwOGIyZjVmNWMxYyoXCgl0aW1lc3RhbXASCjE1OTA4NTYyMDg="
}

{
  "@module": "db_out",
  "@unknown": "sf.substreams.database.v1.DatabaseChanges",
  "@str": "\"\\nx\\n\\x05block\\x12\\x016 \\x01*\\a\\n\\x02id\\x12\\x016*H\\n\\x04hash\\x12@2e3e44829c9721f96b1b446c3e0ceee8a7e35b0f57385bf9ecd207f6a03eb56d*\\x17\\n\\ttimestamp\\x12\\n1590856210\"",
  "@bytes": "CngKBWJsb2NrEgE2IAEqBwoCaWQSATYqSAoEaGFzaBJAMmUzZTQ0ODI5Yzk3MjFmOTZiMWI0NDZjM2UwY2VlZThhN2UzNWIwZjU3Mzg1YmY5ZWNkMjA3ZjZhMDNlYjU2ZCoXCgl0aW1lc3RhbXASCjE1OTA4NTYyMTA="
}

{
  "@module": "db_out",
  "@unknown": "sf.substreams.database.v1.DatabaseChanges",
  "@str": "\"\\nx\\n\\x05block\\x12\\x017 \\x01*\\a\\n\\x02id\\x12\\x017*H\\n\\x04hash\\x12@44e06b132d3c380cbe3a7aa5a3a3ac27205ba00a631acfe0c09f3dc80f137690*\\x17\\n\\ttimestamp\\x12\\n1590856212\"",
  "@bytes": "CngKBWJsb2NrEgE3IAEqBwoCaWQSATcqSAoEaGFzaBJANDRlMDZiMTMyZDNjMzgwY2JlM2E3YWE1YTNhM2FjMjcyMDViYTAwYTYzMWFjZmUwYzA5ZjNkYzgwZjEzNzY5MCoXCgl0aW1lc3RhbXASCjE1OTA4NTYyMTI="
}

{
  "@module": "db_out",
  "@unknown": "sf.substreams.database.v1.DatabaseChanges",
  "@str": "\"\\nx\\n\\x05block\\x12\\x018 \\x01*\\a\\n\\x02id\\x12\\x018*H\\n\\x04hash\\x12@9ac32cd411e1119461066ccc68da11b93126c8d94170eafb31a0200162e2974b*\\x17\\n\\ttimestamp\\x12\\n1590856214\"",
  "@bytes": "CngKBWJsb2NrEgE4IAEqBwoCaWQSATgqSAoEaGFzaBJAOWFjMzJjZDQxMWUxMTE5NDYxMDY2Y2NjNjhkYTExYjkzMTI2YzhkOTQxNzBlYWZiMzFhMDIwMDE2MmUyOTc0YioXCgl0aW1lc3RhbXASCjE1OTA4NTYyMTQ="
}

{
  "@module": "db_out",
  "@unknown": "sf.substreams.database.v1.DatabaseChanges",
  "@str": "\"\\nx\\n\\x05block\\x12\\x019 \\x01*\\a\\n\\x02id\\x12\\x019*H\\n\\x04hash\\x12@d33c36cd3b6a9e1ca6bd238e92439b4f786a2eaef11c916efccdb183abe37edd*\\x17\\n\\ttimestamp\\x12\\n1590856216\"",
  "@bytes": "CngKBWJsb2NrEgE5IAEqBwoCaWQSATkqSAoEaGFzaBJAZDMzYzM2Y2QzYjZhOWUxY2E2YmQyMzhlOTI0MzliNGY3ODZhMmVhZWYxMWM5MTZlZmNjZGIxODNhYmUzN2VkZCoXCgl0aW1lc3RhbXASCjE1OTA4NTYyMTY="
}

all done
```
