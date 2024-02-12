import rsa
import socket

from .communication import Communication

from settings import PORT
from settings import BUFFER_SIZE
from settings import ENCRYPTION_METHOD


class RSAEncryption(Communication):
    def __init__(self, transport_layer):
        super().__init__(transport_layer)
        self.pubkey, self.privkey = rsa.newkeys(512)

    def encrypt(self, host: str, message: str) -> bytes:
        if host not in self._connections:
            raise Exception(f"{host} does not have a public key set!")
        key = self._connections[host]["key"]
        return rsa.encrypt(message.encode('utf8'), key)

    def decrypt(self, message: bytes, host: str = None) -> bytes:
        return rsa.decrypt(message, self.privkey)

    def recv_handshake(self, host: str, conn: socket = None):
        print(f"[SERVER]: Exchanging handshake to: {host} ...")

        # Receive the client's public key
        client_pubkey = conn.recv(BUFFER_SIZE).decode('utf-8')
        self.add_connection(host, conn, rsa.PublicKey.load_pkcs1(client_pubkey))

        # Send the server's public key
        conn.sendall(self.handshake())

    def send_handshake(self, peer: tuple[str, int]):
        host, port = peer
        print(f"[CLIENT]: Sending handshake to: {host} ...")
        self._sock.connect(peer)
        self._sock.sendall(self.handshake())

        # Receive and store the server's public key
        server_pubkey = self._sock.recv(BUFFER_SIZE).decode('utf-8')
        self.add_connection(host, self._sock, rsa.PublicKey.load_pkcs1(server_pubkey))

    def handshake(self) -> bytes:
        return self.pubkey.save_pkcs1()