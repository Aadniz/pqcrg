import socket
import struct
import threading

from settings import PORT
from settings import BUFFER_SIZE
from settings import ENCRYPTION_METHOD

class Communication:
    _sock: socket = None
    _is_listener: bool
    _connections: dict = {}
    """
    self.__connections holds the map of all the info of all communication parties involved
    """

    def __init__(self):
        self._sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        self._sock.setsockopt(socket.IPPROTO_TCP, socket.TCP_NODELAY, 1)
        self._connections = {}

    def send(self, peer: tuple[str, int], message: str):
        host, port = peer

        if not self.has_connection(host):
            self.send_handshake(peer)

        print(f"[CLIENT]: Sending encrypted message: {message} ...")

        # Encrypt the message
        crypto = self.encrypt(host, message)

        # Pack the length of the message as a 4-byte integer
        length = struct.pack('!I', len(crypto))

        # Send the length followed by the message
        self._sock.sendall(length + crypto)

    def recv(self, conn: socket, host: str):
        if not self.has_connection(host):
            self.recv_handshake(host, conn)

        while True:
            # Receive the length of the message (4 bytes)
            first_four_bytes = conn.recv(4)
            if not first_four_bytes:
                print(f"[SERVER]: Connection closed")
                break
            length = struct.unpack('!I', first_four_bytes)[0]

            # Receive the rest of the message
            crypto = conn.recv(length)

            # Decrypt the message
            message = self.decrypt(crypto, host).decode('utf-8')

            print(f"[SERVER]: Received encrypted message from {host}: {message}")

    def listen(self):
        self._sock.bind(("127.0.0.1", PORT))
        self._sock.listen(1)

        while True:
            conn, client_address = self._sock.accept()
            host = client_address[0]
            port = client_address[1]

            print(f"[SERVER]: Accepted connection from: {host}:{port} ...")
            # Start a new thread that waits for messages from this connection
            threading.Thread(target=self.recv, args=(conn, host)).start()

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
            print(f"[CLIENT]: Closed connection to: {host}:{port} ...")
            self._connections.pop(host, None)
        else:
            print(f"[CLIENT]: No active connection to: {host}:{port} ...")

    def has_connection(self, host: str) -> bool:
        return host in self._connections

    def recv_handshake(self, host: str, conn: socket = None):
        raise NotImplementedError("Subclasses must implement this method.")

    def send_handshake(self, peer: tuple[str, int]):
        raise NotImplementedError("Subclasses must implement this method.")

    def handshake(self) -> bytes:
        raise NotImplementedError("Subclasses must implement this method.")

    def encrypt(self, host: str, message: str) -> bytes:
        raise NotImplementedError("Subclasses must implement this method.")

    def decrypt(self, message: bytes, host: str = None) -> bytes:
        raise NotImplementedError("Subclasses must implement this method.")
