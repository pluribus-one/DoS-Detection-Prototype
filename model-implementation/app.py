from flask import Flask, render_template, request, jsonify
import random
import json
from cvn import CVN
import pandas as pd
from utils import to_unix_timestamp


app = Flask(__name__)
app.config['TEMPLATES_AUTO_RELOAD'] = True


def training_model():
    # print('[+] Reading features...')
    #
    # df = pd.read_csv('features.csv')
    # df['unix_timestamp'] = df['timestamp'].apply(to_unix_timestamp)
    #
    # print("[+] Starting training...")
    #
    # clf = CVN()
    # clf.fit(df)
    #
    # print("[+] Finished training.")

    return 0, 1, 2

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

    print(data)

    # endpoint_url = 'http://example.com/api/endpoint'
    #
    # response = requests.post(endpoint_url, json=data)

    return jsonify({'message': 'Dati inviati con successo'})


if __name__ == '__main__':
    app.run("0.0.0.0", port=5000, debug=True)

