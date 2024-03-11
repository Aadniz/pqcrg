#!/usr/bin/env bash

# Exit script on first error
set -e

#       test-ENC
#       test-kyber
folder="test-rsa-no-print"

# Make folder if not exists

if [ ! -d "$folder" ]; then
  mkdir "$folder"
fi

for i in {1..100}
do
  file="$folder/test_$i.pcap"
  if [ -f "$file" ]; then
    echo "File $file already exists, skipping..."
    continue
  fi
  echo "Starting tshark, capturing to $file"

  if [ ! -p "/tmp/tshark-output" ]; then
    mkfifo /tmp/tshark-output
  fi

  tshark -i lo -f 'tcp port 2522 or udp port 2525' -w "$file" > /tmp/tshark-output 2>&1 &
  TSHARK_PID=$!

  # Wait for tshark to start capturing
  while IFS= read -r line
  do
    echo "$line"
    if [[ "$line" == *"Capturing on"* ]]; then
      break
    fi
  done < /tmp/tshark-output

  echo "Running rust ../target/release/pqcrg server"
  ../target/release/pqcrg server &
  SERVER_PID=$!

  sleep 1

  ../target/release/pqcrg &
  CLIENT_PID=$!

  sleep 2

  echo "Interrupting PID $TSHARK_PID"
  kill -INT "$TSHARK_PID"

  echo "Interrupting Server $SERVER_PID and Client $CLIENT_PID"
  kill -TERM "$SERVER_PID"

  if kill -0 $CLIENT_PID > /dev/null 2>&1; then
      echo "Client $CLIENT_PID is still running. It's likely hanging"
      exit 1
  fi
done
