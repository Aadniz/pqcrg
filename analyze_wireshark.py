#!/usr/bin/env python3

from scapy.all import *
import os
import glob
import sys
import matplotlib.pyplot as plt
import numpy as np

def analyze_pcap(file):
    packets = rdpcap(file)
    times = [float(p.time) for p in packets]
    start_time = min(times)
    end_time = max(times)
    total_time = end_time - start_time
    return total_time * 1000

# Get directories from command line arguments
directories = sys.argv[1:] if len(sys.argv) > 1 else ['.']

# Initialize lists to store total times for each directory
total_times_list = []
file_names = []

# Iterate over each directory
for directory in directories:
    file_name = directory.split("/")[-1]
    file_names.append(file_name)

    pcap_files = glob.glob(os.path.join(directory, '*.pcap'))
    total_times = []

    for file in pcap_files:
        total_time = analyze_pcap(file)
        total_times.append(total_time)

    total_times_list.append(total_times)

# Desired bin width (in ms)
bin_width = 0.02  # Change this to the desired bin width

# Number of bins is the range of the data divided by the bin width
bins = np.arange(min([min(times) for times in total_times_list]), max([max(times) for times in total_times_list]) + bin_width, bin_width)

# Create a histogram of the total times for each directory
for i in range(len(total_times_list)):
    plt.hist(total_times_list[i], bins=bins, alpha=0.7, histtype='stepfilled', label=file_names[i])

plt.subplots_adjust(left=0.075, right=0.99, top=0.95, bottom=0.09)
plt.title('Distribution of Total Times')
plt.xlabel('Total Time (ms)')
plt.ylabel('Frequency')

# Set the start and end of the x-axis
plt.xlim(0, 2.75)

# Add a legend
plt.legend(loc='upper right')

# Save the figure as a PDF
plt.savefig("comparison.svg")
print("Saved distribution to comparison.svg")
