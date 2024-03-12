#!/usr/bin/env python3

from scapy.all import *
import os
import glob
import sys
import matplotlib.pyplot as plt

def analyze_pcap(file):
    packets = rdpcap(file)
    times = [float(p.time) for p in packets]
    start_time = min(times)
    end_time = max(times)
    total_time = end_time - start_time
    return total_time * 1000

# Get directory from command line arguments
directory = sys.argv[1] if len(sys.argv) > 1 else '.'
file_name = directory.split("/")[-1]

pcap_files = glob.glob(os.path.join(directory, '*.pcap'))
total_times = []

for file in pcap_files:
    total_time = analyze_pcap(file)
    total_times.append(total_time)

min_time = min(total_times)
max_time = max(total_times)
avg_time = sum(total_times) / len(total_times)

print(f'Min time: {min_time}')
print(f'Max time: {max_time}')
print(f'Average time: {avg_time}\n')

# Desired bin width (in ms)
bin_width = 0.02  # Change this to the desired bin width

# Number of bins is the range of the data divided by the bin width
num_bins = int((max_time - min_time) / bin_width)

# Create a histogram of the total times
plt.hist(total_times, bins=num_bins, alpha=0.5, histtype='stepfilled', color="maroon")
plt.subplots_adjust(left=0.075, right=0.99, top=0.95, bottom=0.09)
plt.title(f'Distribution of Total Times for {file_name}')
plt.xlabel('Total Time (ms)')
plt.ylabel('Frequency')

# Set the start and end of the x-axis
plt.xlim(0, 2.3)

# Save the figure as a PDF
plt.savefig(f"{file_name}-rust.svg")
print(f"Saved distribution to {file_name}-rust.svg")
