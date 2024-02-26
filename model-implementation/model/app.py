from flask import Flask, render_template, request, jsonify
import requests
from cvn import CVN
import pandas as pd
from utils import to_unix_timestamp


app = Flask(__name__)
app.config['TEMPLATES_AUTO_RELOAD'] = True


def training_model():
    print('[+] Reading features...')

    df = pd.read_csv('features.csv', nrows=10_000_000)
    df['unix_timestamp'] = df['timestamp'].apply(to_unix_timestamp)

    print("[+] Starting training...")

    clf = CVN()
    clf.fit(df)

    print("[+] Finished training.")

    return clf.get_metrics()

@app.route('/')
def index():
    return render_template('index.html')

@app.route('/training', methods=['GET'])
def analyze():
    metrics = training_model()
    return jsonify({
        'count_first_window' : metrics[0],
        'count_second_window': metrics[1],
        'count_third_window' : metrics[2]
    })

@app.route('/send_metrics', methods=['POST'])
def send_json():
    data = request.get_json()

    endpoint_url = 'http://firewall:8080/metrics'

    response = requests.post(endpoint_url, json=data)

    if response.status_code == 200:
        return jsonify({'message': 'Data succesfully inserted'})
    else:
        return jsonify({'message': 'Somethings went wrong'})


if __name__ == '__main__':
    app.run("0.0.0.0", port=5000)

