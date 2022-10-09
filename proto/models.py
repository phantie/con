from __future__ import annotations
# __all__ = ['Node', 'Con']

from platform import node
import pydantic
from pprint import pp
from typing import Tuple, Any, Union, Literal
from ruption import Option, some, none, Panic
from itertools import chain
from abc import ABC, abstractmethod


# class IntoNode(ABC):
#     @property
#     @abstractmethod
#     def node(self) -> Node: ...

class Node(pydantic.BaseModel):
    id: Any

    def __init__(self, id, /):
        return super().__init__(id = id)

    @property
    def node(self):
        return self

class City(pydantic.BaseModel):
    bank: int
    node: Node

    def __init__(self, node, /, *, bank):
        return super().__init__(node = node, bank = bank)

    def set_bank(self, bank):
        return self.__class__(self.node, bank = bank)

    def inc_bank(self, inc):
        return self.set_bank(self.bank + inc)

    def dec_bank(self, dec):
        return self.set_bank(self.bank - dec)

    def pretty(self):
        return f"City({self.node.id}, bank={self.bank})"


class Con(pydantic.BaseModel):
    targets: Tuple[Any, Any]

    def __init__(self, targets):
        return super().__init__(targets = targets)

    @property
    def con(self):
        return self

class Route(pydantic.BaseModel):
    con: Con
    send: int

    def __init__(self, con, /, *, send):
        return super().__init__(con = con, send = send)

    def pretty(self):
        return \
        f"Route: {self.con.targets[0].pretty()}" \
        f"-> {self.send} -> {self.con.targets[1].pretty()}"

def cons_into_edge_set(cons):
    return [(con.con.targets[0].node.id, con.con.targets[1].node.id) for con in cons]

def edge_set_into_node_set(edge_set):
    return set(chain.from_iterable(edge_set))

def adj_map_into_node_set(adj_map):
    return set(adj_map)

def edge_set_into_adj_map(edge_set):
    node_set = edge_set_into_node_set(edge_set)
    return {
        node: [
            n1 if n2 == node else n2 for (n1, n2) in edge_set 
            if n1 == node or n2 == node]
    for node in node_set}


def cons_into_adj_map(cons):
    return edge_set_into_adj_map(cons_into_edge_set(cons))



