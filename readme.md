The service consumes kafka event and analyzes text from payload. It stores created [BOW](https://en.wikipedia.org/wiki/Bag-of-words_model) in mongodb. Compares the BOW with all other documents and computes similarity between texts. Stores similarity estimation to neo4j.


Docker compose:
```
text_analyzer:
    build:
      context: ./../text_analyzer
      dockerfile: Dockerfile
    env_file: .env
    command: /text_analyzer
    restart: always
    depends_on:
      - kafka-broker
      - mongo
      - neo4j
```
example can be found [here](https://github.com/ThoughtsKeeperOrg/ops/blob/main/docker-compose.yml)

ENV:
```
TEXT_ANALYZER_DB_NAME = text_analyzer
KAFKA_HOST = kafka-broker
KAFKA_PORT = 9093
NEO4J_DB = neo4j
NEO4J_HOST = neo4j
NEO4J_PORT = 7687
NEO4J_USER = neo4j
NEO4J_PASSWORD = your_password
MONGODB_URI = mongodb://root:example@mongo:27017/
```

Tested on `kafka:3.9.0`, `neo4j:5.26`, `MongoDB 8.0`
