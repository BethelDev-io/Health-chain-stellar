#[cfg(test)]
mod security_tests {
    use crate::{BloodStatus, BloodType, ContractError, InventoryContract};
    use soroban_sdk::{testutils::Address as _, Address, Env, String};

    /// #1150: Verify reserve_blood enforces bank_id ownership check.
    /// Bank B cannot reserve blood units that belong to Bank A.
    #[test]
    fn test_reserve_blood_prevents_cross_bank_allocation() {
        let env = Env::default();
        let admin = Address::random(&env);
        let bank_a = Address::random(&env);
        let bank_b = Address::random(&env);
        let hospital = Address::random(&env);

        env.mock_all_auths();

        InventoryContract::initialize(env.clone(), admin).unwrap();
        InventoryContract::authorize_bank(env.clone(), admin.clone(), bank_a.clone())
            .unwrap();
        InventoryContract::authorize_bank(env.clone(), admin.clone(), bank_b.clone())
            .unwrap();

        // Bank A registers a blood unit
        let unit_id = InventoryContract::register_blood(
            env.clone(),
            bank_a.clone(),
            String::from_slice(&env, "SN001"),
            BloodType::OPos,
            500,
            None,
        )
        .unwrap();

        // Bank B attempts to reserve Bank A's blood unit
        let result = InventoryContract::reserve_blood(
            env.clone(),
            bank_b.clone(),
            {
                let mut units = soroban_sdk::Vec::new(&env);
                units.push_back(unit_id);
                units
            },
            1,
            3600,
        );

        // Should fail: unit belongs to Bank A, not Bank B
        assert_eq!(result, Err(ContractError::NotUnitOwner));
    }

    /// #1150: Verify reserve_blood allows owner bank to reserve its own units.
    #[test]
    fn test_reserve_blood_owner_bank_succeeds() {
        let env = Env::default();
        let admin = Address::random(&env);
        let bank_a = Address::random(&env);

        env.mock_all_auths();

        InventoryContract::initialize(env.clone(), admin).unwrap();
        InventoryContract::authorize_bank(env.clone(), admin.clone(), bank_a.clone())
            .unwrap();

        // Bank A registers a blood unit
        let unit_id = InventoryContract::register_blood(
            env.clone(),
            bank_a.clone(),
            String::from_slice(&env, "SN001"),
            BloodType::OPos,
            500,
            None,
        )
        .unwrap();

        // Bank A reserves its own blood unit
        let result = InventoryContract::reserve_blood(
            env.clone(),
            bank_a.clone(),
            {
                let mut units = soroban_sdk::Vec::new(&env);
                units.push_back(unit_id);
                units
            },
            1,
            3600,
        );

        assert!(result.is_ok());
    }

    /// #1150: Verify batch operations enforce bank ownership.
    #[test]
    fn test_batch_reserve_blood_enforces_ownership() {
        let env = Env::default();
        let admin = Address::random(&env);
        let bank_a = Address::random(&env);
        let bank_b = Address::random(&env);

        env.mock_all_auths();

        InventoryContract::initialize(env.clone(), admin).unwrap();
        InventoryContract::authorize_bank(env.clone(), admin.clone(), bank_a.clone())
            .unwrap();
        InventoryContract::authorize_bank(env.clone(), admin.clone(), bank_b.clone())
            .unwrap();

        // Bank A registers units
        let unit_1 = InventoryContract::register_blood(
            env.clone(),
            bank_a.clone(),
            String::from_slice(&env, "SN001"),
            BloodType::OPos,
            500,
            None,
        )
        .unwrap();

        let unit_2 = InventoryContract::register_blood(
            env.clone(),
            bank_a.clone(),
            String::from_slice(&env, "SN002"),
            BloodType::OPos,
            500,
            None,
        )
        .unwrap();

        // Bank B attempts batch reserve of Bank A's units
        let mut unit_ids = soroban_sdk::Vec::new(&env);
        unit_ids.push_back(unit_1);
        unit_ids.push_back(unit_2);

        let result = InventoryContract::batch_reserve_blood(
            env.clone(),
            bank_b.clone(),
            unit_ids,
            1,
            3600,
        );

        // Should fail on first unit check
        assert_eq!(result, Err(ContractError::NotUnitOwner));
    }

    /// #1153: Verify register_blood requires bank authorization.
    /// Unauthorized addresses cannot register blood units.
    #[test]
    fn test_register_blood_requires_authorized_bank() {
        let env = Env::default();
        let admin = Address::random(&env);
        let unauthorized_bank = Address::random(&env);

        env.mock_all_auths();

        InventoryContract::initialize(env.clone(), admin).unwrap();

        // Unauthorized bank attempts to register blood
        let result = InventoryContract::register_blood(
            env.clone(),
            unauthorized_bank,
            String::from_slice(&env, "SN001"),
            BloodType::OPos,
            500,
            None,
        );

        // Should fail: bank is not authorized
        assert_eq!(result, Err(ContractError::NotAuthorizedBloodBank));
    }

    /// #1153: Verify register_blood succeeds for authorized banks.
    #[test]
    fn test_register_blood_by_authorized_bank_succeeds() {
        let env = Env::default();
        let admin = Address::random(&env);
        let authorized_bank = Address::random(&env);

        env.mock_all_auths();

        InventoryContract::initialize(env.clone(), admin.clone()).unwrap();
        InventoryContract::authorize_bank(env.clone(), admin, authorized_bank.clone())
            .unwrap();

        let result = InventoryContract::register_blood(
            env.clone(),
            authorized_bank,
            String::from_slice(&env, "SN001"),
            BloodType::OPos,
            500,
            None,
        );

        assert!(result.is_ok());
    }

    /// #1315: Verify reserve_blood rejects oversized duration_seconds with typed error.
    /// Previously this panicked; now it should return InvalidInput error.
    #[test]
    fn test_reserve_blood_rejects_oversized_duration() {
        let env = Env::default();
        let admin = Address::random(&env);
        let bank = Address::random(&env);

        env.mock_all_auths();

        InventoryContract::initialize(env.clone(), admin.clone()).unwrap();
        InventoryContract::authorize_bank(env.clone(), admin, bank.clone())
            .unwrap();

        let unit_id = InventoryContract::register_blood(
            env.clone(),
            bank.clone(),
            String::from_slice(&env, "SN001"),
            BloodType::OPos,
            500,
            None,
        )
        .unwrap();

        // Attempt to reserve with duration exceeding MAX_RESERVATION_DURATION_SECS (86400 * 7)
        let max_allowed = 86_400 * 7;
        let oversized_duration = max_allowed + 1;

        let mut unit_ids = soroban_sdk::Vec::new(&env);
        unit_ids.push_back(unit_id);

        let result = InventoryContract::reserve_blood(
            env.clone(),
            bank.clone(),
            unit_ids,
            1,
            oversized_duration,
        );

        // Should return InvalidInput error, not panic
        assert_eq!(result, Err(ContractError::InvalidInput));
    }

    /// #1315: Verify reserve_blood succeeds with maximum allowed duration.
    #[test]
    fn test_reserve_blood_accepts_max_duration() {
        let env = Env::default();
        let admin = Address::random(&env);
        let bank = Address::random(&env);

        env.mock_all_auths();

        InventoryContract::initialize(env.clone(), admin.clone()).unwrap();
        InventoryContract::authorize_bank(env.clone(), admin, bank.clone())
            .unwrap();

        let unit_id = InventoryContract::register_blood(
            env.clone(),
            bank.clone(),
            String::from_slice(&env, "SN001"),
            BloodType::OPos,
            500,
            None,
        )
        .unwrap();

        let max_allowed = 86_400 * 7;
        let mut unit_ids = soroban_sdk::Vec::new(&env);
        unit_ids.push_back(unit_id);

        let result = InventoryContract::reserve_blood(
            env.clone(),
            bank,
            unit_ids,
            1,
            max_allowed,
        );

        assert!(result.is_ok());
    }

    /// #1314: Verify release_reservation_by_contract is callable as a public entry point.
    /// This tests that the function signature matches the requests contract's #[contractclient] interface.
    #[test]
    fn test_release_reservation_by_contract_is_callable() {
        let env = Env::default();
        let admin = Address::random(&env);
        let bank = Address::random(&env);
        let authorized_contract = Address::random(&env);

        env.mock_all_auths();

        InventoryContract::initialize(env.clone(), admin.clone()).unwrap();
        InventoryContract::authorize_bank(env.clone(), admin, bank.clone())
            .unwrap();

        let unit_id = InventoryContract::register_blood(
            env.clone(),
            bank.clone(),
            String::from_slice(&env, "SN001"),
            BloodType::OPos,
            500,
            None,
        )
        .unwrap();

        let mut unit_ids = soroban_sdk::Vec::new(&env);
        unit_ids.push_back(unit_id);

        let reservation_id = InventoryContract::reserve_blood(
            env.clone(),
            bank.clone(),
            unit_ids,
            1,
            3600,
        )
        .unwrap();

        // Verify reservation exists before release
        let reservation = InventoryContract::get_reservation(env.clone(), reservation_id).unwrap();
        assert_eq!(reservation.unit_ids.len(), 1);

        // Call release_reservation_by_contract with owned Env and Address (the public signature)
        let result = InventoryContract::release_reservation_by_contract(
            env.clone(),
            authorized_contract,
            reservation_id,
        );

        // Should succeed (authorized_contract auth is mocked, so it passes)
        assert!(result.is_ok());

        // Verify reservation was released
        let result = InventoryContract::get_reservation(env, reservation_id);
        assert_eq!(result, Err(ContractError::ReservationNotFound));
    }
}
