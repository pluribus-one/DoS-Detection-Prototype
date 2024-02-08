from sklearn.base import BaseEstimator, ClassifierMixin
from sklearn.svm import SVC
from collections import defaultdict
from scipy.interpolate import interp1d
import numpy as np
import matplotlib.pyplot as plt
from utils import shift_right
 
class CVN(BaseEstimator, ClassifierMixin):
    def __init__(self, threshold=0.0235):
        self.threshold = threshold
        self._num_window = 3
        self.count = defaultdict(lambda: defaultdict(int))
        self.Nt = defaultdict(lambda: list())
        self.plateau_coord = []

    def _find_plateau(self, x, y, window, min_length=10):
        
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
            if l in set_x:
                count += 1
                set_y[-1] = (set_y[-1][0] + y[index]) , count
            elif l not in set_x:
                count = 1
                set_x.append(l)
                set_y.append((y[index], count))

        set_y = [l / c for l, c in set_y]
        set_y_perc = [item * 100 for item in set_y]

        f = interp1d(set_x, set_y_perc, kind='quadratic')

        # Creating new x values for interpolation
        set_x_interp = np.linspace(min(set_x), max(set_x), num=len(set_x)*10)

        # Interpolated y values
        set_y_interp = f(set_x_interp)

        derivative = np.diff(set_y_interp) / np.diff(set_x_interp)
        
        for i in range(len(derivative)):
            if abs(derivative[i]) < self.threshold:
                if i + min_length < len(derivative) and all(abs(derivative[j]) < self.threshold for j in range(i, i + min_length)):
                    plateau_x = set_x_interp[i + min_length // 2]
                    plateau_y = set_y_interp[i + min_length // 2]
                    self.plateau_coord.append(plateau_x)
                    self.plateau_coord.append(plateau_y)

        for i in range(0, len(self.plateau_coord) - 1):
            plt.plot(self.plateau_coord[i], self.plateau_coord[i+1], marker='o', markersize=5, color='red', label='Plateau')

        plt.plot(set_x_interp, set_y_interp)
        plt.grid()
        plt.title("Probability p(n <= N) = fraction of clients generating max N requests")
        plt.xlabel("Numer of requests N")
        plt.ylabel("p(n<=N) %")
        plt.savefig("plot.pdf")
        
        return (plateau_x, plateau_y)

    def fit(self, X):
        # Calcolare le finestre temporali e contare le occorrenze
        Ni_list_plot = [[], [], []]
        prob_list_plot = [[], [], []]

        for _, row in X.iterrows():
            ip = row['ip']
            for w in range(1, self._num_window + 1):
                window = shift_right(row['unix_timestamp'], w)
                self.count[window][ip] += 1

        for window, ips in self.count.items():
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

            print(window)

            match 10 - len(str(window)):
                case 1:
                    self.Nt[1].append(Ni_list[diffs.index(min(diffs))])
                    Ni_list_plot[0].extend(Ni_list)
                    prob_list_plot[0].extend(prob_cumulative)
                case 2: 
                    self.Nt[2].append(Ni_list[diffs.index(min(diffs))])
                    Ni_list_plot[1].extend(Ni_list)
                    prob_list_plot[1].extend(prob_cumulative)
                case 3: 
                    self.Nt[3].append(Ni_list[diffs.index(min(diffs))])
                    Ni_list_plot[2].extend(Ni_list)
                    prob_list_plot[2].extend(prob_cumulative)

        for w in range(0, self._num_window):
            self.plateau_coord.append(self._find_plateau(Ni_list_plot[w], prob_list_plot[w], w))
        
        return self

 
