import json
from typing import Tuple, Set, List


class KnowledgeBase:
    def __init__(self, clauses: List[Set[Tuple[int, bool]]]):
        self.clauses = clauses

    @staticmethod
    def from_json(path):
        with open(path, 'r') as f:
            data = json.load(f)
        data = [set((int(i), bool(b)) for i, b in c) for c in data]
        return KnowledgeBase(data)

    def get_vars(self) -> Set[int]:
        return set(v for c in self.clauses for (v, b) in c)

    def __repr__(self):
        n_vars = len(self.get_vars())
        return f'KnowledgeBase(len={len(self.clauses)} vars={n_vars})'
