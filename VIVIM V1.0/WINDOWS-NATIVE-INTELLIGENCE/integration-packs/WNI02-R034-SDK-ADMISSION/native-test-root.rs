mod campaign;
mod scheduler;
mod resource_campaign;
mod resources;
mod hardware;
#[path="native-sampling.rs"] mod native_sampling;
mod protocol;
mod meta_language;
mod meta_resident;
#[path="meta-policy.rs"] mod meta_policy;
#[path="workload-profile.rs"] mod workload_profile;

#[cfg(windows)] mod workstation_process;
#[cfg(windows)] mod owned_campaign;
#[cfg(all(windows,test))] #[path="owned-metrics-tests.rs"] mod owned_metrics_tests;
