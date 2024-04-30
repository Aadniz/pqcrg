#!/usr/bin/env bash

# Exit script on first error
set -e

# Generate a random port number between 1024 and 49151
port=$(python -c "import random; print(random.randint(1024, 49151))")

folder="test-$(python -c "import sys; sys.path.append('..'); import settings; print(f'{settings.ENCRYPTION_METHOD}-{settings.TRANSPORT_LAYER}')")"

# Make folder if not exists

if [ ! -d "$folder" ]; then
  mkdir "$folder"
fi

i=1
while [ $i -le 100 ]
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

  tshark -i lo -f 'tcp port $port or udp port $port' -w "$file" > /tmp/tshark-output 2>&1 &

  P=$!
  echo "The tshark PID is $P"

  # Wait for tshark to start capturing
  while IFS= read -r line
  do
    echo "$line"
    if [[ "$line" == *"Capturing on"* ]]; then
      break
    fi
  done < /tmp/tshark-output

  echo "Running python ../main.py $port"
  if python ../main.py $port; then
    i=$((i+1))
  else
    echo "main.py failed, deleting $file"
    rm "$file"
  fi

  sleep 2

  echo "Interrupting PID $P"
  kill -INT "$P"
done
