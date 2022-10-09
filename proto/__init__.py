
from models import *
from ruption import Option, some, none, Panic
from functools import partial

def derive_neighbours(adj_map, node):
    return adj_map[node]

def dfs(adj_map, node, postorder = False):
    visited = []
    def _dfs(adj_map, node, marked = None):
        def visit(node):
            nonlocal visited
            visited.append(node)

        marked = marked or {node: False for node in adj_map_into_node_set(adj_map)}
        if not postorder:
            visit(node)
        marked[node] = True
        for w in derive_neighbours(adj_map, node):
            if not marked[w]:
                _dfs(adj_map, w, marked)
        if postorder:
            visit(node)
        return visited
    return _dfs(adj_map, node)



# nodes = [
#     13, # 0
#     42, # 1
#     7,  # 2
#     69, # 3
# ]

# 13 - 42 - 7
# |
# 69

nodes = {
    13: City(Node(13), bank = 0),
    42: City(Node(42), bank = 0),
    7: City(Node(7), bank = 0),
    69: City(Node(69), bank = 0),
}

def node_pair(nodes, first, second):
    return (nodes[first], nodes[second])

node_pair = partial(node_pair, nodes)

# 13 - 42 - 7
# |
# 69

cons = [
    Route(Con(node_pair(13, 42)), send = 10),
    Route(Con(node_pair(42, 7)), send = 5),
    Route(Con(node_pair(13, 69)), send = 10),
]

pp(cons)
pp(cons_into_edge_set(cons))
pp(cons_into_adj_map(cons))

adj_map = cons_into_adj_map(cons)
assert dfs(adj_map, 13) == [13, 42, 7, 69]
assert dfs(adj_map, 13, postorder = True) == [7, 42, 69, 13]


def step(cons):
    node_id_to_city = {}
    for route in cons:
        con = route.con
        send_amount = route.send
        city_send, city_receive = con.targets
        city_send_node = city_send.node
        city_receive_node = city_receive.node

        node_id_to_city.setdefault(city_send_node.id, city_send)
        node_id_to_city.setdefault(city_receive_node.id, city_receive)

        node_id_to_city[city_send_node.id] = \
            node_id_to_city[city_send_node.id].dec_bank(send_amount)

        node_id_to_city[city_receive_node.id] = \
            node_id_to_city[city_receive_node.id].inc_bank(send_amount)

    # pp(node_id_to_city)

    return [
        Route(Con((
            node_id_to_city[route.con.targets[0].node.id],
            node_id_to_city[route.con.targets[1].node.id],
        )), send = route.send)
        for route in cons
    ]



def steps(cons, n):
    for _ in range(n):
        cons = step(cons)
        yield cons

for _cons in (cons, *steps(cons, 2)):
    # display_cons(_cons)
    for route in _cons:
        print(route.pretty())

    print()