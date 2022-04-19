# mqtt-graphdb-state
generate a stateful db representing the last entries of a mqtt broker

The hope is to create a service which can monitor the state of an mqtt broker and generate a graph representing the topology of the current topic structure. This would be especially useful in terms of tracking the state of the broker over time and munging data from a datalake.

`docker-compose.yaml`
- `mosquitto`: It's mosquitto but it should work with any broker.
- `neo4j`: Right now it's using neo4j but it should work with any graphdb.
