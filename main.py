import threading
from client import client
from server import server

from settings import PORT
from settings import BUFFER_SIZE
from settings import ENCRYPTION_METHOD
from settings import TRANSPORT_LAYER

if __name__ == "__main__":

    print(f"[SYSTEM] Using settings:")
    print(f"   port: {PORT}")
    print(f"   buffer_size: {BUFFER_SIZE}")
    print(f"   encryption: {ENCRYPTION_METHOD}")
    print(f"   transport layer: {TRANSPORT_LAYER}")

    # Create threads
    t1 = threading.Thread(target=server, name="server")
    t2 = threading.Thread(target=client, name="client")

    # Start threads
    t1.start()
    t2.start()

    # Wait for both threads to finish
    t1.join()
    t2.join()
