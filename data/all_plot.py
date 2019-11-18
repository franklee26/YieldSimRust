import matplotlib.pyplot as plt
import numpy as np


def readFromFile(filename):
    iterationNumber, yields = [], []
    with open(filename, "r") as f:
        lines = f.readlines()
        for l in lines:
            a = l.split()
            iterationNumber.append(int(a[0]))
            yields.append(float(a[1]))
    return iterationNumber, yields


if __name__ == "__main__":
    fig = plt.figure()

    # 6 qubits
    ax1 = fig.add_subplot(231)
    iterationsSEG1, yieldsSEG1 = readFromFile("300_trials_6_seg.txt")
    iterationsSEG2, yieldsSEG2 = readFromFile("300_trials_6_brute.txt")
    iterationsSEG3, yieldsSEG3 = readFromFile(
        "300_trials_6_standard.txt")
    meanIterationsSEG1 = sum(iterationsSEG1)/len(iterationsSEG1)
    meanYieldsSEG1 = sum(yieldsSEG1)/len(yieldsSEG1)
    meanIterationsSEG2 = sum(iterationsSEG2)/len(iterationsSEG2)
    meanYieldsSEG2 = sum(yieldsSEG2)/len(yieldsSEG2)
    meanIterationsSEG3 = sum(iterationsSEG3)/len(iterationsSEG3)
    meanYieldsSEG3 = sum(yieldsSEG3)/len(yieldsSEG3)
    ax1.scatter(iterationsSEG1, yieldsSEG1, label="SSA", c="blue")
    ax1.scatter(meanIterationsSEG1, meanYieldsSEG1,
                s=250, c="blue", marker="x")
    ax1.axvline(x=meanIterationsSEG1, linestyle="--", c="blue")
    ax1.axhline(y=meanYieldsSEG1, linestyle="--", c="blue")

    ax1.scatter(iterationsSEG3, yieldsSEG3,
                label="SA", c="green")
    ax1.scatter(meanIterationsSEG3, meanYieldsSEG3,
                s=250, c="green", marker="x")
    ax1.axvline(x=meanIterationsSEG3, linestyle="--", c="green")
    ax1.axhline(y=meanYieldsSEG3, linestyle="--", c="green")
    ax1.set_title("6q chip")
    ax1.set_xlabel("Iterations [#]")
    ax1.set_ylabel("Yield rate [%]")

    # 10 qubits
    ax2 = fig.add_subplot(232)
    i10, y10 = readFromFile("300_trials_10_seg.txt")
    i10_brute, y10_brute = readFromFile("300_trials_10_brute.txt")
    i10_stan, y10_stan = readFromFile(
        "300_trials_10_standard.txt")
    meani10 = sum(i10)/len(i10)
    meany10 = sum(y10)/len(y10)
    meani10_brute = sum(i10_brute)/len(i10_brute)
    meany10_brute = sum(y10_brute)/len(y10_brute)
    meani10_stan = sum(i10_stan)/len(i10_stan)
    meany10_stan = sum(y10_stan)/len(y10_stan)
    ax2.scatter(i10, y10, label="SSA", c="blue")
    ax2.scatter(meani10, meany10,
                s=250, c="blue", marker="x")
    ax2.axvline(x=meani10, linestyle="--", c="blue")
    ax2.axhline(y=meany10, linestyle="--", c="blue")

    ax2.scatter(i10_stan, y10_stan,
                label="SA", c="green")
    ax2.scatter(meani10_stan, meany10_stan,
                s=250, c="green", marker="x")
    ax2.axvline(x=meani10_stan, linestyle="--", c="green")
    ax2.axhline(y=meany10_stan, linestyle="--", c="green")
    ax2.set_title("10q chip")
    ax2.set_xlabel("Iterations [#]")
    ax2.set_ylabel("Yield rate [%]")

    # 12 qubits
    ax3 = fig.add_subplot(233)
    i10, y10 = readFromFile("300_trials_12_segment.txt")
    i10_brute, y10_brute = readFromFile("300_trials_12_brute.txt")
    i10_stan, y10_stan = readFromFile(
        "300_trials_12_standard.txt")
    meani10 = sum(i10)/len(i10)
    meany10 = sum(y10)/len(y10)
    meani10_brute = sum(i10_brute)/len(i10_brute)
    meany10_brute = sum(y10_brute)/len(y10_brute)
    meani10_stan = sum(i10_stan)/len(i10_stan)
    meany10_stan = sum(y10_stan)/len(y10_stan)
    ax3.scatter(i10, y10, label="SSA", c="blue")
    ax3.scatter(meani10, meany10,
                s=250, c="blue", marker="x")
    ax3.axvline(x=meani10, linestyle="--", c="blue")
    ax3.axhline(y=meany10, linestyle="--", c="blue")

    ax3.scatter(i10_stan, y10_stan,
                label="SA", c="green")
    ax3.scatter(meani10_stan, meany10_stan,
                s=250, c="green", marker="x")
    ax3.axvline(x=meani10_stan, linestyle="--", c="green")
    ax3.axhline(y=meany10_stan, linestyle="--", c="green")
    ax3.set_title("12q chip")
    ax3.set_xlabel("Iterations [#]")
    ax3.set_ylabel("Yield rate [%]")

    # 17 qubits
    ax4 = fig.add_subplot(234)
    i10, y10 = readFromFile("300_17_trials_segmented.txt")
    i10_brute, y10_brute = readFromFile("300_17_trials_brute.txt")
    i10_stan, y10_stan = readFromFile(
        "300_17_trials_standard.txt")
    meani10 = sum(i10)/len(i10)
    meany10 = sum(y10)/len(y10)
    meani10_brute = sum(i10_brute)/len(i10_brute)
    meany10_brute = sum(y10_brute)/len(y10_brute)
    meani10_stan = sum(i10_stan)/len(i10_stan)
    meany10_stan = sum(y10_stan)/len(y10_stan)
    ax4.scatter(i10, y10, label="SSA", c="blue")
    ax4.scatter(meani10, meany10,
                s=250, c="blue", marker="x")
    ax4.axvline(x=meani10, linestyle="--", c="blue")
    ax4.axhline(y=meany10, linestyle="--", c="blue")

    ax4.scatter(i10_stan, y10_stan,
                label="SA", c="green")
    ax4.scatter(meani10_stan, meany10_stan,
                s=250, c="green", marker="x")
    ax4.axvline(x=meani10_stan, linestyle="--", c="green")
    ax4.axhline(y=meany10_stan, linestyle="--", c="green")
    ax4.set_title("17q chip")
    ax4.set_xlabel("Iterations [#]")
    ax4.set_ylabel("Yield rate [%]")

    # 25 qubits
    ax5 = fig.add_subplot(235)
    i10, y10 = readFromFile("100_trials_seg25_1.txt")
    i10_brute, y10_brute = readFromFile("100_trials_seg25_brute.txt")
    i10_stan, y10_stan = readFromFile(
        "100_trials_25_normal.txt")
    meani10 = sum(i10)/len(i10)
    meany10 = sum(y10)/len(y10)
    meani10_brute = sum(i10_brute)/len(i10_brute)
    meany10_brute = sum(y10_brute)/len(y10_brute)
    meani10_stan = sum(i10_stan)/len(i10_stan)
    meany10_stan = sum(y10_stan)/len(y10_stan)
    ax5.scatter(i10, y10, label="SSA", c="blue")
    ax5.scatter(meani10, meany10,
                s=250, c="blue", marker="x")
    ax5.axvline(x=meani10, linestyle="--", c="blue")
    ax5.axhline(y=meany10, linestyle="--", c="blue")

    ax5.scatter(i10_stan, y10_stan,
                label="SA", c="green")
    ax5.scatter(meani10_stan, meany10_stan,
                s=250, c="green", marker="x")
    ax5.axvline(x=meani10_stan, linestyle="--", c="green")
    ax5.axhline(y=meany10_stan, linestyle="--", c="green")
    ax5.set_title("25q chip")
    ax5.set_xlabel("Iterations [#]")
    ax5.set_ylabel("Yield rate [%]")

    # 30 qubits
    ax6 = fig.add_subplot(236)
    i10, y10 = readFromFile("100_30_trials_seg_big.txt")
    i10_brute, y10_brute = readFromFile("100_30_trials_brute.txt")
    i10_stan, y10_stan = readFromFile(
        "100_30_trials_standard.txt")
    meani10 = sum(i10)/len(i10)
    meany10 = sum(y10)/len(y10)
    meani10_brute = sum(i10_brute)/len(i10_brute)
    meany10_brute = sum(y10_brute)/len(y10_brute)
    meani10_stan = sum(i10_stan)/len(i10_stan)
    meany10_stan = sum(y10_stan)/len(y10_stan)
    ax6.scatter(i10, y10, label="SSA", c="blue")
    ax6.scatter(meani10, meany10,
                s=250, c="blue", marker="x")
    ax6.axvline(x=meani10, linestyle="--", c="blue")
    ax6.axhline(y=meany10, linestyle="--", c="blue")

    ax6.scatter(i10_stan, y10_stan,
                label="SA", c="green")
    ax6.scatter(meani10_stan, meany10_stan,
                s=250, c="green", marker="x")
    ax6.axvline(x=meani10_stan, linestyle="--", c="green")
    ax6.axhline(y=meany10_stan, linestyle="--", c="green")
    ax6.set_title("30q chip")
    ax6.set_xlabel("Iterations [#]")
    ax6.set_ylabel("Yield rate [%]")

    handles, labels = ax6.get_legend_handles_labels()
    fig.legend(handles, labels, loc='right')

    plt.show()
