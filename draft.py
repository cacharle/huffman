#!/usr/bin/python3

import sys
import heapq
import collections
import copy


class BitRepr:
    def __init__(self, x: int, length: int):
        self.x = x
        self.length = length

    def add_zero(self):
        self.x = self.x << 1
        self.length += 1

    def add_one(self):
        self.x = self.x << 1
        self.length += 1
        self.x = self.x | 1

    def __repr__(self):
        return "{0:b}".format(self.x).zfill(self.length)


class HuffmanNode:
    def __init__(
        self, occurence: int, is_leaf: bool,
        symbol: int = None, left: 'HuffmanNode' = None, right: 'HuffmanNode' = None
    ):
        self.occurence = occurence
        self.is_leaf = is_leaf
        if is_leaf:
            self.symbol = symbol
        else:
            self.left = left
            self.right = right

    @staticmethod
    def leaf(occurence: int, symbol: int):
        return HuffmanNode(occurence, is_leaf=True, symbol=symbol)

    @staticmethod
    def node(left: 'HuffmanNode', right: 'HuffmanNode'):
        return HuffmanNode(left.occurence + right.occurence, is_leaf=False, left=left, right=right)

    def __gt__(self, other: 'HuffmanNode') -> bool:
        return self.occurence > other.occurence

    def __lt__(self, other: 'HuffmanNode') -> bool:
        return self.occurence < other.occurence

    def __repr__(self) -> str:
        if self.is_leaf:
            return f"({self.symbol} {self.occurence})"
        else:
            return f"(NODE {self.occurence})"  # left {str(self.left)} right {str(self.right)})"

    def put_tree(self, level: int = 0):
        if self.is_leaf:
            print(f"({self.symbol} {self.occurence})")
            return
        print(f"NODE {self.occurence}")
        print(" " * level * 4, "left ", end="")
        self.left.put_tree(level + 1)
        print(" " * level * 4, "right ", end="")
        self.right.put_tree(level + 1)

    def to_bit_repr(self, parent_bits: 'BitRepr' = BitRepr(0, 0)) -> {int: 'BitRepr'}:
        if self.is_leaf:
            return {self.symbol: parent_bits}
        left_bits = copy.deepcopy(parent_bits)
        left_bits.add_zero()
        right_bits = copy.deepcopy(parent_bits)
        right_bits.add_one()
        return {
            **self.left.to_bit_repr(left_bits),
            **self.right.to_bit_repr(right_bits)
        }

if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("missing file name")
        sys.exit(1)

    file_name = sys.argv[1]
    # print(file_name)

    content = None
    with open(file_name, "rb") as f:
        content = f.read()
    counter = collections.Counter(content)

    huffman_nodes = [HuffmanNode.leaf(v, k) for k, v in dict(counter).items()]
    heapq.heapify(huffman_nodes)

    while len(huffman_nodes) >= 2:
        # print(huffman_nodes)
        left = heapq.heappop(huffman_nodes)
        right = heapq.heappop(huffman_nodes)
        node = HuffmanNode.node(left, right)
        heapq.heappush(huffman_nodes, node)
    # print(huffman_nodes)

    huffman_tree = huffman_nodes[0]
    huffman_tree.put_tree()
    bit_repr = huffman_tree.to_bit_repr()
    print(bit_repr)

    huffman_content = ''.join([str(bit_repr[s]) for s in content])
    print(huffman_content)
