import json
from typing import Tuple, Set, List, Dict


class KnowledgeBase:
    def __init__(self, clauses: List[Set[Tuple[int, bool]]], mapping: Dict[int, str]):
        self.clauses = clauses
        self.mapping = mapping

    @staticmethod
    def from_json(path):
        with open(path, 'r') as f:
            data = json.load(f)
        clauses = [set((int(i), bool(b)) for i, b in c) for c in data['knowledge_base']]
        mapping = {index: name for name, index in data['mapping']}
        kb = KnowledgeBase(clauses, mapping)
        return kb

    def get_vars(self) -> Set[int]:
        return set(v for c in self.clauses for (v, b) in c)

    def reindex(self) -> Dict[int, int]:
        old_vars = self.get_vars()
        var_translation = {old_var: new_var for new_var, old_var in enumerate(sorted(old_vars))}

        new_clauses = []
        for old_clause in self.clauses:
            new_clause = set((var_translation[v], b) for (v, b) in old_clause)
            new_clauses.append(new_clause)
        self.clauses = new_clauses

        new_mapping = {}
        for old_var, old_name in self.mapping.items():
            if old_var in var_translation:
                new_mapping[var_translation[old_var]] = old_name
        self.mapping = new_mapping

        return var_translation

    def validate(self):
        variables = self.get_vars()
        assert variables == set(range(len(variables))), 'invalid variables (not contiguous starting at 0)'

    def __repr__(self):
        n_vars = len(self.get_vars())
        return f'KnowledgeBase(len={len(self.clauses)} vars={n_vars})'
