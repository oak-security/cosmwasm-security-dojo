#[cfg(test)]
pub mod tests {
    use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, UserResponse};
    use cw_multi_test::{App, Contract, ContractWrapper, Executor};
    use cosmwasm_std::{coin, coins, Addr, Empty};
    use cosmwasm_std:: Uint128;

    pub const USER: &str = "user";
    pub const ADMIN: &str = "admin";
    pub const DENOM: &str = "uoaksec";

    pub fn challenge_contract() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(
            crate::contract::execute,
            crate::contract::instantiate,
            crate::contract::query,
        );
        Box::new(contract)
    }

    pub fn proper_instantiate() -> (App, Addr) {
        let mut app = App::default();
        let cw_template_id = app.store_code(challenge_contract());

        // init contract
        let msg = InstantiateMsg { initial_deny: vec![] };
        let contract_addr = app
            .instantiate_contract(
                cw_template_id,
                Addr::unchecked(ADMIN),
                &msg,
                &[],
                "test",
                None,
            )
            .unwrap();

        // mint funds to user
        app = mint_tokens(app, USER.to_string(), Uint128::from(100u128));
        
        (app, contract_addr)
    }  

    pub fn mint_tokens(mut app: App, recipient: String, amount: Uint128) -> App {
        app.sudo(cw_multi_test::SudoMsg::Bank(
            cw_multi_test::BankSudo::Mint {
                to_address: recipient.to_owned(),
                amount: vec![coin(amount.u128(), DENOM)],
            },
        ))
        .unwrap();
        app
    }

   #[test]
    fn deposit_withdraw() {
        let (mut app, contract_addr) = proper_instantiate();
        let sender = Addr::unchecked(USER);

        // Deposit funds
        app.execute_contract(
            sender.clone(), 
            contract_addr.clone(), 
            &ExecuteMsg::Deposit {}, 
            &coins(100, DENOM)
        ).unwrap();
        // Verify deposit succeeded
        let msg = QueryMsg::GetUserData {
            address: USER.to_string(),
        };
        let userdata: UserResponse = app
            .wrap()
            .query_wasm_smart(contract_addr.clone(), &msg)
            .unwrap();
        assert_eq!(userdata.amount, Uint128::from(100_u64));

        // Withdraw funds
        app.execute_contract(
            sender.clone(), 
            contract_addr.clone(), 
            &ExecuteMsg::Withdraw {
                amount: Uint128::from(100_u64),
                destination: Some("rcpt".to_string()),
            }, 
            &[],
        ).unwrap();
        // Verify withdraw succeeded
        let msg = QueryMsg::GetUserData {
            address: USER.to_string(),
        };
        let userdata: UserResponse = app
            .wrap()
            .query_wasm_smart(contract_addr, &msg)
            .unwrap();
        assert_eq!(userdata.amount, Uint128::from(0_u64));        
    }


    #[test]
    fn denylisting() {
        let (mut app, contract_addr) = proper_instantiate();
        let sender = Addr::unchecked(USER);

        // Deposit funds
        app.execute_contract(
            sender.clone(), 
            contract_addr.clone(), 
            &ExecuteMsg::Deposit {}, 
            &coins(100, DENOM),
        ).unwrap();

        // Deny-list rcpt
        app.execute_contract(
            Addr::unchecked(ADMIN), 
            contract_addr.clone(), 
            &ExecuteMsg::AddToDenylist { address: "rcpt".to_string() }, 
            &[],
        ).unwrap();

        // Failed withdraw of funds
        let res = app.execute_contract(
            sender.clone(), 
            contract_addr.clone(), 
            &ExecuteMsg::Withdraw {
                amount: Uint128::from(100_u64),
                destination: Some("rcpt".to_string()),
            },
            &[],
        ).unwrap_err();
        assert_eq!(
            res.root_cause().to_string(), 
            "Address is part of the denylist."
        );      

        // Allow-list rcpt
        app.execute_contract(
            Addr::unchecked(ADMIN), 
            contract_addr.clone(), 
            &ExecuteMsg::RemoveFromDenylist { address: "rcpt".to_string() }, 
            &[],
        ).unwrap();

        // Successful withdraw of funds
        app.execute_contract(
            sender.clone(), 
            contract_addr.clone(), 
            &ExecuteMsg::Withdraw {
                amount: Uint128::from(100_u64),
                destination: Some("rcpt".to_string()),
            },
            &[],
        ).unwrap();
        
        // Verify deposit
        let msg = QueryMsg::GetUserData {
            address: USER.to_string(),
        };
        let userdata: UserResponse = app
            .wrap()
            .query_wasm_smart(contract_addr, &msg)
            .unwrap();
        assert_eq!(userdata.amount, Uint128::from(0_u64));        
    }   

}

