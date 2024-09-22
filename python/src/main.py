from grad_solver import solve
from knowledge_base import KnowledgeBase


def main():
    knowledge_base = KnowledgeBase.from_json('../data/knowledge_base.json')
    # print(knowledge_base.get_vars())
    # for clause in knowledge_base.clauses:
    #     print(clause)
    solve(knowledge_base)


if __name__ == '__main__':
    main()

