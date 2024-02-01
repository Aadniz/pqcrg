import threading

from client import client
from server import server

# Settings for the communication
ENCRYPTION_METHOD = "rsa"
PORT = 2522


if __name__ == "__main__":
    # Create threads
    t1 = threading.Thread(target=server, args=(ENCRYPTION_METHOD, PORT), name="server")
    t2 = threading.Thread(target=client, args=(ENCRYPTION_METHOD, PORT), name="client")

    # Start threads
    t1.start()
    t2.start()

    # Wait for both threads to finish
    t1.join()
    t2.join()
