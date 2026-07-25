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
}
