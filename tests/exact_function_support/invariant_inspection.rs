// Included only in the test's copy of the emitted source. This exposes the
// original constructor result for diagnostic evidence, without replacing it.
pub(super) fn inspect_environment(
    declaring_package: &CheckedPackageV2,
    acquired: &qsl_replay::CheckedClosureCompilation,
    selection: &qsl_replay::CallableSelection,
    resources: &quire_contract_codegen::CheckedFunctionResources,
) -> Result<sv::declaration::TypeEnvironment, sv::declaration::EnvironmentFailure> {
    PreparedFunction::new(
        declaring_package,
        acquired,
        selection,
        resources,
        sv::location::ExpressionLocation::root(sv::location::ExpressionOwner::Expression),
    )
    .expect("authentic selected owner inputs")
    .environment
}
