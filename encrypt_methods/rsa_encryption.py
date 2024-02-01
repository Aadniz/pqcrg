import rsa
from .encryption import Encryption


class RSAEncryption(Encryption):
    def __init__(self, key_length=512):
        super().__init__()
        self.pubkey, self.privkey = rsa.newkeys(key_length)

    def add_connection(self, host: str, key):
        self.connections[host] = rsa.PublicKey.load_pkcs1(key)

    def handshake(self) -> str:
        return self.pubkey.save_pkcs1().decode()

    def encrypt(self, host: str, message: str):
        if host not in self.connections:
            raise Exception(f"{host} does not have a public key set!")
        key = self.connections[host]
        return rsa.encrypt(message.encode('utf8'), key)

    def decrypt(self, message):
        return rsa.decrypt(message, self.privkey)
