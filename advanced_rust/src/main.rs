use crate::identical_methods::animals;

mod deref_rawpointer;
mod safe_abstraction;
mod call_extern_code;
mod access_or_mut_static;
mod associated_type;
mod default_gen_params_op_overloading;
mod identical_methods;
mod super_trait;
mod advance_type;
mod adv_fn_closures;

fn main() {
    
    // deref_rawpointer::deref_raw();
    // safe_abstraction::safe_abs();
    // call_extern_code::extern_code();
    // access_or_mut_static::counter();
    // associated_type::run_counter();
//  default_gen_params_op_overloading::ops();
//   default_gen_params_op_overloading::nongeneric_rhs();
  // identical_methods::specific();
  // animals::specific_animal();
  // super_trait::printer();
  // advance_type::synonyms_alias();
  adv_fn_closures::advanced_fn();
 

}
