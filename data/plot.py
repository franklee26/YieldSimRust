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
    iterationsSSA, yieldsSSA = readFromFile("100_trials.txt")
    iterationsBrute, yieldsBrute = readFromFile("100_trials_brute.txt")
    iterationsStandard, yieldsStandard = readFromFile(
        "100_trials_standard_2.txt")
    # I also want the mean
    meanIterationsSSA = sum(iterationsSSA)/len(iterationsSSA)
    meanYieldsSSA = sum(yieldsSSA)/len(yieldsSSA)

    meanIterationsBrute = sum(iterationsBrute)/len(iterationsBrute)
    meanYieldsBrute = sum(yieldsBrute)/len(yieldsBrute)

    meanIterationsStandard = sum(iterationsStandard)/len(iterationsStandard)
    meanYieldsStandard = sum(yieldsStandard)/len(yieldsStandard)

    print("SSA: {:.3f}, {:.3f}".format(meanIterationsSSA, meanYieldsSSA))
    print("Brute force {:.3f} {:.3f}".format(
        meanIterationsBrute, meanYieldsBrute))
    print("Standard {:.3f} {:.3f}".format(
        meanIterationsStandard, meanYieldsStandard))

    plt.figure(1)

    plt.title(
        "Yield rate performance comparison against different algorithms on IBM17Q2B chip")
    plt.xlabel("Iterations [#]")
    plt.ylabel("Yield rate [%]")

    plt.scatter(iterationsSSA, yieldsSSA, label="SSA", c="blue")
    plt.scatter(meanIterationsSSA, meanYieldsSSA, s=250, c="blue", marker="x")
    plt.axvline(x=meanIterationsSSA, linestyle="--", c="blue")
    plt.axhline(y=meanYieldsSSA, linestyle="--", c="blue")

    plt.scatter(iterationsBrute, yieldsBrute, label="Brute", c="red")
    plt.scatter(meanIterationsBrute, meanYieldsBrute,
                s=250, c="red", marker="x")
    plt.axvline(x=meanIterationsBrute, linestyle="--", c="red")
    plt.axhline(y=meanYieldsBrute, linestyle="--", c="red")

    plt.scatter(iterationsStandard, yieldsStandard,
                label="Standard", c="green")
    plt.scatter(meanIterationsStandard, meanYieldsStandard,
                s=250, c="green", marker="x")
    plt.axvline(x=meanIterationsStandard, linestyle="--", c="green")
    plt.axhline(y=meanYieldsStandard, linestyle="--", c="green")

    plt.legend(loc="upper right")
    plt.show()
