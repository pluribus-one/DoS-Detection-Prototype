from datetime import datetime
import re

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