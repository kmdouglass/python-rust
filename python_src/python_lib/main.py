"""Call a Rust function from Python"""

from .python_lib import meaning_of_life


def what_is_the_meaning_of_life() -> int:
    """Example of how to call a Rust function from Python"""
    meaning: int = meaning_of_life()
    return meaning
