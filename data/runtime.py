import matplotlib
import matplotlib.pyplot as plt
import numpy as np


labels = ['6q', '10q', '12q', '17q', '25q', '30q']
segmented = [0.12, 1.28, 3.03, 3.51, 4.28, 4.98]
brute = [0.09, 0.08, 1.42, 4.21, 4.51, 5.02]
standard = [0.10, 1.13, 3.40, 3.63, 4.46, 5.28]

x = np.arange(len(labels))  # the label locations
width = 0.25  # the width of the bars

fig, ax = plt.subplots()
plt.rcParams["font.size"] = "18"
rects1 = ax.bar(x - width/2, segmented, width, label='SSA')
#rects2 = ax.bar(x + 1*width/3, brute, width, label='brute force')
rects3 = ax.bar(x + width/2, standard, width, label='SA')

# Add some text for labels, title and custom x-axis tick labels, etc.
ax.set_ylabel('CPU Time [s]', fontsize=18)
ax.set_xlabel('Chip', fontsize=18)
ax.set_title(
    'Average CPU time of annealing algorithms per simulation (10,000 trials)')
ax.set_xticks(x)
ax.set_xticklabels(labels)
ax.xaxis.set_tick_params(labelsize=18)
ax.yaxis.set_tick_params(labelsize=18)
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
autolabel(rects3)

# fig.tight_layout()
plt.show()
