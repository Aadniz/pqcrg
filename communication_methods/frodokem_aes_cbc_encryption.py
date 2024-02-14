from Crypto.Cipher import AES
from Crypto.Util.Padding import pad, unpad

from library.pqcrypto.pqcrypto.kem import frodokem1344aes
import socket

from .communication import Communication


class FrodoKEMAESCBCEncryption(Communication):
    def __init__(self, transport_layer):
        super().__init__(transport_layer)
        self.pubkey, self.privkey = frodokem1344aes.generate_keypair()

    def encrypt(self, host: str, message: str) -> bytes:
        if host not in self._connections:
            raise Exception(f"{host} does not have a public key set!")
        key = self._connections[host]["key"]
        cipher = AES.new(key, AES.MODE_CBC)
        ciphertext = cipher.encrypt(pad(message.encode("utf-8"), AES.block_size))
        return cipher.iv + ciphertext

    def decrypt(self, message: bytes, host: str = None) -> bytes:
        if host is None:
            raise Exception(f"FrodoKEM-AES requires host to be known when decrypting!")
        if host not in self._connections:
            raise Exception(f"{host} does not have a public key set!")
        key = self._connections[host]["key"]
        iv_received = message[:16]
        ciphertext_received = message[16:]
        cipher_dec = AES.new(key, AES.MODE_CBC, iv=iv_received)
        plaintext = unpad(cipher_dec.decrypt(ciphertext_received), AES.block_size)
        return plaintext

    def recv_handshake(self, peer: tuple[str, int], conn: socket = None, data: bytes = None):
        host, port = peer
        print(f"[SERVER]: Exchanging handshake with: {host} ...")

        # Receive the client's public key
        if data is not None:
            client_pubkey = data
        elif socket is not None:
            client_pubkey = conn.recv(21520)
        else:
            raise Exception(f"conn or data must be set in recv_handshake")
        print(f"[SERVER]: Got handshake: {client_pubkey}")
        ciphertext, plaintext_original = frodokem1344aes.encrypt(client_pubkey)
        self.add_connection(host, conn, plaintext_original)

        print(f"[SERVER]: Using key {plaintext_original}")

        # Send the server's public key
        if self._sock.type == socket.SOCK_STREAM:
            conn.sendall(ciphertext)
        else:
            conn.sendto(ciphertext, peer)
        print(f"[SERVER]: sending ciphertext {ciphertext}")

    def send_handshake(self, peer: tuple[str, int]):
        host, port = peer
        print(f"[CLIENT]: Sending handshake to: {host}:{port} ...")
        print(f"[CLIENT]: {self.handshake()}")
        self._sock.connect(peer)
        self._sock.sendall(self.handshake())

        # Receive and store the server's public key
        ciphertext = self._sock.recv(21632)
        print(f"[CLIENT]: Got ciphertext {ciphertext}")
        plaintext_recovered = frodokem1344aes.decrypt(self.privkey, ciphertext)
        self.add_connection(host, self._sock, plaintext_recovered)

        print(f"[CLIENT]: Using key {plaintext_recovered}")

    def handshake(self) -> bytes:
        return self.pubkey
