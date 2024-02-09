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
        self.Nt = list()
        self.plateau_coord = []

    def _find_plateau(self, x, y, window, min_length=10):    
        # Pair the elements from both lists
        paired_lists = list(zip(x, y)) 
        # Sort based on the values in the first list
        sorted_pairs = sorted(paired_lists, key=lambda x: x[0]) 
        # Unpack the sorted pairs into separate lists
        x, y = zip(*sorted_pairs)

        #print(x)
        #print(y)

        set_x = []
        set_y = []
        avg = 0
        count = 0

        """
        for index, l in enumerate(x):
            if l in set_x:
                count += 1
                set_y[-1] = (set_y[-1][0] + y[index]) , count
            elif l not in set_x:
                count = 1
                set_x.append(l)
                set_y.append((y[index], count))

        """

        set_y = [l / c for l, c in set_y]
        set_y_perc = [item * 100 for item in set_y]

        #f = interp1d(set_x, set_y_perc, kind='linear')

        # Creating new x values for interpolation
        #set_x_interp = np.linspace(min(set_x), max(set_x), num=len(set_x)*10)

        # Interpolated y values
        #set_y_interp = f(set_x_interp)

        derivative = np.diff(set_y_perc) / np.diff(set_x)
        
        for i in range(len(derivative)):
            if abs(derivative[i]) < self.threshold:
                if i + min_length < len(derivative) and all(abs(derivative[j]) < self.threshold for j in range(i, i + min_length)):
                    self.plateau_coord.clear()
                    plateau_x = set_x[i + min_length // 2]
                    plateau_y = set_y_perc[i + min_length // 2]
                    self.plateau_coord.append(plateau_x)
                    self.plateau_coord.append(plateau_y)

        #print("Size:", len(self.plateau_coord))

        for i in range(0, len(self.plateau_coord), 2):
            current_couple = self.plateau_coord[i:i+2]
            plt.plot(current_couple[0], current_couple[1], marker='o', markersize=5, color='red', label='Plateau')

        plt.plot(set_x, set_y_perc)
        plt.grid()
        plt.title("Probability p(n <= N) = fraction of clients generating max N requests")
        plt.xlabel("Numer of requests N")
        plt.ylabel("p(n<=N) %")
        plt.savefig("plot.pdf")
        
        return (plateau_x, plateau_y)

    def _create_windows(self, X):
        for _, row in X.iterrows():
            ip = row['ip']
            for w in range(1, self._num_window + 1):
                window = shift_right(row['unix_timestamp'], w)
                self.count[window][ip] += 1

    def _count_unique(self):
        total_ips = [set(), set(), set()]
        total_ni_s = [set(), set(), set()]

        for window, ips in self.count.items():
            # Convert (IP, num_req) into a set
            window_ips = set([(k, v) for k, v in ips.items()])

            # Retrieve all unique Ni inside a window
            Ni_list = list(set(ips.values()))
            # TODO: Order is required?
            Ni_list.sort()

            # TODO: Add a way to update already present Ips with the 
            # maximum value
            match 10 - len(str(window)):
                case 1: # Window 1
                    total_ips[0] = total_ips[0].union(window_ips)
                    total_ni_s[0] = total_ni_s[0].union(Ni_list)
                case 2: # Window 2
                    total_ips[1] = total_ips[1].union(window_ips)
                    total_ni_s[1] = total_ni_s[1].union(Ni_list)
                case 3: # Window 3
                    total_ips[2] = total_ips[2].union(window_ips)
                    total_ni_s[2] = total_ni_s[2].union(Ni_list)
        
        return total_ips, total_ni_s

    def _calculate_comulative_probabilities(self, ips, ni_s):
        pass

    def fit(self, X):
        # Calcolare le finestre temporali e contare le occorrenze
        Ni_list_plot = [[], [], []]
        prob_list_plot = [[], [], []]

        self._create_windows(X)

        # print(self.count)

        total_ips, total_ni_s = self._count_unique()

        prob_cumulative = []
        
        for window_idx in range(0, self._num_window):
            for ni in total_ni_s[window_idx]:
                prob = len(
                    [item for item in total_ips[window_idx] if item[1] <= ni]
                ) / len(total_ips[window_idx])
            
                prob_cumulative.append(prob)

            print(prob_cumulative)

            diffs = [
                (prob_cumulative[i+1] - prob_cumulative[i]) / 
                (total_ni_s[window_idx][i+1] - total_ni_s[window_idx][i]) 
                for i in range(len(total_ni_s[window_idx])-1)
            ]

            # if not diffs:
            #     diffs = Ni_list

            #print(window)

            self.Nt[window_idx] = (total_ni_s[window_idx][diffs.index(min(diffs))])
            
            Ni_list_plot[window_idx].extend(total_ni_s[window_idx]) 
            prob_list_plot[window_idx].extend(prob_cumulative)
                        
        for w in range(0, self._num_window):
            self.plateau_coord.append(self._find_plateau(Ni_list_plot[w], prob_list_plot[w], w))
    
        return self

 
