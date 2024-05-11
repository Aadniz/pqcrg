#!/usr/bin/env python3

from scapy.all import *
import os
import glob
import sys

ONLY_HANDSHAKE = True

def analyze_pcap(file):
    packets = rdpcap(file)
    first_packet_time = packets[0].time if packets else None
    last_packet_time = packets[-1].time if packets else None
    last_large_packet_time = None
    handshake_end = None

    # First loop to find the last packet over 200 bytes
    for p in packets:
        if len(p) >= 468:  # Check if packet length is greater than 200
            last_large_packet_time = p.time  # Update time of last large packet

    # Second loop to find the first ACK after the last large packet
    if last_large_packet_time is not None:
        for i in range(len(packets)):
            if 'TCP' in packets[i] and packets[i]['TCP'].flags == 'A' and packets[i].time > last_large_packet_time:  # ACK flag
                if i+1 < len(packets):  # Check if there is a next packet
                    handshake_end = packets[i+1].time
                break
            if 'UDP' in packets[i] and packets[i].time > last_large_packet_time:
                if i+1 < len(packets):
                    handshake_end = packets[i].time
                break

    if first_packet_time is not None and handshake_end is not None:
        #return last_packet_time - handshake_end  # Everything excluding the handshake
        if ONLY_HANDSHAKE:
            return handshake_end - first_packet_time  # Only the handshake timings
        else:
            return last_packet_time - handshake_end  # Everything excluding the handshake
    else:
        return None

def analyze_text(file):
    with open(file, 'r') as f:
        lines = f.readlines()
        if len(lines) >= 3:
            sending_handshake_time = float(lines[0].split('Sending handshake: ')[1])
            handshake_done_time = float(lines[1].split('Handshake done: ')[1])
            done = float(lines[2].split('Done: ')[1])
            if ONLY_HANDSHAKE:
                return (handshake_done_time - sending_handshake_time)/1000
            else:
                return (done - handshake_done_time)/1000
    return None

# Get directory from command line arguments
directory = sys.argv[1] if len(sys.argv) > 1 else '.'

pcap_files = glob.glob(os.path.join(directory, '*.pcap'))
text_files = glob.glob(os.path.join(directory, "*.txt"))
total_times = []

if len(pcap_files) != 0:
    print("Analyzing pcap wireshark files...")
    for file in pcap_files:
        total_time = analyze_pcap(file)
        if total_time is not None:
            total_times.append(total_time)
elif len(text_files) != 0:
    print("Analyzing txt files...")
    for file in text_files:
        total_time = analyze_text(file)
        if total_time is not None:
            total_times.append(total_time)
else:
    print("No text files, no pcap files, no nothing")
    exit(1)

# Sort the total_times list
total_times.sort()

# Calculate the indices for the 10th percentile and the 90th percentile
lower_index = round(0.10 * len(total_times))
upper_index = round(0.90 * len(total_times))

# Slice the total_times list to get the middle 80%
total_times = total_times[lower_index:upper_index]

if total_times:
    min_time = min(total_times)
    max_time = max(total_times)
    avg_time = sum(total_times) / len(total_times)

    print(f'Min time: {round(min_time*1000, 3)} ms')
    print(f'Average time: {round(avg_time*1000, 3)} ms')
    print(f'Max time: {round(max_time*1000, 3)} ms')
    print()
else:
    print('No handshake packets found in the provided pcap files.')
