from typing import Tuple, List
import time

import numpy as np

from knowledge_base import KnowledgeBase
import torch

from sudoku import get_hard_solution


def solve(knowledge_base: KnowledgeBase):
    knowledge_base.reindex()

    # device = torch.device('cuda' if torch.cuda.is_available() else 'cpu')
    device = torch.device('cpu')
    torch_variables, variables = create_variables(knowledge_base, device)
    start_signs = torch.sign(torch_variables - 0.5)

    all_var_indices, all_positive = prepare_clause_sums(knowledge_base, device)

    learning_rate = 5.0
    alpha = 0.2

    optimizer = torch.optim.SGD([torch_variables], lr=learning_rate)

    for epoch in range(20):
        optimizer.zero_grad()
        torch_clause_sums = create_clause_sums(all_var_indices, all_positive, torch_variables)
        satisfied_loss = get_satisfied_loss(torch_clause_sums) * 25
        decided_loss = get_decided_loss(torch_variables)
        loss = satisfied_loss + alpha * decided_loss
        loss.backward()

        accuracy = calc_accuracy(torch_variables.detach().cpu().numpy(), knowledge_base, variables)

        print(f'epoch={epoch:<4}  alpha={alpha:.3f}  Ls={satisfied_loss:.5f}  Ld={decided_loss:.5f}  L={loss:.4f}  '
              f'acc={accuracy:.4f}')

        optimizer.step()

        # print(torch_variables)

        # alpha += 0.01
        # if epoch % 100 == 0 and epoch > 0:
        #     torch_variables = restart_variables(torch_variables)
        #     optimizer = torch.optim.SGD([torch_variables], lr=learning_rate)
        #     alpha = 0.01

    end_signs = torch.sign(torch_variables - 0.5)

    result = torch_variables.detach().cpu().numpy()
    lowest = np.argsort(result)
    sorted_variables = variables[lowest]
    sorted_probabilities = result[lowest]
    sudoku = np.zeros((9, 9), dtype=np.int32)
    real_sudoku = get_hard_solution()
    for variable, probability in zip(sorted_variables, sorted_probabilities):
        var_name = knowledge_base.mapping[variable]
        x, y, value = int(var_name[1]), int(var_name[2]), int(var_name[3])
        correct = int(real_sudoku[y - 1, x - 1] == value)
        print(f'{variable:<4} {probability:.3f} {var_name}  correct={correct}')
        if probability > 0.5:
            sudoku[y-1, x-1] = value
    print(sudoku)
    print('\nsame')
    print(torch.isclose(start_signs, end_signs).type(torch.int32))
    accuracy = calc_accuracy(result, knowledge_base, variables)
    print('accuracy:', accuracy)


def calc_accuracy(result: np.ndarray, knowledge_base: KnowledgeBase, variables: np.ndarray):
    lowest = np.argsort(result)
    sorted_variables = variables[lowest]
    sorted_probabilities = result[lowest]
    sudoku = np.zeros((9, 9), dtype=np.int32)
    for variable, probability in zip(sorted_variables, sorted_probabilities):
        var_name = knowledge_base.mapping[variable]
        if probability > 0.5:
            x, y, value = int(var_name[1]), int(var_name[2]), int(var_name[3])
            sudoku[y-1, x-1] = value

    real_sudoku = get_hard_solution()

    return np.sum(np.equal(sudoku, real_sudoku)) / np.sum(sudoku != 0)


def restart_variables(variables):
    new_variables = torch.sign(variables - 0.5) * 0.001 + 0.5
    new_variables.detach_()
    new_variables.requires_grad_(True)
    return new_variables


def create_variables(knowledge_base: KnowledgeBase, device: torch.device) -> Tuple[torch.Tensor, np.ndarray]:
    variables = np.array(list(knowledge_base.get_vars()))
    torch_variables = (torch.rand(len(variables), device=device) - 0.5) * 0.001 + 0.5
    # torch_variables = torch.full((len(variables),), 0.499, dtype=torch.float32, device=device, requires_grad=True)

    torch_variables.detach_()
    torch_variables.requires_grad_(True)

    assert torch_variables.is_leaf and torch_variables.requires_grad

    return torch_variables, variables


def prepare_clause_sums(knowledge_base: KnowledgeBase, device: torch.device):
    all_var_indices = []
    all_positive = []
    for clause in knowledge_base.clauses:
        var_indices = torch.tensor([v for v, _ in clause], dtype=torch.int32, device=device)
        all_var_indices.append(var_indices)
        positive = torch.tensor([1 - int(p) for _, p in clause], dtype=torch.float32, device=device)
        all_positive.append(positive)
    return all_var_indices, all_positive


def create_clause_sums(all_var_indices, all_positive, torch_variables: torch.Tensor) -> torch.Tensor:
    torch_sums = []
    # start_time = time.time()
    for var_indices, positive in zip(all_var_indices, all_positive):
        torch_sum = torch.sum(positive - torch_variables[var_indices] * (2*positive - 1))
        torch_sums.append(torch_sum)
    # end_time = time.time()
    # print('create_clause_sums:', end_time - start_time)

    return torch.stack(torch_sums)


def get_satisfied_loss(clause_sums: torch.Tensor) -> torch.Tensor:
    return torch.mean(torch.square(torch.clip(clause_sums, 0.0, 1.0) - 1.0))


def get_decided_loss(variables) -> torch.Tensor:
    return torch.mean(torch.pow(0.5 - torch.abs(variables - 0.5), 3.0))
