//! Methods to enforce constraints on booleans in a resolved Leo program.

use crate::{
    constraints::{new_variable_from_variable, ConstrainedProgram, ConstrainedValue},
    types::{ParameterModel, ParameterValue, Variable},
};

use snarkos_errors::gadgets::SynthesisError;
use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::{
        r1cs::ConstraintSystem,
        utilities::{alloc::AllocGadget, boolean::Boolean, eq::EqGadget},
    },
};

impl<F: Field + PrimeField, CS: ConstraintSystem<F>> ConstrainedProgram<F, CS> {
    pub(crate) fn bool_from_parameter(
        &mut self,
        cs: &mut CS,
        scope: String,
        parameter_model: ParameterModel<F>,
        parameter_value: Option<ParameterValue<F>>,
    ) -> Variable<F> {
        // Check that the parameter value is the correct type
        let bool_value = parameter_value.map(|parameter| match parameter {
            ParameterValue::Boolean(b) => b,
            value => unimplemented!("expected boolean parameter, got {}", value),
        });

        // Check visibility of parameter
        let name = parameter_model.variable.name.clone();
        let number = if parameter_model.private {
            Boolean::alloc(cs.ns(|| name), || {
                bool_value.ok_or(SynthesisError::AssignmentMissing)
            })
            .unwrap()
        } else {
            Boolean::alloc_input(cs.ns(|| name), || {
                bool_value.ok_or(SynthesisError::AssignmentMissing)
            })
            .unwrap()
        };

        let parameter_variable = new_variable_from_variable(scope, &parameter_model.variable);

        // store each argument as variable in resolved program
        self.store_variable(
            parameter_variable.clone(),
            ConstrainedValue::Boolean(number),
        );

        parameter_variable
    }

    pub(crate) fn boolean_array_from_parameter(
        &mut self,
        _cs: &mut CS,
        _scope: String,
        _parameter_model: ParameterModel<F>,
        _parameter_value: Option<ParameterValue<F>>,
    ) -> Variable<F> {
        unimplemented!("Cannot enforce boolean array as parameter")
        // // Check visibility of parameter
        // let mut array_value = vec![];
        // let name = parameter.variable.name.clone();
        // for argument in argument_array {
        //     let number = if parameter.private {
        //         Boolean::alloc(cs.ns(|| name), ||bool_value.ok_or(SynthesisError::AssignmentMissing).unwrap()
        //     } else {
        //         Boolean::alloc_input(cs.ns(|| name), || Ok(argument)).unwrap()
        //     };
        //
        //     array_value.push(number);
        // }
        //
        //
        // let parameter_variable = new_variable_from_variable(scope, &parameter.variable);
        //
        // // store array as variable in resolved program
        // self.store_variable(parameter_variable.clone(), ResolvedValue::BooleanArray(array_value));
        //
        // parameter_variable
    }

    pub(crate) fn get_boolean_constant(bool: bool) -> ConstrainedValue<F> {
        ConstrainedValue::Boolean(Boolean::Constant(bool))
    }

    pub(crate) fn evaluate_not(value: ConstrainedValue<F>) -> ConstrainedValue<F> {
        match value {
            ConstrainedValue::Boolean(boolean) => ConstrainedValue::Boolean(boolean.not()),
            value => unimplemented!("cannot enforce not on non-boolean value {}", value),
        }
    }

    pub(crate) fn enforce_or(
        &mut self,
        cs: &mut CS,
        left: ConstrainedValue<F>,
        right: ConstrainedValue<F>,
    ) -> ConstrainedValue<F> {
        match (left, right) {
            (ConstrainedValue::Boolean(left_bool), ConstrainedValue::Boolean(right_bool)) => {
                ConstrainedValue::Boolean(Boolean::or(cs, &left_bool, &right_bool).unwrap())
            }
            (left_value, right_value) => unimplemented!(
                "cannot enforce or on non-boolean values {} || {}",
                left_value,
                right_value
            ),
        }
    }

    pub(crate) fn enforce_and(
        &mut self,
        cs: &mut CS,
        left: ConstrainedValue<F>,
        right: ConstrainedValue<F>,
    ) -> ConstrainedValue<F> {
        match (left, right) {
            (ConstrainedValue::Boolean(left_bool), ConstrainedValue::Boolean(right_bool)) => {
                ConstrainedValue::Boolean(Boolean::and(cs, &left_bool, &right_bool).unwrap())
            }
            (left_value, right_value) => unimplemented!(
                "cannot enforce and on non-boolean values {} && {}",
                left_value,
                right_value
            ),
        }
    }

    pub(crate) fn boolean_eq(left: Boolean, right: Boolean) -> ConstrainedValue<F> {
        ConstrainedValue::Boolean(Boolean::Constant(left.eq(&right)))
    }

    pub(crate) fn enforce_boolean_eq(&mut self, cs: &mut CS, left: Boolean, right: Boolean) {
        left.enforce_equal(cs.ns(|| format!("enforce bool equal")), &right)
            .unwrap();
    }
}
