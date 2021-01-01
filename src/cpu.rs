// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2018-2021 by the author(s)
//
// Author(s):
//   - Tock Project Developers <tock-dev@googlegroups.com>
//   - Andre Richter <andre.o.richter@gmail.com>

//! CPU traits.

use tock_registers::registers::{
    Field, FieldValue, IntLike, LocalRegisterCopy, RegisterLongName, TryFromValue,
};

/// Trait for register R/W functions.
pub trait RegisterReadWrite<T: IntLike, R: RegisterLongName> {
    /// Implement this as #[inline].
    fn get(&self) -> T;

    /// Implement this as #[inline].
    fn set(&self, value: T);

    #[inline]
    fn read(&self, field: Field<T, R>) -> T {
        field.read(self.get())
    }

    #[inline]
    fn read_as_enum<E: TryFromValue<T, EnumType = E>>(&self, field: Field<T, R>) -> Option<E> {
        field.read_as_enum(self.get())
    }

    #[inline]
    fn extract(&self) -> LocalRegisterCopy<T, R> {
        LocalRegisterCopy::new(self.get())
    }

    #[inline]
    fn write(&self, field: FieldValue<T, R>) {
        self.set(field.value);
    }

    #[inline]
    fn modify(&self, field: FieldValue<T, R>) {
        self.set(field.modify(self.get()));
    }

    #[inline]
    fn modify_no_read(&self, original: LocalRegisterCopy<T, R>, field: FieldValue<T, R>) {
        self.set(field.modify(original.get()));
    }

    #[inline]
    fn is_set(&self, field: Field<T, R>) -> bool {
        field.is_set(self.get())
    }

    #[inline]
    fn matches_any(&self, field: FieldValue<T, R>) -> bool {
        field.matches_any(self.get())
    }

    #[inline]
    fn matches_all(&self, field: FieldValue<T, R>) -> bool {
        field.matches_all(self.get())
    }
}

/// Trait for register RO functions.
pub trait RegisterReadOnly<T: IntLike, R: RegisterLongName> {
    /// Implement this as #[inline].
    fn get(&self) -> T;

    #[inline]
    fn read(&self, field: Field<T, R>) -> T {
        field.read(self.get())
    }

    #[inline]
    fn read_as_enum<E: TryFromValue<T, EnumType = E>>(&self, field: Field<T, R>) -> Option<E> {
        field.read_as_enum(self.get())
    }

    #[inline]
    fn extract(&self) -> LocalRegisterCopy<T, R> {
        LocalRegisterCopy::new(self.get())
    }

    #[inline]
    fn is_set(&self, field: Field<T, R>) -> bool {
        self.read(field) != T::zero()
    }

    #[inline]
    fn matches_any(&self, field: FieldValue<T, R>) -> bool {
        field.matches_any(self.get())
    }

    #[inline]
    fn matches_all(&self, field: FieldValue<T, R>) -> bool {
        field.matches_all(self.get())
    }
}

/// Trait for register WO functions.
pub trait RegisterWriteOnly<T: IntLike, R: RegisterLongName> {
    /// Implement this as #[inline].
    fn set(&self, value: T);

    #[inline]
    fn write(&self, field: FieldValue<T, R>) {
        self.set(field.value);
    }
}
