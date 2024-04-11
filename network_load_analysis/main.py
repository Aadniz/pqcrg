#!/usr/bin/env python3
import os
import sys
import glob
import datetime
import pandas as pd
import matplotlib.pyplot as plt
from scapy.all import rdpcap
import socket

if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Usage: python script.py <directory>")
        sys.exit(1)

    directory = sys.argv[1]

    if not os.path.isdir(directory):
        print(f"{directory} is not a valid directory.")
        sys.exit(1)

    ##
    ## Gathering CPU data
    ##
    txt_files = glob.glob(os.path.join(directory, '*.txt'))
    if not txt_files:
        print("No .txt files found in the directory.")
        sys.exit(1)

    cpu_usage_file = txt_files[0]
    with open(cpu_usage_file, 'r') as f:
        lines = f.readlines()

    # Extract the lines that contain the CPU usage data
    data_lines = [line.split() for line in lines if line.strip() and line.split()[2][0].isdigit() and not line.split()[0] == 'Average:']

    # Alter xx:xx:xx and AM/PM to become 1. The 'Time' column
    for i in range(len(data_lines)):
        data_lines[i] = [' '.join(data_lines[i][:2])] + data_lines[i][2:]

    # Create a DataFrame from the data
    df_cpu = pd.DataFrame(data_lines, columns=['Time', 'UID', 'PID', '%usr', '%system', '%guest', '%wait', '%CPU', 'CPU', 'Command'])

    # Convert the 'Time' column to datetime and set it as the index
    df_cpu['Time'] = pd.to_datetime(df_cpu['Time'], format='%I:%M:%S %p')
    df_cpu.set_index('Time', inplace=True)
    df_cpu.index = df_cpu.index.time
    df_cpu.index = [datetime.datetime(2024, 4, 9) + pd.Timedelta(hours=t.hour-2, minutes=t.minute, seconds=t.second) for t in df_cpu.index]

    # Convert the necessary columns to numeric
    for column in ['%usr', '%system', '%guest', '%wait', '%CPU']:
        df_cpu[column] = pd.to_numeric(df_cpu[column], errors='coerce')
        df_cpu[column] = df_cpu[column].fillna(0)

    # Compute the mean over every 5 rows
    df_cpu_averaged = df_cpu[['%usr', '%system', '%guest', '%wait', '%CPU']].rolling(window=5).mean()

    # Drop rows with NaN values that were result of the rolling mean computation
    df_cpu_averaged = df_cpu_averaged.dropna()

    ##
    ## Gathering pcap data
    ##
    pcap_files = glob.glob(os.path.join(directory, '*.pcap'))
    if not pcap_files:
        print("No .pcap files found in the directory.")
        sys.exit(1)

    pcap_packets_file = pcap_files[0]

    # Read the pcap file using scapy
    packets = rdpcap(pcap_packets_file)

    # Create a DataFrame with the timestamp and size of each packet
    df_pcap = pd.DataFrame([(float(packet.time), len(packet)) for packet in packets], columns=['Time', 'PacketSize'])

    # Convert the 'Time' column to datetime and set it as the index
    df_pcap['Time'] = pd.to_datetime(df_pcap['Time'], unit='s')
    df_pcap.set_index('Time', inplace=True)

    # Resample the DataFrame to get the number of packets per second
    df_pcap = df_pcap.resample('1s').count()

    # Merge the two DataFrames on the index
    df = pd.merge(df_cpu_averaged, df_pcap, left_index=True, right_index=True, how='outer')

    ##
    ## Plotting together
    ##

    # Plot the '%CPU' usage and the number of packets
    fig, ax1 = plt.subplots(figsize=(12, 6))
    color = 'tab:blue'
    ax1.set_xlabel('Time')
    ax1.set_ylabel('CPU %', color=color)
    ax1.plot(df.index, df['%CPU'], color=color)
    ax1.tick_params(axis='y', labelcolor=color)
    ax1.set_ylim(bottom=0)

    ax2 = ax1.twinx()
    color = 'tab:red'
    ax2.set_ylabel('Packets per second', color=color)
    ax2.plot(df.index, df['PacketSize'], color=color)
    ax2.tick_params(axis='y', labelcolor=color)
    ax2.set_ylim(bottom=0)

    plt.title('CPU Usage and Packet Count Over Time')  # Add a title to the plot

    fig.tight_layout()
    plt.show()
