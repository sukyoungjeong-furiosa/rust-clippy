// This file was generated by `cargo dev update_lints`.
// Use that command to update this file and do not edit by hand.
// Manual edits will be overwritten.

store.register_group(true, "clippy::complexity", Some("clippy_complexity"), vec![
    LintId::of(attrs::DEPRECATED_CFG_ATTR),
    LintId::of(booleans::NONMINIMAL_BOOL),
    LintId::of(bytes_count_to_len::BYTES_COUNT_TO_LEN),
    LintId::of(casts::CHAR_LIT_AS_U8),
    LintId::of(casts::UNNECESSARY_CAST),
    LintId::of(derivable_impls::DERIVABLE_IMPLS),
    LintId::of(double_comparison::DOUBLE_COMPARISONS),
    LintId::of(double_parens::DOUBLE_PARENS),
    LintId::of(duration_subsec::DURATION_SUBSEC),
    LintId::of(explicit_write::EXPLICIT_WRITE),
    LintId::of(format::USELESS_FORMAT),
    LintId::of(functions::TOO_MANY_ARGUMENTS),
    LintId::of(identity_op::IDENTITY_OP),
    LintId::of(int_plus_one::INT_PLUS_ONE),
    LintId::of(lifetimes::EXTRA_UNUSED_LIFETIMES),
    LintId::of(lifetimes::NEEDLESS_LIFETIMES),
    LintId::of(loops::EXPLICIT_COUNTER_LOOP),
    LintId::of(loops::MANUAL_FLATTEN),
    LintId::of(loops::SINGLE_ELEMENT_LOOP),
    LintId::of(loops::WHILE_LET_LOOP),
    LintId::of(manual_strip::MANUAL_STRIP),
    LintId::of(manual_unwrap_or::MANUAL_UNWRAP_OR),
    LintId::of(map_unit_fn::OPTION_MAP_UNIT_FN),
    LintId::of(map_unit_fn::RESULT_MAP_UNIT_FN),
    LintId::of(matches::MATCH_AS_REF),
    LintId::of(matches::MATCH_SINGLE_BINDING),
    LintId::of(matches::NEEDLESS_MATCH),
    LintId::of(matches::WILDCARD_IN_OR_PATTERNS),
    LintId::of(methods::BIND_INSTEAD_OF_MAP),
    LintId::of(methods::CLONE_ON_COPY),
    LintId::of(methods::FILTER_MAP_IDENTITY),
    LintId::of(methods::FILTER_NEXT),
    LintId::of(methods::FLAT_MAP_IDENTITY),
    LintId::of(methods::GET_LAST_WITH_LEN),
    LintId::of(methods::INSPECT_FOR_EACH),
    LintId::of(methods::ITER_COUNT),
    LintId::of(methods::MANUAL_FILTER_MAP),
    LintId::of(methods::MANUAL_FIND_MAP),
    LintId::of(methods::MANUAL_SPLIT_ONCE),
    LintId::of(methods::MAP_FLATTEN),
    LintId::of(methods::MAP_IDENTITY),
    LintId::of(methods::NEEDLESS_OPTION_AS_DEREF),
    LintId::of(methods::NEEDLESS_OPTION_TAKE),
    LintId::of(methods::NEEDLESS_SPLITN),
    LintId::of(methods::OPTION_AS_REF_DEREF),
    LintId::of(methods::OPTION_FILTER_MAP),
    LintId::of(methods::OR_THEN_UNWRAP),
    LintId::of(methods::SEARCH_IS_SOME),
    LintId::of(methods::SKIP_WHILE_NEXT),
    LintId::of(methods::UNNECESSARY_FILTER_MAP),
    LintId::of(methods::UNNECESSARY_FIND_MAP),
    LintId::of(methods::USELESS_ASREF),
    LintId::of(misc::SHORT_CIRCUIT_STATEMENT),
    LintId::of(misc_early::UNNEEDED_WILDCARD_PATTERN),
    LintId::of(misc_early::ZERO_PREFIXED_LITERAL),
    LintId::of(mixed_read_write_in_expression::DIVERGING_SUB_EXPRESSION),
    LintId::of(needless_arbitrary_self_type::NEEDLESS_ARBITRARY_SELF_TYPE),
    LintId::of(needless_bool::BOOL_COMPARISON),
    LintId::of(needless_bool::NEEDLESS_BOOL),
    LintId::of(needless_borrowed_ref::NEEDLESS_BORROWED_REFERENCE),
    LintId::of(needless_question_mark::NEEDLESS_QUESTION_MARK),
    LintId::of(needless_update::NEEDLESS_UPDATE),
    LintId::of(neg_cmp_op_on_partial_ord::NEG_CMP_OP_ON_PARTIAL_ORD),
    LintId::of(no_effect::NO_EFFECT),
    LintId::of(no_effect::UNNECESSARY_OPERATION),
    LintId::of(overflow_check_conditional::OVERFLOW_CHECK_CONDITIONAL),
    LintId::of(partialeq_ne_impl::PARTIALEQ_NE_IMPL),
    LintId::of(precedence::PRECEDENCE),
    LintId::of(ptr_offset_with_cast::PTR_OFFSET_WITH_CAST),
    LintId::of(ranges::RANGE_ZIP_WITH_LEN),
    LintId::of(redundant_closure_call::REDUNDANT_CLOSURE_CALL),
    LintId::of(redundant_slicing::REDUNDANT_SLICING),
    LintId::of(reference::DEREF_ADDROF),
    LintId::of(repeat_once::REPEAT_ONCE),
    LintId::of(strings::STRING_FROM_UTF8_AS_BYTES),
    LintId::of(strlen_on_c_strings::STRLEN_ON_C_STRINGS),
    LintId::of(swap::MANUAL_SWAP),
    LintId::of(temporary_assignment::TEMPORARY_ASSIGNMENT),
    LintId::of(transmute::CROSSPOINTER_TRANSMUTE),
    LintId::of(transmute::TRANSMUTES_EXPRESSIBLE_AS_PTR_CASTS),
    LintId::of(transmute::TRANSMUTE_BYTES_TO_STR),
    LintId::of(transmute::TRANSMUTE_FLOAT_TO_INT),
    LintId::of(transmute::TRANSMUTE_INT_TO_BOOL),
    LintId::of(transmute::TRANSMUTE_INT_TO_CHAR),
    LintId::of(transmute::TRANSMUTE_INT_TO_FLOAT),
    LintId::of(transmute::TRANSMUTE_NUM_TO_BYTES),
    LintId::of(transmute::TRANSMUTE_PTR_TO_REF),
    LintId::of(types::BORROWED_BOX),
    LintId::of(types::TYPE_COMPLEXITY),
    LintId::of(types::VEC_BOX),
    LintId::of(unit_types::UNIT_ARG),
    LintId::of(unnecessary_sort_by::UNNECESSARY_SORT_BY),
    LintId::of(unwrap::UNNECESSARY_UNWRAP),
    LintId::of(useless_conversion::USELESS_CONVERSION),
    LintId::of(zero_div_zero::ZERO_DIVIDED_BY_ZERO),
])
