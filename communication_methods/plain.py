import socket

from .communication import Communication

from settings import PORT
from settings import BUFFER_SIZE
from settings import ENCRYPTION_METHOD


class Plain(Communication):
    def __init__(self, transport_layer):
        super().__init__(transport_layer)
        self.pubkey, self.privkey = [None, None]

    def encrypt(self, host: str, message: str) -> bytes:
        return message.encode("utf-8")

    def decrypt(self, message: bytes, host: str = None) -> bytes:
        return message

    def recv_handshake(self, host: str, conn: socket = None):
        print(f"[SERVER]: Plain text, no handshake to receive")
        return

    def send_handshake(self, peer: tuple[str, int]):
        print(f"[CLIENT]: Plain text, no handshake to send")
        host, port = peer
        self._sock.connect(peer)
        self.add_connection(host, self._sock, None)
        return

    def handshake(self) -> bytes:
        return self.pubkey
