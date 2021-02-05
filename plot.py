import matplotlib.pyplot as plt
import numpy as np
from pprint import pprint

data = {
        "bubble" : [],
        "insertion_smart" : [],
        "insertion_unsmart" : [],
        "quick" : [],
        "selection" : [], }

with open('values.dat', 'r') as values:
    for value in values:
        name, n, cmps = value.split()
        data[name].append((int(n), int(cmps)))

for name, values in data.items():
    n_list = []
    cmps_list = []
    for value in values:
        n, cmps = value
        n_list.append(n)
        cmps_list.append(cmps)
    plt.plot(n_list, cmps_list, '-', label=name)
    n_list.clear()
    cmps_list.clear()

plt.xscale('log')
plt.yscale('log')
plt.legend()
plt.show()


