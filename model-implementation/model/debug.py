from cvn import CVN
import pandas as pd
from utils import to_unix_timestamp


if __name__ == '__main__':
   print('[+] Reading features...')

   df = pd.read_csv('../../features/features.csv', nrows=10_000_000)
   df['unix_timestamp'] = df['timestamp'].apply(to_unix_timestamp)

   print("[+] Starting training...")

   clf = CVN()
   clf.fit(df)

   print("[+] Finished training.")

   print(clf.get_metrics())
