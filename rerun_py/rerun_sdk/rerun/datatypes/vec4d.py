# DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/store/re_types/definitions/rerun/datatypes/vec4d.fbs".

# You can extend this class by creating a "Vec4DExt" class in "vec4d_ext.py".

from __future__ import annotations

from typing import TYPE_CHECKING, Any, Sequence, Union

import numpy as np
import numpy.typing as npt
import pyarrow as pa
from attrs import define, field

from .._baseclasses import (
    BaseBatch,
)
from .._converters import (
    to_np_float32,
)
from .vec4d_ext import Vec4DExt

__all__ = ["Vec4D", "Vec4DArrayLike", "Vec4DBatch", "Vec4DLike"]


@define(init=False)
class Vec4D(Vec4DExt):
    """**Datatype**: A vector in 4D space."""

    def __init__(self: Any, xyzw: Vec4DLike):
        """Create a new instance of the Vec4D datatype."""

        # You can define your own __init__ function as a member of Vec4DExt in vec4d_ext.py
        self.__attrs_init__(xyzw=xyzw)

    xyzw: npt.NDArray[np.float32] = field(converter=to_np_float32)

    def __array__(self, dtype: npt.DTypeLike = None) -> npt.NDArray[Any]:
        # You can define your own __array__ function as a member of Vec4DExt in vec4d_ext.py
        return np.asarray(self.xyzw, dtype=dtype)


if TYPE_CHECKING:
    Vec4DLike = Union[Vec4D, npt.NDArray[Any], npt.ArrayLike, Sequence[float]]
else:
    Vec4DLike = Any

Vec4DArrayLike = Union[
    Vec4D, Sequence[Vec4DLike], npt.NDArray[Any], npt.ArrayLike, Sequence[Sequence[float]], Sequence[float]
]


class Vec4DBatch(BaseBatch[Vec4DArrayLike]):
    _ARROW_DATATYPE = pa.list_(pa.field("item", pa.float32(), nullable=False, metadata={}), 4)

    @staticmethod
    def _native_to_pa_array(data: Vec4DArrayLike, data_type: pa.DataType) -> pa.Array:
        return Vec4DExt.native_to_pa_array_override(data, data_type)
