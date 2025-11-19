import math

import py_pi
import pytest

N = 10_000_000


def test_single():
    assert math.isclose(py_pi.single(N), math.pi, abs_tol=1e-2)


def test_parallel():
    assert math.isclose(py_pi.parallel(N), math.pi, abs_tol=1e-2)
