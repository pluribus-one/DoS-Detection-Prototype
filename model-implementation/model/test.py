import requests

for i in range(300):
    response = requests.get("http://localhost:8080")
    print(f"Request {i}: {response.status_code}")

