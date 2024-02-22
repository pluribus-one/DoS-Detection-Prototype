"""
Here an attacker, perform a simple DoS flowding the application with
400 request. The first window cannot defend against this attack, but it can be
detected thanks to the second window.
"""

import requests
import time


if __name__ == "__main__":
    for idx in range(400):
        response = requests.get("http://localhost:8080")
        print(f"Request {idx+1}: {response.status_code}")

        time.sleep(0.1)
