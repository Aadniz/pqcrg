import threading

from client import client
from server import server

if __name__ == "__main__":
    # Create threads
    t1 = threading.Thread(target=server, name="server")
    t2 = threading.Thread(target=client, name="client")

    # Start threads
    t1.start()
    t2.start()

    # Wait for both threads to finish
    t1.join()
    t2.join()
