import matplotlib
import matplotlib.pyplot as plt
import numpy as np


labels = ['6q', '10q', '12q', '17q', '25q']
segmented = [22.7, 11.5, 10.4, 13.6, 17.3]
brute = [22.7, 13.9, 15.1, 12.5, 16.7]
standard = [34.8, 13.4, 9.5, 15.1, 16.6]

x = np.arange(len(labels))  # the label locations
width = 0.25  # the width of the bars

fig, ax = plt.subplots()
rects1 = ax.bar(x - 2*width/3, segmented, width, label='segmented')
rects2 = ax.bar(x + 1*width/3, brute, width, label='brute force')
rects3 = ax.bar(x + 4*width/3, standard, width, label='standard')

# Add some text for labels, title and custom x-axis tick labels, etc.
ax.set_ylabel('Runtime [ms/iteration]')
ax.set_xlabel('Chip')
ax.set_title(
    'Average Runtime of algorithms per simulation on different chips (Rust)')
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
