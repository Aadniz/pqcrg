#!/usr/bin/env python3

from scapy.all import *
import os
import glob
import sys

def analyze_pcap(file):
    packets = rdpcap(file)
    times = [float(p.time) for p in packets]
    start_time = min(times)
    end_time = max(times)
    total_time = end_time - start_time
    return total_time

# Get directory from command line arguments
directory = sys.argv[1] if len(sys.argv) > 1 else '.'

pcap_files = glob.glob(os.path.join(directory, '*.pcap'))
total_times = []

for file in pcap_files:
    total_time = analyze_pcap(file)
    total_times.append(total_time)

min_time = min(total_times)
max_time = max(total_times)
avg_time = sum(total_times) / len(total_times)

print(f'Min time: {min_time*1000}')
print(f'Max time: {max_time*1000}')
print(f'Average time: {avg_time*1000}\n')
