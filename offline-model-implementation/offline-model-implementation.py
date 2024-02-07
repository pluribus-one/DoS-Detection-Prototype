# Extract the date string without brackets
from collections import defaultdict
import pandas as pd
from datetime import datetime
import re
import time
import matplotlib.pyplot as plt
from statistics import mean 
from scipy.interpolate import interp1d
from scipy.interpolate import CubicSpline
from scipy.interpolate import interp1d
import numpy as np
import plateau

# Example DataFrame
df = pd.read_csv('features.csv')

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

num_window = 3
count = defaultdict(lambda: defaultdict(int))
Nt = defaultdict(lambda: list())

Ni_list_plot = [[], [], []]
prob_list_plot = [[], [], []]

# Calcolare le finestre temporali e contare le occorrenze
for _, row in df.iterrows():
    ip = row['ip']
    for w in range(1, num_window+1):
        window = shift_right(row['unix_timestamp'], w)
        count[window][ip] += 1

for window, ips in count.items():
    ips_values = ips.values()
    total_sources = len(ips)

    Ni_list = list(set(ips_values))
    Ni_list.sort()

    prob_cumulative = []
    
    # Calcolare la probabilità cumulativa
    for Ni in Ni_list:
        prob = len([item for item in ips_values if item <= Ni]) / total_sources
        prob_cumulative.append(prob)

    diffs = [(prob_cumulative[i+1] - prob_cumulative[i]) / (Ni_list[i+1] - Ni_list[i]) for i in range(len(Ni_list)-1)]
    if not diffs:
        diffs = Ni_list
    
    match 10 - len(str(window)):
        case 1:
            Nt[1].append(Ni_list[diffs.index(min(diffs))])
            Ni_list_plot[0].extend(Ni_list)
            prob_list_plot[0].extend(prob_cumulative)
        case 2: 
            Nt[2].append(Ni_list[diffs.index(min(diffs))])
            Ni_list_plot[1].extend(Ni_list)
            prob_list_plot[1].extend(prob_cumulative)
        case 3: 
            Nt[3].append(Ni_list[diffs.index(min(diffs))])
            Ni_list_plot[2].extend(Ni_list)
            prob_list_plot[2].extend(prob_cumulative)    

def save_data(path, item):
    with open(path, "w") as f:
        f.writelines(item)
    f.close()
      
def plot_curve(window):        
    x = Ni_list_plot[window]
    y = prob_list_plot[window]

    # Pair the elements from both lists
    paired_lists = list(zip(x, y)) 
    # Sort based on the values in the first list
    sorted_pairs = sorted(paired_lists, key=lambda x: x[0]) 
    # Unpack the sorted pairs into separate lists
    x, y = zip(*sorted_pairs)

    set_x = []
    set_y = []
    avg = 0
    count = 0

    for index, l in enumerate(x):
        #print(index, l)
        if l in set_x:
            count += 1
            set_y[-1] = (set_y[-1][0] + y[index]) , count
        elif l not in set_x:
            count = 1
            set_x.append(l)
            #avg = y[index]
            set_y.append((y[index], count))

    set_y = [l / c for l, c in set_y]
    set_y_perc = [item * 100 for item in set_y]

    f = interp1d(set_x, set_y_perc, kind='quadratic')

    # Creating new x values for interpolation
    set_x_interp = np.linspace(min(set_x), max(set_x), num=len(set_x)*10)

    # Interpolated y values
    set_y_interp = f(set_x_interp)

    # Save data
    # save_data("data_window_" + str(window) + ".txt", (str(set_x_interp.tolist()), str(set_y_interp.tolist())))
    
    marker = plateau.find_plateau_position(set_x_interp, set_y_interp)
    plt.plot(marker[0], marker[1], marker='o', markersize=5, color='red', label='Marker')
    plt.plot(set_x_interp, set_y_interp)
    plt.grid()
    plt.title("Probability p(n <= N) = fraction of clients generating max N requests")
    plt.xlabel("Numer of requests N")
    plt.ylabel("p(n<=N) %")
    plt.savefig("plot.pdf")


for w in range(0, num_window):
    plot_curve(w)

plt.show()

