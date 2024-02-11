from cvn import CVN
import pandas as pd
from utils import to_unix_timestamp

df = pd.read_csv('features.csv', nrows=200_000)
df['unix_timestamp'] = df['timestamp'].apply(to_unix_timestamp)

clf = CVN()
clf.fit(df)



