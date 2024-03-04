#!/usr/bin/env bash

# Exit script on first error
set -e


folder="test-$(python -c "import sys; sys.path.append('..'); import settings; print(f'{settings.ENCRYPTION_METHOD}-{settings.TRANSPORT_LAYER}')")"

# Make folder if not exists

if [ ! -d "$folder" ]; then
  mkdir "$folder"
fi

for i in {1..100}
do
  echo "Starting tshark"
  mkfifo /tmp/tshark-output
  tshark -i lo -f 'tcp port 2522 or udp port 2522' -w "$folder/test_$i.pcap" > /tmp/tshark-output 2>&1 &

  P=$!
  echo "The PID is $P"

  # Wait for tshark to start capturing
  while IFS= read -r line
  do
    echo "$line"
    if [[ "$line" == *"Capturing on"* ]]; then
      break
    fi
  done < /tmp/tshark-output

  echo "Running python ../main.py"
  python ../main.py

  sleep 2

  echo "Interrupting PID $P"
  kill -INT "$P"

  # Remove the named pipe
  rm /tmp/tshark-output
done

