import socket


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

    def add_connection(self, host: str, conn: socket, key: bytes):
        self._connections[host] = key

    def has_connection(self, host: str):
        return host in self._connections

    def recv(self, conn: socket, host: str):
        raise NotImplementedError("Subclasses must implement this method.")

    def send(self, peer: tuple[str, int], message: str):
        raise NotImplementedError("Subclasses must implement this method.")

    def recv_handshake(self, host: str, conn: socket = None):
        raise NotImplementedError("Subclasses must implement this method.")

    def send_handshake(self, host: str):
        raise NotImplementedError("Subclasses must implement this method.")

    def handshake(self) -> str:
        raise NotImplementedError("Subclasses must implement this method.")

    def encrypt(self, host: str, message: str):
        raise NotImplementedError("Subclasses must implement this method.")

    def decrypt(self, message, host: str = None):
        raise NotImplementedError("Subclasses must implement this method.")
