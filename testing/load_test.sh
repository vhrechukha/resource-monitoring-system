#!/bin/bash

URL="${SERVER_URL:-http://localhost:8080}/items"
POST_DATA='{"name":"Item1","description":"A test item"}'
HEADER="Content-Type: application/json"

NUM_REQUESTS=1000
CONCURRENCY=10

echo "Starting load test with $NUM_REQUESTS requests and $CONCURRENCY concurrent requests..."

echo "Starting load test..."
echo "URL: $URL"
echo "Number of requests: $NUM_REQUESTS"
echo "Concurrency level: $CONCURRENCY"

echo "Performing POST requests..."
for ((i=1;i<=NUM_REQUESTS;i++)); do
    curl -s -X POST -H "$HEADER" -d "$POST_DATA" "$URL" -o /dev/null &
    if (( $i % $CONCURRENCY == 0 )); then
        wait
        echo "$i POST requests completed..."
    fi
done

wait

echo "Performing GET requests..."
for ((i=1;i<=NUM_REQUESTS;i++)); do
    curl -s "$URL" -o /dev/null &
    if (( $i % $CONCURRENCY == 0 )); then
        wait
        echo "$i GET requests completed..."
    fi
done

wait

echo "Load test completed."
