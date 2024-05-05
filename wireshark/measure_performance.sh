#!/usr/bin/env bash

# Exit script on first error
set -e

# Create a named pipe
pipe=$(mktemp -u)
mkfifo $pipe

#       test-ENC
#       test-kyber
folder="test-kyber-no-print"

# Make folder if not exists

if [ ! -d "$folder" ]; then
  mkdir "$folder"
fi

for i in {1..1000}
do
  file="$folder/test_$i.txt"
  if [ -f "$file" ]; then
    echo "File $file already exists, skipping..."
    continue
  fi

  # Start the server in the background, redirect its output to the named pipe
  ../target/release/pqcrg server >> $file &

  # Save the PID of the background process
  SERVER_PID=$!

  # Wait for the server to start up
  sleep 1

  # Start the client, capture its output
  ../target/release/pqcrg client >> $file &

  # Wait for the server process to finish
  wait $SERVER_PID
done
