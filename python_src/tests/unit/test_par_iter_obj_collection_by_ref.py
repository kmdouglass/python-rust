from python_lib import StateMachine, Transition, par_run


def test_par_run():
    num_machines = 10
    machines = [StateMachine() for _ in range(num_machines)]

    transitions = par_run(machines)

    assert num_machines == len(transitions)
