"""Stubs for Rust functions."""

from dataclasses import dataclass
from typing import Protocol

class StateMachine(Protocol): ...

@dataclass(frozen=True)
class Transition(Protocol): ...

def par_run(machines: list[StateMachine]) -> list[Transition]: ...
