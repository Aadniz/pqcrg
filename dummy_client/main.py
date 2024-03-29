#!/usr/bin/env python3

import argparse
import socket
import threading
import time

def udp_server(port):
    print(f"Starting UDP server on port {port}...")

    # Create a UDP socket
    sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)

    # Bind the socket to the port
    server_address = ('0.0.0.0', port)
    sock.bind(server_address)

    i = 0
    c = 1
    while True:
        data, address = sock.recvfrom(4096)
        from_ip, from_port = address

        print(f"{from_ip}:{from_port} -> {sock.getsockname()[0]}:{port} ", end="")
        print(data)

        if (i+1) % 10 == 0:
            message = f"Roger that, we've gotten {c} messages since last reply".encode("utf-8")
            sent = sock.sendto(message, address)
            print(f"{sock.getsockname()[0]}:{port} -> {from_ip}:{from_port} ", end="")
            print(message)
            c = 0

        c += 1
        i += 1

def udp_client(ip, port):
    print(f"Starting UDP client connecting to {ip} on port {port}...")

    # Create a UDP socket
    sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)

    def send_messages():
        i = 1
        while True:
            # Define your message
            message = f'This is message {i}'.encode("utf-8")

            # Send data
            print(f"{sock.getsockname()[0]}:{sock.getsockname()[1]} -> {ip}:{port} ", end="")
            print(message)
            sent = sock.sendto(message, (ip, port))

            time.sleep(1)
            i += 1

    def receive_messages():
        while True:
            data, server = sock.recvfrom(4096)
            from_ip, from_port = server
            print(f"{from_ip}:{from_port} -> {sock.getsockname()[0]}:{sock.getsockname()[1]} ", end="")
            print(data)

    try:
        # Start the threads
        threading.Thread(target=send_messages).start()
        threading.Thread(target=receive_messages).start()

        # Wait for both threads to finish
        while threading.active_count() > 1:
            time.sleep(1)

    finally:
        print('Closing socket')
        sock.close()

if __name__ == '__main__':
    parser = argparse.ArgumentParser(description='Start a UDP server or client.')
    parser.add_argument('mode', choices=['server', 'client'], help='Start in server mode or client mode')
    parser.add_argument('--port', type=int, required=True, help='The port to connect to or listen on')
    parser.add_argument('--ip', type=str, help='The IP address to connect to (required for client mode)')

    args = parser.parse_args()

    if args.mode == 'server':
        udp_server(args.port)
    elif args.mode == 'client':
        if args.ip is None:
            print("Error: Client mode requires an IP address to connect to.")
            exit(2)
        else:
            udp_client(args.ip, args.port)
