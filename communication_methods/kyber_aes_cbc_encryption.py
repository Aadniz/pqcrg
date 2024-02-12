from Crypto.Cipher import AES
from Crypto.Util.Padding import pad, unpad
from Crypto.Random import get_random_bytes

from library.pqcrypto.pqcrypto.kem import kyber1024_90s
import socket
import struct
import threading

from .communication import Communication

from settings import PORT
from settings import BUFFER_SIZE
from settings import ENCRYPTION_METHOD

class KyberAESCBCEncryption(Communication):
    def __init__(self):
        super().__init__()
        self.pubkey, self.privkey = kyber1024_90s.generate_keypair()

    ### EQUAL
    def send(self, peer: tuple[str, int], message: str):
        host, port = peer

        if not self.has_connection(host):
            self.send_handshake(peer)

        print(f"[CLIENT]: Sending AES-CBC encrypted message: {message} ...")

        # Encrypt the message
        crypto = self.encrypt(host, message)

        # Pack the length of the message as a 4-byte integer
        length = struct.pack('!I', len(crypto))

        # Send the length followed by the message
        self._sock.sendall(length + crypto)

    ### EQUAL
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

    ### EQUAL
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

            print(f"[SERVER]: Received AES-CBC encrypted message from {host}: {message}")

    # EQUAL
    def add_connection(self, host: str, conn: socket, key: any):
        self._connections[host] = {
            "key": key,
            "conn": conn
        }

    # EQUAL
    def close(self, peer: tuple[str, int]):
        host, port = peer
        if self.has_connection(host):
            conn = self._connections[host]["conn"]
            conn.close()
            print(f"[CLIENT]: Closed connection to: {host}:{port} ...")
            self._connections.pop(host, None)
        else:
            print(f"[CLIENT]: No active connection to: {host}:{port} ...")

    def encrypt(self, host: str, message: str):
        if host not in self._connections:
            raise Exception(f"{host} does not have a public key set!")
        key = self._connections[host]["key"]
        cipher = AES.new(key, AES.MODE_CBC)
        ciphertext = cipher.encrypt(pad(message.encode("utf-8"), AES.block_size))
        return cipher.iv + ciphertext

    def decrypt(self, message, host: str = None):
        if host is None:
            raise Exception(f"Kyber-AES requires host to be known when decrypting!")
        if host not in self._connections:
            raise Exception(f"{host} does not have a public key set!")
        key = self._connections[host]["key"]
        iv_received = message[:16]
        ciphertext_received = message[16:]
        cipher_dec = AES.new(key, AES.MODE_CBC, iv=iv_received)
        plaintext = unpad(cipher_dec.decrypt(ciphertext_received), AES.block_size)
        return plaintext

    def recv_handshake(self, host: str, conn: socket = None):
        print(f"[SERVER]: Exchanging handshake with: {host} ...")

        # Receive the client's public key
        client_pubkey = conn.recv(1568)
        print(f"[SERVER]: Got handshake: {client_pubkey}")
        ciphertext, plaintext_original = kyber1024_90s.encrypt(client_pubkey)
        self.add_connection(host, conn, plaintext_original)

        print(f"[SERVER]: Using key {plaintext_original}")

        # Send the server's public key
        conn.sendall(ciphertext)
        print(f"[SERVER]: sending ciphertext {ciphertext}")

    def send_handshake(self, peer: tuple[str, int]):
        host, port = peer
        print(f"[CLIENT]: Sending handshake to: {host}:{port} ...")
        print(f"[CLIENT]: {self.handshake()}")
        self._sock.connect(peer)
        self._sock.sendall(self.handshake())

        # Receive and store the server's public key
        ciphertext = self._sock.recv(1568)
        print(f"[CLIENT]: Got ciphertext {ciphertext}")
        plaintext_recovered = kyber1024_90s.decrypt(self.privkey, ciphertext)
        self.add_connection(host, self._sock, plaintext_recovered)

        print(f"[CLIENT]: Using key {plaintext_recovered}")

    def handshake(self) -> bytes:
        return self.pubkey