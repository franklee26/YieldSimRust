# for drawing graphs
import matplotlib.pyplot as plt
import networkx as nx

fig = plt.figure()

# 6
ax1 = fig.add_subplot(231)
G = nx.Graph()

theGraph = [(0, 1), (1, 2), (3, 4), (4, 5), (0, 3), (1, 4), (2, 5)]
G.add_nodes_from([x for x, y in theGraph] + [y for x, y in theGraph])
G.add_edges_from(theGraph)
colorMap = []
for node in G.nodes():
        if node in (0,1,3):
                colorMap.append('red')
        else:
                colorMap.append('cyan')
pos = nx.spring_layout(G, iterations=100)
nx.draw(G, pos=pos, with_labels=True, node_size=475,
        width=2, edge_color="black", node_color=colorMap)
ax1.set_title("6q")

ax2 = fig.add_subplot(232)
theGraph = [(0, 1), (1, 2), (2, 3), (3, 4), (4, 5),
            (5, 6), (6, 7), (7, 8), (8, 9)]
G = nx.Graph()
G.add_nodes_from([x for x, y in theGraph] + [y for x, y in theGraph])
G.add_edges_from(theGraph)
colorMap = []
for node in G.nodes():
        if node in (0,1,2,3,4):
                colorMap.append('red')
        else:
                colorMap.append('cyan')
pos = nx.spring_layout(G, iterations=100)
nx.draw(G, pos=pos, with_labels=True, node_size=475,
        width=2, edge_color="black", node_color=colorMap)
ax2.set_title("10q")

ax3 = fig.add_subplot(233)
theGraph = [(0, 1), (1, 2), (3, 4), (4, 5), (5, 6), (7, 8), (9, 10),
            (10, 11), (0, 4), (1, 5), (2, 6), (4, 7), (5, 8), (7, 10), (8, 11)]
G = nx.Graph()
G.add_nodes_from([x for x, y in theGraph] + [y for x, y in theGraph])
G.add_edges_from(theGraph)
colorMap = []
for node in G.nodes():
        if node in (0,1,2,3,4,7):
                colorMap.append('red')
        else:
                colorMap.append('cyan')
pos = nx.spring_layout(G, iterations=100)
nx.draw(G, pos=pos, with_labels=True, node_size=475,
        width=2, edge_color="black", node_color=colorMap)
ax3.set_title("12q")

ax4 = fig.add_subplot(234)
theGraph = [(2, 6), (3, 7), (7, 11), (11, 15), (0, 4), (4, 8), (8, 12), (12, 16), (1, 5), (5, 9), (9, 13), (10, 14),
            (2, 3), (6, 7), (3, 4), (7, 8), (11, 12), (15, 16), (0, 1), (4, 5), (8, 9), (12, 13), (9, 10), (13, 14)]
G = nx.Graph()
G.add_nodes_from([x for x, y in theGraph] + [y for x, y in theGraph])
G.add_edges_from(theGraph)
colorMap = []
for node in G.nodes():
        if node in (0,1,4,5,8,9):
                colorMap.append('red')
        elif node in (2,3,6,7,11,15):
                colorMap.append('cyan')
        else:
                colorMap.append('green')
pos = nx.spring_layout(G, iterations=300)
nx.draw(G, pos=pos, with_labels=True, node_size=475,
        width=2, edge_color="black", node_color=colorMap)
ax4.set_title("17q")

ax5 = fig.add_subplot(235)
theGraph = [(22,23),(23,24),(0,1),(1,2),(2,3),(4,5),(5,6),(6,7),(7,8),(8,9),(10,11),(11,12),(12,13),(14,15),(15,16),(16,17),(18,19),(19,20),(20,21),(22,1),(23,2),(24,3),(0,5),(1,6),(2,7),(3,8),(5,10),(6,11),(7,12),(8,13),(10,15),(11,16),(12,17),(16,18),(17,19)]
G = nx.Graph()
G.add_nodes_from([x for x, y in theGraph] + [y for x, y in theGraph])
G.add_edges_from(theGraph)
colorMap = []
for node in G.nodes():
        if node in (0,1,4,5,6,10,22):
                colorMap.append('red')
        elif node in (2,3,8,9,23,24):
                colorMap.append('cyan')
        elif node in (11,14,15,16,18):
                colorMap.append('gold')
        else:
                colorMap.append('green')
pos = nx.spring_layout(G, iterations=400)
nx.draw(G, pos=pos, with_labels=True, node_size=475,
        width=2, edge_color="black", node_color=colorMap)
ax5.set_title("25q")

ax6 = fig.add_subplot(236)
theGraph = [(0,1),(1,2),(2,3),(3,4),(5,6),(6,7),(7,8),(8,9),(10,11),(11,12),(12,13),(14,15),(15,16),(16,17),(18,19),(19,20),(20,21),(21,22),(22,23),(24,25),(25,26),(26,27),(28,29),(0,6),(1,7),(2,8),(3,9),(7,10),(8,11),(9,12),(10,15),(11,16),(12,17),(14,19),(15,20),(16,21),(17,22),(18,24),(19,25),(20,26),(21,27),(27,28)]
G = nx.Graph()
G.add_nodes_from([x for x, y in theGraph] + [y for x, y in theGraph])
G.add_edges_from(theGraph)
colorMap = []
for node in G.nodes():
        if node in (0,1,2,3,4):
                colorMap.append('red')
        elif node in (5,6,7,8,9,10):
                colorMap.append('cyan')
        elif node in (11,12,13,14,15,16):
                colorMap.append('gold')
        elif node in (17,19,20,21,22,23):
                colorMap.append('magenta')
        else:
                colorMap.append('green')
pos = nx.spring_layout(G, iterations=600)
nx.draw(G, pos=pos, with_labels=True, node_size=475,
        width=2, edge_color="black", node_color=colorMap)
ax6.set_title("30q")

plt.show()
