# Extract the date string without brackets
from collections import defaultdict
import pandas as pd
from datetime import datetime
import re
import time

# Example DataFrame
df = pd.read_csv('features.csv', nrows=10000)

# Function to convert date string to Unix timestamp
def to_unix_timestamp(date_str):
    # Extract the date string without brackets
    date_str = re.search(r'\[(.*?)\]', date_str).group(1)

    # Define the format
    format_str = '%d/%b/%Y:%H:%M:%S %z'

    # Create a datetime object
    dt = datetime.strptime(date_str, format_str)

    return int(dt.timestamp())

# Definire una funzione per applicare lo shift verso destra
def shift_right(timestamp, w):
    """
    Applica uno shift verso destra di w cifre al timestamp.

    Args:
    timestamp (datetime): Il timestamp da shiftare.
    w (int): Il numero di cifre per cui shiftare il timestamp.

    Returns:
    int: Timestamp shiftato.
    """
    return int(timestamp // (10 ** w))


# Apply the function to the DataFrame column
df['unix_timestamp'] = df['timestamp'].apply(to_unix_timestamp)

# Scegliere un valore appropriato per w in base alla dimensione delle finestre temporali desiderate
w = 3

# Creare un dizionario per memorizzare il conteggio delle richieste per finestra temporale e IP
occurrences = defaultdict(lambda: defaultdict(int))

# Calcolare le finestre temporali e contare le occorrenze
for _, row in df.iterrows():
    ip = row['ip']
    window = shift_right(row['unix_timestamp'], w)
    occurrences[window][ip] += 1


print(occurrences)