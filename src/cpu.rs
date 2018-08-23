/*
 * Copyright (c) 2018 by the author(s)
 *
 * =============================================================================
 *
 * Licensed under either of
 *   - Apache License, Version 2.0 (http://www.apache.org/licenses/LICENSE-2.0)
 *   - MIT License (http://opensource.org/licenses/MIT)
 * at your option.
 *
 * =============================================================================
 *
 * Author(s):
 *   - Tock Project Developers <tock-dev@googlegroups.com>
 *   - Andre Richter <andre.o.richter@gmail.com>
 */

use tock_registers::registers::{Field, FieldValue, IntLike, LocalRegisterCopy, RegisterLongName, TryFromValue};

/// Trait for register R/W functions
pub trait RegisterReadWrite<T: IntLike, R: RegisterLongName> {
    /// Implement this as #[inline]
    fn get(&self) -> T;

    /// Implement this as #[inline]
    fn set(&self, value: T);

    #[inline]
    fn read(&self, field: Field<T, R>) -> T {
        (self.get() & (field.mask << field.shift)) >> field.shift
    }

    #[inline]
    fn read_as_enum<E: TryFromValue<T, EnumType = E>>(&self, field: Field<T, R>) -> Option<E> {
        let val: T = self.read(field);

        E::try_from(val)
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
        let reg: T = self.get();
        self.set((reg & !field.mask) | field.value);
    }

    #[inline]
    fn modify_no_read(&self, original: LocalRegisterCopy<T, R>, field: FieldValue<T, R>) {
        self.set((original.get() & !field.mask) | field.value);
    }

    #[inline]
    fn is_set(&self, field: Field<T, R>) -> bool {
        self.read(field) != T::zero()
    }

    #[inline]
    fn matches_any(&self, field: FieldValue<T, R>) -> bool {
        self.get() & field.mask != T::zero()
    }

    #[inline]
    fn matches_all(&self, field: FieldValue<T, R>) -> bool {
        self.get() & field.mask == field.value
    }
}

/// Trait for register RO functions
pub trait RegisterReadOnly<T: IntLike, R: RegisterLongName> {
    /// Implement this as #[inline]
    fn get(&self) -> T;

    #[inline]
    fn read(&self, field: Field<T, R>) -> T {
        (self.get() & (field.mask << field.shift)) >> field.shift
    }

    #[inline]
    fn read_as_enum<E: TryFromValue<T, EnumType = E>>(&self, field: Field<T, R>) -> Option<E> {
        let val: T = self.read(field);

        E::try_from(val)
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
        self.get() & field.mask != T::zero()
    }

    #[inline]
    fn matches_all(&self, field: FieldValue<T, R>) -> bool {
        self.get() & field.mask == field.value
    }
}

/// Trait for register WO functions
pub trait RegisterWriteOnly<T: IntLike, R: RegisterLongName> {
    /// Implement this as #[inline]
    fn set(&self, value: T);

    #[inline]
    fn write(&self, field: FieldValue<T, R>) {
        self.set(field.value);
    }
}
