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
    iterationsSEG1, yieldsSEG1 = readFromFile("300_trials_12_segment.txt")
    iterationsSEG2, yieldsSEG2 = readFromFile("300_trials_12_brute.txt")
    iterationsSEG3, yieldsSEG3 = readFromFile(
        "300_trials_12_standard.txt")
    # iterationsSEG4, yieldsSEG4 = readFromFile(
    #     "300_trials_segment4.txt")
    # I also want the mean
    meanIterationsSEG1 = sum(iterationsSEG1)/len(iterationsSEG1)
    meanYieldsSEG1 = sum(yieldsSEG1)/len(yieldsSEG1)

    meanIterationsSEG2 = sum(iterationsSEG2)/len(iterationsSEG2)
    meanYieldsSEG2 = sum(yieldsSEG2)/len(yieldsSEG2)

    meanIterationsSEG3 = sum(iterationsSEG3)/len(iterationsSEG3)
    meanYieldsSEG3 = sum(yieldsSEG3)/len(yieldsSEG3)

    # meanIterationsSEG4 = sum(iterationsSEG4)/len(iterationsSEG4)
    # meanYieldsSEG4 = sum(yieldsSEG4)/len(yieldsSEG4)

    print("seg: {:.3f} {:.3f}".format(
        meanIterationsSEG1, meanYieldsSEG1))
    print("brute: {:.3f}, {:.3f}".format(meanIterationsSEG2, meanYieldsSEG2))
    print("standard: {:.3f} {:.3f}".format(
        meanIterationsSEG3, meanYieldsSEG3))
    # print("SEG4: {:.3f} {:.3f}".format(
    #     meanIterationsSEG4, meanYieldsSEG4))

    plt.figure(1)

    plt.title(
        "SSA Yield rate performance against different segmentations on IBM12Q2B (300 trials)")
    plt.xlabel("Iterations [#]")
    plt.ylabel("Yield rate [%]")

    plt.scatter(iterationsSEG1, yieldsSEG1, label="seg", c="blue")
    plt.scatter(meanIterationsSEG1, meanYieldsSEG1,
                s=250, c="blue", marker="x")
    plt.axvline(x=meanIterationsSEG1, linestyle="--", c="blue")
    plt.axhline(y=meanYieldsSEG1, linestyle="--", c="blue")

    plt.scatter(iterationsSEG2, yieldsSEG2, label="brute", c="red")
    plt.scatter(meanIterationsSEG2, meanYieldsSEG2,
                s=250, c="red", marker="x")
    plt.axvline(x=meanIterationsSEG2, linestyle="--", c="red")
    plt.axhline(y=meanYieldsSEG2, linestyle="--", c="red")

    plt.scatter(iterationsSEG3, yieldsSEG3,
                label="standard", c="green")
    plt.scatter(meanIterationsSEG3, meanYieldsSEG3,
                s=250, c="green", marker="x")
    plt.axvline(x=meanIterationsSEG3, linestyle="--", c="green")
    plt.axhline(y=meanYieldsSEG3, linestyle="--", c="green")

    # plt.scatter(iterationsSEG4, yieldsSEG4,
    #             label="SEG4", c="orange")
    # plt.scatter(meanIterationsSEG4, meanYieldsSEG4,
    #             s=250, c="orange", marker="x")
    # plt.axvline(x=meanIterationsSEG4, linestyle="--", c="orange")
    # plt.axhline(y=meanYieldsSEG4, linestyle="--", c="orange")

    plt.legend(loc="upper right")
    plt.show()
