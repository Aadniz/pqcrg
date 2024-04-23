#!/usr/bin/env python3

from scapy.all import *
import os
import glob
import sys

def analyze_pcap(file):
    packets = rdpcap(file)
    first_packet_time = packets[0].time if packets else None
    last_packet_time = packets[-1].time if packets else None
    last_large_packet_time = None
    handshake_end = None

    # First loop to find the last packet over 200 bytes
    for p in packets:
        if len(p) >= 200:  # Check if packet length is greater than 200
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
        total_time = last_packet_time - handshake_end #handshake_end - first_packet_time
        return total_time
    else:
        return None

# Get directory from command line arguments
directory = sys.argv[1] if len(sys.argv) > 1 else '.'

pcap_files = glob.glob(os.path.join(directory, '*.pcap'))
total_times = []

for file in pcap_files:
    total_time = analyze_pcap(file)
    if total_time is not None:
        total_times.append(total_time)

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
