import pytest
import rastro

def test_rclass():
    rclass = rastro.RClass(3, True)

    assert rastro.count_class(rclass) == 4

    print(rclass)
    assert rastro.access_internal(rclass) == 1.0

    rastro.add_internal(rclass)
    rastro.add_internal(rclass)

    assert rclass.count == 3
    assert rclass.truth == True

    print(rclass)

    assert rastro.access_internal(rclass) == 3.0