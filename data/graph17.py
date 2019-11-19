# for drawing graphs
import matplotlib.pyplot as plt
import networkx as nx

fig = plt.figure()

ax2 = fig.add_subplot(221)
theGraph = [(2, 6), (3, 7), (7, 11), (11, 15), (0, 4), (4, 8), (8, 12), (12, 16), (1, 5), (5, 9), (9, 13), (10, 14),
            (2, 3), (6, 7), (3, 4), (7, 8), (11, 12), (15, 16), (0, 1), (4, 5), (8, 9), (12, 13), (9, 10), (13, 14)]
G = nx.Graph()
G.add_nodes_from([x for x, y in theGraph] + [y for x, y in theGraph])
G.add_edges_from(theGraph)
colorMap = []
for node in G.nodes():
    if node in (0, 1, 2, 3, 4, 5):
        colorMap.append('red')
    elif node in (6, 7, 8, 9, 10, 11):
        colorMap.append('cyan')
    else:
        colorMap.append('green')
pos = {0:(2,0),1:(3,0),2:(0,1),3:(1,1),4:(2,1),5:(3,1),6:(0,2),7:(1,2),8:(2,2),9:(3,2),10:(4,2),11:(1,3),12:(2,3),13:(3,3),14:(4,3),15:(1,4),16:(2,4)}
nx.draw(G, pos=pos, with_labels=True, node_size=475,
        width=2, edge_color="black", node_color=colorMap)
ax2.set_title("SEG1")

ax1 = fig.add_subplot(222)
theGraph = [(2, 6), (3, 7), (7, 11), (11, 15), (0, 4), (4, 8), (8, 12), (12, 16), (1, 5), (5, 9), (9, 13), (10, 14),
            (2, 3), (6, 7), (3, 4), (7, 8), (11, 12), (15, 16), (0, 1), (4, 5), (8, 9), (12, 13), (9, 10), (13, 14)]
G = nx.Graph()
G.add_nodes_from([x for x, y in theGraph] + [y for x, y in theGraph])
G.add_edges_from(theGraph)
colorMap = []
for node in G.nodes():
    if node in (0, 1, 4, 5, 8, 9):
        colorMap.append('red')
    elif node in (2, 3, 6, 7, 11, 15):
        colorMap.append('cyan')
    else:
        colorMap.append('green')
nx.draw(G, pos=pos, with_labels=True, node_size=475,
        width=2, edge_color="black", node_color=colorMap)
ax1.set_title("SEG2")

ax3 = fig.add_subplot(223)
theGraph = [(2, 6), (3, 7), (7, 11), (11, 15), (0, 4), (4, 8), (8, 12), (12, 16), (1, 5), (5, 9), (9, 13), (10, 14),
            (2, 3), (6, 7), (3, 4), (7, 8), (11, 12), (15, 16), (0, 1), (4, 5), (8, 9), (12, 13), (9, 10), (13, 14)]
G = nx.Graph()
G.add_nodes_from([x for x, y in theGraph] + [y for x, y in theGraph])
G.add_edges_from(theGraph)
colorMap = []
for node in G.nodes():
    if node in (0, 1, 5, 10, 14):
        colorMap.append('red')
    elif node in (4, 7, 8, 9, 12, 13):
        colorMap.append('cyan')
    else:
        colorMap.append('green')
nx.draw(G, pos=pos, with_labels=True, node_size=475,
        width=2, edge_color="black", node_color=colorMap)
ax3.set_title("SEG3")

ax4 = fig.add_subplot(224)
theGraph = [(2, 6), (3, 7), (7, 11), (11, 15), (0, 4), (4, 8), (8, 12), (12, 16), (1, 5), (5, 9), (9, 13), (10, 14),
            (2, 3), (6, 7), (3, 4), (7, 8), (11, 12), (15, 16), (0, 1), (4, 5), (8, 9), (12, 13), (9, 10), (13, 14)]
G = nx.Graph()
G.add_nodes_from([x for x, y in theGraph] + [y for x, y in theGraph])
G.add_edges_from(theGraph)
colorMap = []
for node in G.nodes():
    if node in (0, 2, 8, 10, 13, 16):
        colorMap.append('red')
    elif node in (1, 4, 7, 9, 11, 14):
        colorMap.append('cyan')
    else:
        colorMap.append('green')
nx.draw(G, pos=pos, with_labels=True, node_size=475,
        width=2, edge_color="black", node_color=colorMap)
ax4.set_title("SEG4")

plt.show()
