use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};
use super::*;

#[test]
fn create_claim_works(){
    new_test_ext().execute_with(||{
        let claim = vec![1, 2];
        assert_ok!(TemplateModule::create_claim(Origin::signed(1), claim.clone()));
        assert_eq!(Proofs::<Test>::get(&claim), (1, frame_system::Module::<Test>::block_number()));
    })
}

#[test]
fn create_claim_failed_when_claim_already_exist(){
    new_test_ext().execute_with(||{
        let claim = vec![1, 2];
        assert_ok!(TemplateModule::create_claim(Origin::signed(1), claim.clone()));

        assert_noop!(
            TemplateModule::create_claim(Origin::signed(1), claim.clone()),
            Error::<Test>::ProofAlreadyClaimed
        );
    })
}

#[test]
fn create_claim_failed_when_proof_too_long(){
    new_test_ext().execute_with(||{
        let hash = "0x7bd10a1e21ce718846b5dcf1dece9b08a4d84958ae74024b3160bbc748f624cf123";
        let claim:Vec<u8> = Vec::from(hash);

        assert_noop!(
            TemplateModule::create_claim(Origin::signed(1), claim.clone()),
            Error::<Test>::ProofTooLong
        );
    })
}

#[test]
fn revoke_claim_works(){
    new_test_ext().execute_with(||{
        let claim = vec![1, 2];
        assert_ok!(TemplateModule::create_claim(Origin::signed(1), claim.clone()));

        assert_ok!(TemplateModule::revoke_claim(Origin::signed(1), claim.clone()));
    })
}

#[test]
fn revoke_claim_failed_when_no_such_proof(){
    new_test_ext().execute_with(||{
        let claim = vec![1, 2];

        assert_noop!(
            TemplateModule::revoke_claim(Origin::signed(1), claim.clone()),
            Error::<Test>::NoSuchProof
        );
    })
}

#[test]
fn revoke_claim_failed_when_no_proof_owner(){
    new_test_ext().execute_with(||{
        let claim = vec![1, 2];
        assert_ok!(TemplateModule::create_claim(Origin::signed(1), claim.clone()));

        assert_noop!(
            TemplateModule::revoke_claim(Origin::signed(2), claim.clone()),
            Error::<Test>::NotProofOwner
        );
    })
}

#[test]
fn move_claim_works(){
    new_test_ext().execute_with(||{
        let claim = vec![1, 2];
        assert_ok!(TemplateModule::create_claim(Origin::signed(1), claim.clone()));

        let new_owner = ensure_signed(Origin::signed(2)).unwrap();
        assert_ok!(TemplateModule::move_claim(Origin::signed(1), new_owner, claim.clone()));
    });
}

#[test]
fn move_claim_failed_when_no_such_proof(){
    new_test_ext().execute_with(||{
        let claim = vec![1, 2];

        let new_owner = ensure_signed(Origin::signed(2)).unwrap();
        assert_noop!(
            TemplateModule::move_claim(Origin::signed(1), new_owner, claim.clone()),
            Error::<Test>::NoSuchProof
        );
    })
}

#[test]
fn move_claim_failed_when_no_proof_owner(){
    new_test_ext().execute_with(||{
        let claim = vec![1, 2];
        assert_ok!(TemplateModule::create_claim(Origin::signed(1), claim.clone()));

        let new_owner = ensure_signed(Origin::signed(2)).unwrap();
        assert_noop!(
            TemplateModule::move_claim(Origin::signed(3), new_owner, claim.clone()),
            Error::<Test>::NotProofOwner
        );
    })
}

