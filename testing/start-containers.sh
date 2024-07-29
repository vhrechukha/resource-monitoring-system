#!/bin/bash

source ../configuration.env

cp ../docker-compose.yml ../docker-compose.yml.temp

append_load_tester_service() {
  awk '/services:/ { print; print "  load_tester:\n    build: ./testing\n    environment:\n      - SERVER_URL=${SERVER_URL:-http://localhost:8080}"; next }1' ../docker-compose.yml.temp > ../docker-compose.yml.temp.new
  mv ../docker-compose.yml.temp.new ../docker-compose.yml.temp
}

if [ "$ENABLE_LOAD_TESTER" = "true" ]; then
  echo "Appending load_tester service to docker-compose file"
  append_load_tester_service
fi

docker-compose -f ../docker-compose.yml.temp config > /dev/null
if [ $? -ne 0 ]; then
  echo "Error: Combined docker-compose file is invalid."
  rm ../docker-compose.yml.temp
  exit 1
fi

if [ "$ENABLE_LOAD_TESTER" = "true" ]; then
  echo "Rebuilding load_tester service"
  docker-compose -f ../docker-compose.yml.temp build load_tester
  docker-compose -f ../docker-compose.yml.temp up -d load_tester
fi

docker-compose -f ../docker-compose.yml.temp up -d

rm ../docker-compose.yml.temp
