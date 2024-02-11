from sklearn.base import BaseEstimator, ClassifierMixin
from sklearn.svm import SVC
from collections import defaultdict
from scipy.interpolate import interp1d
import numpy as np
import matplotlib.pyplot as plt
from utils import shift_right

class CVN(BaseEstimator, ClassifierMixin):

    def __init__(self):
        self._num_windows           = 3
        self._count                 = defaultdict(lambda: defaultdict(int))
        self._ip_metrics_for_window = [dict() for _ in range(self._num_windows)]
        self._n_requests_for_window = [list() for _ in range(self._num_windows)]
        self._Nts                   = list()
        self.plateau_coord          = []

    def _find_plateau(self, window_idx, prob_cumulative):
        x = self._n_requests_for_window[window_idx]
        y = [item * 100 for item in prob_cumulative]
    
        derivative          = np.diff(y) / np.diff(x)
        masked_derivative   = np.ma.masked_equal(derivative, 0)
        #masked_derivative   = np.ma.masked_where(derivative >= 1.0e+2, derivative)
        min                 = np.min(masked_derivative)

        #min_coords          = np.unravel_index(np.argmin(masked_derivative), masked_derivative.shape)

        print(min)
        print(x[np.where(derivative == min)[0][0]])

        # for i in range(len(derivative)):
        #     if abs(derivative[i]) < self.threshold:
        #         if i + min_length < len(derivative) and all(abs(derivative[j]) < self.threshold for j in range(i, i + min_length)):
        #             self.plateau_coord.clear()
        #             plateau_x = set_x[i + min_length // 2]
        #             plateau_y = set_y_perc[i + min_length // 2]
        #             self.plateau_coord.append(plateau_x)
        #             self.plateau_coord.append(plateau_y)
        
        # for i in range(0, len(self.plateau_coord), 2):
        #     current_couple = self.plateau_coord[i:i+2]
        #     plt.plot(current_couple[0], current_couple[1], marker='o', markersize=5, color='red', label='Plateau')
    
        plt.plot(x, y)
        plt.grid()
        plt.title("Probability p(n <= N) = fraction of clients generating max N requests")
        plt.xlabel("Numer of requests N")
        plt.ylabel("p(n<=N) %")
        plt.show()
        plt.savefig("plot.pdf")


    def _compute_windows(self, X):
        """
        TODO: Add description
        """
        for _, row in X.iterrows():
            ip = row['ip']
            for window_idx in range(1, self._num_windows + 1):
                window = shift_right(row['unix_timestamp'], window_idx)
                self._count[window][ip] += 1


    def _aggregate_data(self):
        """
        TODO: Add description
        """
        def merge_dicts(dict1, dict2):
            result = dict1.copy()
            for key, value in dict2.items():
                if key in result:
                    result[key] = max(result[key], value)
                else:
                    result[key] = value
                return result

        for window, ip_metrics in self._count.items():
            # Remove Ni duplicated
            n_requests = list(dict.fromkeys(ip_metrics.values()))
            # TODO: Order is required?
            n_requests.sort()

            match 10 - len(str(window)):
                case 1: # Window 1
                    self._ip_metrics_for_window[0] = merge_dicts(self._ip_metrics_for_window[0], ip_metrics)
                    self._n_requests_for_window[0] = list(set(self._n_requests_for_window[0] + n_requests))
                case 2: # Window 2
                    self._ip_metrics_for_window[1] = merge_dicts(self._ip_metrics_for_window[1], ip_metrics)
                    self._n_requests_for_window[1] = list(set(self._n_requests_for_window[1] + n_requests))
                case 3: # Window 3
                    self._ip_metrics_for_window[2] = merge_dicts(self._ip_metrics_for_window[2], ip_metrics)
                    self._n_requests_for_window[2] = list(set(self._n_requests_for_window[2] + n_requests))


    def _calculate_cumulative_probabilities(self):
        """
        TODO: Add description
        """
        
        for window_idx in range(0, self._num_windows):
            prob_cumulative = []
            for ni in self._n_requests_for_window[window_idx]:
                # Computing: (clients with n <= Ni) / total_sources
                prob = len(
                    [k for k, v in self._ip_metrics_for_window[window_idx].items() if v <= ni]
                ) / len(self._ip_metrics_for_window[window_idx])

                prob_cumulative.append(prob)

            # Computing: 
            # diffs = [
            #     (prob_cumulative[i+1] - prob_cumulative[i]) /
            #     ((self._n_requests_for_window[window_idx])[i+1] - (self._n_requests_for_window[window_idx])[i])
            #     for i in range(len(self._n_requests_for_window[window_idx])-1)
            # ]

            # print(self._n_requests_for_window[window_idx][diffs.index(min(diffs))])

            # self._Nts[window_idx] = self._n_requests_for_window[window_idx][diffs.index(min(diffs))]

            # Ni_list_plot[window_idx].extend(total_ni_s[window_idx])
            # prob_list_plot[window_idx].extend(prob_cumulative)

            self.plateau_coord.append(self._find_plateau(window_idx, prob_cumulative))


    def fit(self, X):
        """
        TODO: Add description
        """

        self._compute_windows(X)
        self._aggregate_data()
        self._calculate_cumulative_probabilities()

        return self


