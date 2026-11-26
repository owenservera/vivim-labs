mod resources;mod hardware;mod scheduler;mod fixture;
#[test]fn reference_scores_and_fixed_model(){assert_eq!(fixture::reference(),[6.125,1.125,3.125,1.625]);assert!(!fixture::MODEL.is_empty());}
