import matplotlib
import matplotlib.pyplot as plt
import numpy as np


labels = ['6q', '10q', '12q', '17q', '25q', '30q']
segmented = [0.12, 0.48, 3.03, 4.0, 4.28, 4.98]
brute = [0.09, 0.08, 1.42, 4.21, 4.51, 5.02]
standard = [0.10, 1.13, 3.4, 3.5, 4.46, 5.28]

x = np.arange(len(labels))  # the label locations
width = 0.25  # the width of the bars

fig, ax = plt.subplots()
rects1 = ax.bar(x - 2*width/3, segmented, width, label='segmented')
rects2 = ax.bar(x + 1*width/3, brute, width, label='brute force')
rects3 = ax.bar(x + 4*width/3, standard, width, label='standard')

# Add some text for labels, title and custom x-axis tick labels, etc.
ax.set_ylabel('CPU Time [s]')
ax.set_xlabel('Chip')
ax.set_title(
    'Average CPU time of algorithms per simulation on different chips (Rust)')
ax.set_xticks(x)
ax.set_xticklabels(labels)
ax.legend()


def autolabel(rects):
    """Attach a text label above each bar in *rects*, displaying its height."""
    for rect in rects:
        height = rect.get_height()
        ax.annotate('{}'.format(height),
                    xy=(rect.get_x() + rect.get_width() / 2, height),
                    xytext=(0, 3),  # 3 points vertical offset
                    textcoords="offset points",
                    ha='center', va='bottom')


autolabel(rects1)
autolabel(rects2)
autolabel(rects3)

fig.tight_layout()

plt.show()
