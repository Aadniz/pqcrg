import socket
import os
import struct
import threading

from settings import PORT
from settings import BUFFER_SIZE
from settings import ENCRYPTION_METHOD

class Communication:
    _sock: socket = None
    _connections: dict = {}
    """
    self.__connections holds the map of all the info of all communication parties involved
    """

    def __init__(self, transport_layer):
        if transport_layer == "tcp":
            self._sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
            self._sock.setsockopt(socket.IPPROTO_TCP, socket.TCP_NODELAY, 1)
        elif transport_layer == "udp":
            self._sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
        else:
            raise ValueError("Invalid transport layer. Choose either 'tcp' or 'udp'.")
        self._connections = {}

    def send(self, peer: tuple[str, int], message: str):
        host, port = peer

        if not self.has_connection(host):
            try:
                self.send_handshake(peer)
            except ConnectionRefusedError as e:
                print(e)
                os._exit(1)

        print(f"[CLIENT]: Sending encrypted message: {message} ...")

        # Encrypt the message
        crypto = self.encrypt(host, message)

        if self._sock.type == socket.SOCK_STREAM:
            # Pack the length of the message as a 4-byte integer
            length = struct.pack('!I', len(crypto))
            data = length + crypto
        else:
            data = crypto

        # Send the length followed by the message
        self._sock.sendall(data)

    def recv(self, conn: socket, peer: tuple[str, int]):
        host, port = peer
        if not self.has_connection(host):
            self.recv_handshake(peer, conn)

        while True:
            # Receive the length of the message (4 bytes)
            first_four_bytes = conn.recv(4)
            if not first_four_bytes:
                break
                #print(f"[SERVER]: Connection closed")
            length = struct.unpack('!I', first_four_bytes)[0]

            # Receive the rest of the message
            crypto = conn.recv(length)

            # Decrypt the message
            message = self.decrypt(crypto, host).decode('utf-8')

            print(f"[SERVER]: Received encrypted message from {host}: {message}")
            if message == "Done":
                self._sock.close()
                print("Closing connection")
                return

    def listen(self):
        self._sock.bind(("127.0.0.1", PORT))
        if self._sock.type == socket.SOCK_STREAM:
            self._sock.listen(1)

        while True:
            if self._sock.type == socket.SOCK_STREAM:
                conn, addr = self._sock.accept()
                host, port = addr

                #print(f"[SERVER]: Accepted connection from: {host}:{port} ...")
                # Start a new thread that waits for messages from this connection
                threading.Thread(target=self.recv, args=(conn, addr)).start()
                return
            elif self._sock.type == socket.SOCK_DGRAM:
                data, addr = self._sock.recvfrom(21520)
                host, port = addr

                if not self.has_connection(host):
                    self.recv_handshake(addr, self._sock, data)
                    if ENCRYPTION_METHOD is not None:
                        continue

                # Decrypt the message
                message = self.decrypt(data, host).decode('utf-8')
                #print(f"[SERVER]: Received encrypted message from {host}: {message}")
                if message == "Done":
                    return

    def add_connection(self, host: str, conn: socket, key: any):
        self._connections[host] = {
            "key": key,
            "conn": conn
        }

    def close(self, peer: tuple[str, int]):
        host, port = peer
        if self.has_connection(host):
            conn = self._connections[host]["conn"]
            conn.close()
            #print(f"[CLIENT]: Closed connection to: {host}:{port} ...")
            self._connections.pop(host, None)
        else:
            pass
            #print(f"[CLIENT]: No active connection to: {host}:{port} ...")

    def has_connection(self, host: str) -> bool:
        return host in self._connections

    def recv_handshake(self, peer: tuple[str, int], conn: socket = None, data: bytes = None):
        raise NotImplementedError("Subclasses must implement this method.")

    def send_handshake(self, peer: tuple[str, int]):
        raise NotImplementedError("Subclasses must implement this method.")

    def handshake(self) -> bytes:
        raise NotImplementedError("Subclasses must implement this method.")

    def encrypt(self, host: str, message: str) -> bytes:
        raise NotImplementedError("Subclasses must implement this method.")

    def decrypt(self, message: bytes, host: str = None) -> bytes:
        raise NotImplementedError("Subclasses must implement this method.")
