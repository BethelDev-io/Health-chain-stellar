use crate::{BloodComponent, BloodType, ContractError, RequestContract, RequestStatus, Urgency};
use soroban_sdk::{
    testutils::{Address as _, Ledger as _},
    Address, Env, String,
};

/// #1151: Verify cancel_request requires caller authorization and must be hospital owner or admin.
/// A third party cannot cancel a hospital's blood request.
#[test]
fn test_cancel_request_requires_ownership() {
    let env = Env::default();
    let admin = Address::generate(&env);
    let hospital_a = Address::generate(&env);
    let hospital_b = Address::generate(&env);

    env.mock_all_auths();

    RequestContract::initialize(env.clone(), admin.clone(), Address::generate(&env)).unwrap();
    RequestContract::authorize_hospital(env.clone(), hospital_a.clone()).unwrap();
    RequestContract::authorize_hospital(env.clone(), hospital_b.clone()).unwrap();

    env.ledger().set_timestamp(1_000);

    // Hospital A creates a request
    let request_id = RequestContract::create_request(
        env.clone(),
        hospital_a.clone(),
        BloodType::OPositive,
        BloodComponent::WholeBlood,
        500,
        Urgency::Urgent,
        1_600,
    )
    .unwrap();

    // Hospital B attempts to cancel Hospital A's request
    let result = RequestContract::cancel_request(
        env.clone(),
        hospital_b.clone(),
        request_id,
        String::from_str(&env, "unauthorized cancel"),
    );

    // Should fail: Hospital B is not the owner and not admin
    assert_eq!(result, Err(ContractError::NotRequestOwner));
}

/// #1151: Verify cancel_request succeeds for hospital owner.
#[test]
fn test_cancel_request_by_owner_succeeds() {
    let env = Env::default();
    let admin = Address::generate(&env);
    let hospital = Address::generate(&env);

    env.mock_all_auths();

    RequestContract::initialize(env.clone(), admin.clone(), Address::generate(&env)).unwrap();
    RequestContract::authorize_hospital(env.clone(), hospital.clone()).unwrap();

    env.ledger().set_timestamp(1_000);

    let request_id = RequestContract::create_request(
        env.clone(),
        hospital.clone(),
        BloodType::OPositive,
        BloodComponent::WholeBlood,
        500,
        Urgency::Urgent,
        1_600,
    )
    .unwrap();

    let result = RequestContract::cancel_request(
        env.clone(),
        hospital.clone(),
        request_id,
        String::from_str(&env, "owned cancel"),
    );

    assert!(result.is_ok());
}

/// #1151: Verify cancel_request succeeds for admin.
#[test]
fn test_cancel_request_by_admin_succeeds() {
    let env = Env::default();
    let admin = Address::generate(&env);
    let hospital = Address::generate(&env);

    env.mock_all_auths();

    RequestContract::initialize(env.clone(), admin.clone(), Address::generate(&env)).unwrap();
    RequestContract::authorize_hospital(env.clone(), hospital.clone()).unwrap();

    env.ledger().set_timestamp(1_000);

    let request_id = RequestContract::create_request(
        env.clone(),
        hospital.clone(),
        BloodType::OPositive,
        BloodComponent::WholeBlood,
        500,
        Urgency::Urgent,
        1_600,
    )
    .unwrap();

    let result = RequestContract::cancel_request(
        env.clone(),
        admin.clone(),
        request_id,
        String::from_str(&env, "admin cancel"),
    );

    assert!(result.is_ok());
}

/// #1151: Verify update_request_status requires admin authorization.
/// Only admin can call this function.
#[test]
fn test_update_request_status_requires_admin() {
    let env = Env::default();
    let admin = Address::generate(&env);
    let hospital = Address::generate(&env);
    let non_admin = Address::generate(&env);

    env.mock_all_auths();

    RequestContract::initialize(env.clone(), admin.clone(), Address::generate(&env)).unwrap();
    RequestContract::authorize_hospital(env.clone(), hospital.clone()).unwrap();

    env.ledger().set_timestamp(1_000);

    let request_id = RequestContract::create_request(
        env.clone(),
        hospital.clone(),
        BloodType::OPositive,
        BloodComponent::WholeBlood,
        500,
        Urgency::Urgent,
        1_600,
    )
    .unwrap();

    // Non-admin attempts to update request status
    let result = RequestContract::update_request_status(
        env.clone(),
        non_admin,
        request_id,
        RequestStatus::Approved,
        String::from_str(&env, "status update"),
    );

    // Should fail: caller is not admin
    assert_eq!(result, Err(ContractError::Unauthorized));
}

/// #1151: Verify update_request_status succeeds for admin.
#[test]
fn test_update_request_status_by_admin_succeeds() {
    let env = Env::default();
    let admin = Address::generate(&env);
    let hospital = Address::generate(&env);

    env.mock_all_auths();

    RequestContract::initialize(env.clone(), admin.clone(), Address::generate(&env)).unwrap();
    RequestContract::authorize_hospital(env.clone(), hospital.clone()).unwrap();

    env.ledger().set_timestamp(1_000);

    let request_id = RequestContract::create_request(
        env.clone(),
        hospital.clone(),
        BloodType::OPositive,
        BloodComponent::WholeBlood,
        500,
        Urgency::Urgent,
        1_600,
    )
    .unwrap();

    let result = RequestContract::update_request_status(
        env.clone(),
        admin.clone(),
        request_id,
        RequestStatus::Approved,
        String::from_str(&env, "admin status update"),
    );

    assert!(result.is_ok());
}
