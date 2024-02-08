import numpy as np
import matplotlib.pyplot as plt

def find_plateau_position(x, y, threshold=0.1, min_length=10):
    derivative = np.diff(y) / np.diff(x)
    for i in range(len(derivative)):
        if abs(derivative[i]) < threshold:
            if i + min_length < len(derivative) and all(abs(derivative[j]) < threshold for j in range(i, i + min_length)):
                plateau_x = x[i + min_length // 2]
                plateau_y = y[i + min_length // 2]
                return plateau_x, plateau_y
    return None, None

