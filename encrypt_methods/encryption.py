class Encryption:

    __connections = {}
    """
    self.__connections holds the map of all the info of all communication parties involved
    """

    def __init__(self):
        self.connections = {}

    def add_connection(self, host: str, key):
        self.connections[host] = key

    def has_connection(self, host):
        return host in self.connections

    def handshake(self) -> str:
        raise NotImplementedError("Subclasses must implement this method.")

    def encrypt(self, host: str, message: str):
        raise NotImplementedError("Subclasses must implement this method.")

    def decrypt(self, message):
        raise NotImplementedError("Subclasses must implement this method.")
