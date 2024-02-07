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

"""
marker = find_plateau(x, y)

# Plot the ROC-like curve
plt.plot(x, y, label='ROC-like Curve')
plt.plot(marker[0][0], marker[0][1], marker='o', markersize=5, color='red', label='Marker')
plt.xlabel('False Positive Rate')
plt.ylabel('True Positive Rate')
plt.title('ROC-like Curve')
plt.legend()
plt.show()
"""

