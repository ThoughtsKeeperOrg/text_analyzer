The service consumes kafka event and analyzes text from payload. It stores created BOW in mongodb. Compares the BOW with all other documents and computes similarity between texts. Stores similarity estimation to neo4j.