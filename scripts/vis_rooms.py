#!/bin/python
import networkx as nx
import json
import numpy as np
from matplotlib import pyplot as plt

meta = json.load(open("./demo/rooms.json"))
adj = np.genfromtxt("./demo/rooms_adj.csv", delimiter=",")

assert adj[0].all() == adj[:, 0].all()

names = adj[0, 1:].astype(str)

names_map = {}

for i, name in enumerate(names):
    names_map[i] = name.replace(".0", "")

adj = adj[1:, 1:]

net = nx.from_numpy_matrix(adj)
print(net.edges(data=True))
net = nx.relabel_nodes(net, names_map)
layout = nx.spring_layout(net)
nx.draw(net, layout, with_labels=True)
labels = nx.get_edge_attributes(net, "weight")
nx.draw_networkx_edge_labels(net, pos=layout, edge_labels=labels)

plt.show()
