from knowledge_base import KnowledgeBase


def main():
    knowledge_base = KnowledgeBase.from_json('../data/knowledge_base.json')
    print(knowledge_base)
    print(knowledge_base.get_vars())


if __name__ == '__main__':
    main()

