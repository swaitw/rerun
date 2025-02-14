# DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/store/re_types/definitions/rerun/blueprint/components/force_strength.fbs".

# You can extend this class by creating a "ForceStrengthExt" class in "force_strength_ext.py".

from __future__ import annotations

from ... import datatypes
from ..._baseclasses import (
    ComponentBatchMixin,
    ComponentDescriptor,
    ComponentMixin,
)

__all__ = ["ForceStrength", "ForceStrengthBatch"]


class ForceStrength(datatypes.Float64, ComponentMixin):
    """
    **Component**: The strength of a given force.

    Allows to assign different weights to the individual forces, prioritizing one over the other.
    """

    _BATCH_TYPE = None
    # You can define your own __init__ function as a member of ForceStrengthExt in force_strength_ext.py

    # Note: there are no fields here because ForceStrength delegates to datatypes.Float64
    pass


class ForceStrengthBatch(datatypes.Float64Batch, ComponentBatchMixin):
    _COMPONENT_DESCRIPTOR: ComponentDescriptor = ComponentDescriptor("rerun.blueprint.components.ForceStrength")


# This is patched in late to avoid circular dependencies.
ForceStrength._BATCH_TYPE = ForceStrengthBatch  # type: ignore[assignment]
