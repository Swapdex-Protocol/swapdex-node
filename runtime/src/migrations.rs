// This file is part of SwapDEX.


// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.


use super::*;

pub struct RemoveCollectiveFlip;
impl frame_support::traits::OnRuntimeUpgrade for RemoveCollectiveFlip {
    fn on_runtime_upgrade() -> Weight {
        use frame_support::storage::migration;
        // Remove the storage value `RandomMaterial` from removed pallet `RandomnessCollectiveFlip`
        migration::remove_storage_prefix(b"RandomnessCollectiveFlip", b"RandomMaterial", b"");
        <Runtime as frame_system::Config>::DbWeight::get().writes(1)
    }
}

/// Migrate from `PalletVersion` to the new `StorageVersion`
pub struct MigratePalletVersionToStorageVersion;
impl frame_support::traits::OnRuntimeUpgrade for MigratePalletVersionToStorageVersion {
    fn on_runtime_upgrade() -> frame_support::weights::Weight {
        frame_support::migrations::migrate_from_pallet_version_to_storage_version::<
            AllPalletsWithSystem,
        >(&RocksDbWeight::get())
    }
}

// 10 SDX
const OLD_CANDIDACY_BOND: Balance = 1000 * DOLLARS;
// 10 mSDX
const OLD_VOTING_BOND: Balance = DOLLARS;
pub struct PhragmenElectionDepositRuntimeUpgrade;
impl pallet_elections_phragmen::migrations::v3::V2ToV3 for PhragmenElectionDepositRuntimeUpgrade {
    type Pallet = Elections;
    type AccountId = AccountId;
    type Balance = Balance;
}
impl frame_support::traits::OnRuntimeUpgrade for PhragmenElectionDepositRuntimeUpgrade {
    fn on_runtime_upgrade() -> frame_support::weights::Weight {
        pallet_elections_phragmen::migrations::v3::apply::<Self>(
            OLD_VOTING_BOND,
            OLD_CANDIDACY_BOND,
        )
    }
}

pub use node_primitives::{AccountId, Index};

pub struct SystemToTripleRefCount;
impl frame_system::migrations::V2ToV3 for SystemToTripleRefCount {
    type Pallet = System;
    type AccountId = AccountId;
    type Index = Index;
    type AccountData = pallet_balances::AccountData<Balance>;
}
impl frame_support::traits::OnRuntimeUpgrade for SystemToTripleRefCount {
    fn on_runtime_upgrade() -> frame_support::weights::Weight {
        frame_system::migrations::migrate_from_dual_to_triple_ref_count::<Self>()
    }
}

impl pallet_babe::migrations::BabePalletPrefix for Runtime {
    fn pallet_prefix() -> &'static str {
        "Babe"
    }
}
pub struct BabeEpochConfigMigrations;
impl frame_support::traits::OnRuntimeUpgrade for BabeEpochConfigMigrations {
    fn on_runtime_upgrade() -> frame_support::weights::Weight {
        pallet_babe::migrations::add_epoch_configuration::<Runtime>(
            sp_consensus_babe::BabeEpochConfiguration {
                allowed_slots: sp_consensus_babe::AllowedSlots::PrimaryAndSecondaryPlainSlots,
                ..BABE_GENESIS_EPOCH_CONFIG
            },
        )
    }
}

pub struct GrandpaStoragePrefixMigration;
impl frame_support::traits::OnRuntimeUpgrade for GrandpaStoragePrefixMigration {
    fn on_runtime_upgrade() -> frame_support::weights::Weight {
        use frame_support::traits::PalletInfo;
        let name = <Runtime as frame_system::Config>::PalletInfo::name::<Grandpa>()
            .expect("grandpa is part of pallets in construct_runtime, so it has a name; qed");
        pallet_grandpa::migrations::v4::migrate::<Runtime, &str>(name)
    }
}

/// Migrate staking
pub struct MigratePalletStakingV5toV7;
impl frame_support::traits::OnRuntimeUpgrade for MigratePalletStakingV5toV7 {
    fn on_runtime_upgrade() -> Weight {
        log::info!("Migrating staking from V5 to V8");
        let mut weight = 0;
        weight += pallet_staking::migrations::v6::migrate::<Runtime>();
        weight += pallet_staking::migrations::v7::migrate::<Runtime>();
        weight
    }
}

// v0.9.11
// Migration to generate pallet staking's `SortedListProvider` from pre-existing nominators.
pub struct StakingBagsListMigrationV8;

impl OnRuntimeUpgrade for StakingBagsListMigrationV8 {
    fn on_runtime_upgrade() -> frame_support::weights::Weight {
        pallet_staking::migrations::v8::migrate::<Runtime>()
    }

    #[cfg(feature = "try-runtime")]
    fn pre_upgrade() -> Result<(), &'static str> {
        pallet_staking::migrations::v8::pre_migrate::<Runtime>()
    }

    #[cfg(feature = "try-runtime")]
    fn post_upgrade() -> Result<(), &'static str> {
        pallet_staking::migrations::v8::post_migrate::<Runtime>()
    }
}

const COUNCIL_OLD_PREFIX: &str = "Instance1Collective";
/// Migrate from `Instance1Collective` to the new pallet prefix `Council`
pub struct CouncilStoragePrefixMigration;
impl frame_support::traits::OnRuntimeUpgrade for CouncilStoragePrefixMigration {
    fn on_runtime_upgrade() -> frame_support::weights::Weight {
        pallet_collective::migrations::v4::migrate::<Runtime, Council, _>(COUNCIL_OLD_PREFIX)
    }

    #[cfg(feature = "try-runtime")]
    fn pre_upgrade() -> Result<(), &'static str> {
        pallet_collective::migrations::v4::pre_migrate::<Council, _>(COUNCIL_OLD_PREFIX);
        Ok(())
    }

    #[cfg(feature = "try-runtime")]
    fn post_upgrade() -> Result<(), &'static str> {
        pallet_collective::migrations::v4::post_migrate::<Council, _>(COUNCIL_OLD_PREFIX);
        Ok(())
    }
}

const TECHNICAL_COMMITTEE_OLD_PREFIX: &str = "Instance2Collective";
/// Migrate from `Instance2Collective` to the new pallet prefix `TechnicalCommittee`
pub struct TechnicalCommitteeStoragePrefixMigration;
impl frame_support::traits::OnRuntimeUpgrade for TechnicalCommitteeStoragePrefixMigration {
    fn on_runtime_upgrade() -> frame_support::weights::Weight {
        pallet_collective::migrations::v4::migrate::<Runtime, TechnicalCommittee, _>(
            TECHNICAL_COMMITTEE_OLD_PREFIX,
        )
    }

    #[cfg(feature = "try-runtime")]
    fn pre_upgrade() -> Result<(), &'static str> {
        pallet_collective::migrations::v4::pre_migrate::<TechnicalCommittee, _>(
            TECHNICAL_COMMITTEE_OLD_PREFIX,
        );
        Ok(())
    }

    #[cfg(feature = "try-runtime")]
    fn post_upgrade() -> Result<(), &'static str> {
        pallet_collective::migrations::v4::post_migrate::<TechnicalCommittee, _>(
            TECHNICAL_COMMITTEE_OLD_PREFIX,
        );
        Ok(())
    }
}

const TECHNICAL_MEMBERSHIP_OLD_PREFIX: &str = "Instance1Membership";
/// Migrate from `Instance1Membership` to the new pallet prefix `TechnicalMembership`
pub struct TechnicalMembershipStoragePrefixMigration;
impl frame_support::traits::OnRuntimeUpgrade for TechnicalMembershipStoragePrefixMigration {
    fn on_runtime_upgrade() -> frame_support::weights::Weight {
        use frame_support::traits::PalletInfo;
        let name = <Runtime as frame_system::Config>::PalletInfo::name::<TechnicalMembership>()
            .expect("TechnicalMembership is part of runtime, so it has a name; qed");
        pallet_membership::migrations::v4::migrate::<Runtime, TechnicalMembership, _>(
            TECHNICAL_MEMBERSHIP_OLD_PREFIX,
            name,
        )
    }

    #[cfg(feature = "try-runtime")]
    fn pre_upgrade() -> Result<(), &'static str> {
        use frame_support::traits::PalletInfo;
        let name = <Runtime as frame_system::Config>::PalletInfo::name::<TechnicalMembership>()
            .expect("TechnicalMembership is part of runtime, so it has a name; qed");
        pallet_membership::migrations::v4::pre_migrate::<TechnicalMembership, _>(
            TECHNICAL_MEMBERSHIP_OLD_PREFIX,
            name,
        );
        Ok(())
    }

    #[cfg(feature = "try-runtime")]
    fn post_upgrade() -> Result<(), &'static str> {
        use frame_support::traits::PalletInfo;
        let name = <Runtime as frame_system::Config>::PalletInfo::name::<TechnicalMembership>()
            .expect("TechnicalMembership is part of runtime, so it has a name; qed");
        pallet_membership::migrations::v4::post_migrate::<TechnicalMembership, _>(
            TECHNICAL_MEMBERSHIP_OLD_PREFIX,
            name,
        );
        Ok(())
    }
}

const TIPS_OLD_PREFIX: &str = "Treasury";
/// Migrate pallet-tips from `Treasury` to the new pallet prefix `Tips`
pub struct MigrateTipsPalletPrefix;
impl frame_support::traits::OnRuntimeUpgrade for MigrateTipsPalletPrefix {
    fn on_runtime_upgrade() -> frame_support::weights::Weight {
        pallet_tips::migrations::v4::migrate::<Runtime, Tips, _>(TIPS_OLD_PREFIX)
    }

    #[cfg(feature = "try-runtime")]
    fn pre_upgrade() -> Result<(), &'static str> {
        pallet_tips::migrations::v4::pre_migrate::<Runtime, Tips, _>(TIPS_OLD_PREFIX);
        Ok(())
    }

    #[cfg(feature = "try-runtime")]
    fn post_upgrade() -> Result<(), &'static str> {
        pallet_tips::migrations::v4::post_migrate::<Runtime, Tips, _>(TIPS_OLD_PREFIX);
        Ok(())
    }
}

const BOUNTIES_OLD_PREFIX: &str = "Treasury";
/// Migrate from 'Treasury' to the new prefix 'Bounties'
pub struct BountiesPrefixMigration;
impl frame_support::traits::OnRuntimeUpgrade for BountiesPrefixMigration {
    fn on_runtime_upgrade() -> frame_support::weights::Weight {
        use frame_support::traits::PalletInfo;
        let name = <Runtime as frame_system::Config>::PalletInfo::name::<Bounties>()
            .expect("Bounties is part of runtime, so it has a name; qed");
        pallet_bounties::migrations::v4::migrate::<Runtime, Bounties, _>(BOUNTIES_OLD_PREFIX, name)
    }
    #[cfg(feature = "try-runtime")]
    fn pre_upgrade() -> Result<(), &'static str> {
        use frame_support::traits::PalletInfo;
        let name = <Runtime as frame_system::Config>::PalletInfo::name::<Bounties>()
            .expect("Bounties is part of runtime, so it has a name; qed");
        pallet_bounties::migrations::v4::pre_migration::<Runtime, Bounties, _>(
            BOUNTIES_OLD_PREFIX,
            name,
        );
        Ok(())
    }

    #[cfg(feature = "try-runtime")]
    fn post_upgrade() -> Result<(), &'static str> {
        use frame_support::traits::PalletInfo;
        let name = <Runtime as frame_system::Config>::PalletInfo::name::<Bounties>()
            .expect("Bounties is part of runtime, so it has a name; qed");
        pallet_bounties::migrations::v4::post_migration::<Runtime, Bounties, _>(
            BOUNTIES_OLD_PREFIX,
            name,
        );
        Ok(())
    }
}

/// Migrate from 'PhragmenElection' to the new prefix 'Elections'
pub struct ElectionsPrefixMigration;
impl frame_support::traits::OnRuntimeUpgrade for ElectionsPrefixMigration {
    fn on_runtime_upgrade() -> frame_support::weights::Weight {
        use frame_support::traits::PalletInfo;
        let name = <Runtime as frame_system::Config>::PalletInfo::name::<Elections>()
            .expect("Elections is part of runtime, so it has a name; qed");
        pallet_elections_phragmen::migrations::v4::migrate::<Runtime, _>(name)
    }
}

// migration 0.9.16
// Migration for scheduler pallet to move from a plain Call to a CallOrHash.
pub struct SchedulerMigrationV3;

impl OnRuntimeUpgrade for SchedulerMigrationV3 {
    fn on_runtime_upgrade() -> frame_support::weights::Weight {
        Scheduler::migrate_v1_to_v3()
    }

    #[cfg(feature = "try-runtime")]
    fn pre_upgrade() -> Result<(), &'static str> {
        Scheduler::migrate_v1_to_v3()
    }

    #[cfg(feature = "try-runtime")]
    fn post_upgrade() -> Result<(), &'static str> {
        Scheduler::migrate_v1_to_v3()
    }
}

pub struct EthereumMigration;

impl OnRuntimeUpgrade for EthereumMigration {
    fn on_runtime_upgrade() -> frame_support::weights::Weight {
        Ethereum::migrate_block_v0_to_v2()
    }

    #[cfg(feature = "try-runtime")]
    fn pre_upgrade() -> Result<(), &'static str> {
        Ethereum::pre_migrate_block_v2()
    }

    #[cfg(feature = "try-runtime")]
    fn post_upgrade() -> Result<(), &'static str> {
        Ethereum::post_migrate_block_v2()
    }
}

// migration 0.9.16
/// Migrate session-historical from `Session` to the new pallet prefix `Historical`
pub struct SessionHistoricalPalletPrefixMigration;

impl OnRuntimeUpgrade for SessionHistoricalPalletPrefixMigration {
    fn on_runtime_upgrade() -> frame_support::weights::Weight {
        pallet_session::migrations::v1::migrate::<Runtime, Historical>()
    }

    #[cfg(feature = "try-runtime")]
    fn pre_upgrade() -> Result<(), &'static str> {
        pallet_session::migrations::v1::pre_migrate::<Runtime, Historical>();
        Ok(())
    }

    #[cfg(feature = "try-runtime")]
    fn post_upgrade() -> Result<(), &'static str> {
        pallet_session::migrations::v1::post_migrate::<Runtime, Historical>();
        Ok(())
    }
}

// migrate vesting
pub struct VestingMigration;

impl OnRuntimeUpgrade for VestingMigration {
    fn on_runtime_upgrade() -> frame_support::weights::Weight {
        pallet_vesting::migrations::v1::migrate::<Runtime>()
    }

    #[cfg(feature = "try-runtime")]
    fn pre_upgrade() -> Result<(), &'static str> {
        pallet_vesting::migrations::v1::migrate::<Runtime>()
    }

    #[cfg(feature = "try-runtime")]
    fn post_upgrade() -> Result<(), &'static str> {
        pallet_vesting::migrations::v1::migrate::<Runtime>()
    }
}

use frame_support::{traits::OnRuntimeUpgrade, weights::Weight};
pub struct STKOnRuntimeUpgrades;
impl OnRuntimeUpgrade for STKOnRuntimeUpgrades {
    fn on_runtime_upgrade() -> Weight {
        let mut weight = 0;

        // 1. SystemToTripleRefCount
        frame_support::log::info!("🔍️ SystemToTripleRefCount start ⛏");
        weight += <SystemToTripleRefCount as OnRuntimeUpgrade>::on_runtime_upgrade();
        frame_support::log::info!("🚀 SystemToTripleRefCount end ⛓");

        // 2. BabeEpochConfigMigrations
        frame_support::log::info!("🔍️ BabeEpochConfigMigrations start ⛏");
        weight += <BabeEpochConfigMigrations as OnRuntimeUpgrade>::on_runtime_upgrade();
        frame_support::log::info!("🚀 BabeEpochConfigMigrations end ⛓");

        // 3. GrandpaStoragePrefixMigration
        frame_support::log::info!("🔍️ GrandpaStoragePrefixMigration start ⛏");
        frame_support::traits::StorageVersion::new(0).put::<Grandpa>();
        weight += <GrandpaStoragePrefixMigration as OnRuntimeUpgrade>::on_runtime_upgrade();
        frame_support::log::info!("🚀 GrandpaStoragePrefixMigration end ⛓");

        // 4. MigratePalletVersionToStorageVersion
        frame_support::log::info!("🔍️ MigratePalletVersionToStorageVersion start ⛏");
        weight += <MigratePalletVersionToStorageVersion as OnRuntimeUpgrade>::on_runtime_upgrade();
        frame_support::log::info!("🚀 MigratePalletVersionToStorageVersion end ⛓");

        // 5. MigratePalletStakingV5toV7
        frame_support::log::info!("🔍️ MigratePalletStakingV5toV7 start ⛏");
        weight += <MigratePalletStakingV5toV7 as OnRuntimeUpgrade>::on_runtime_upgrade();
        frame_support::log::info!("🚀 MigratePalletStakingV5toV7 end ⛓");

        // 6. StakingBagsListMigrationV8
        frame_support::log::info!("🔍️ StakingBagsListMigrationV8 start ⛏");
        weight += <StakingBagsListMigrationV8 as OnRuntimeUpgrade>::on_runtime_upgrade();
        frame_support::log::info!("🚀 CouncilStoragePrefixMigration end ⛓");

        // 7. CouncilStoragePrefixMigration
        frame_support::log::info!("🔍️ CouncilStoragePrefixMigration start ⛏");
        frame_support::traits::StorageVersion::new(0).put::<Council>();
        weight += <CouncilStoragePrefixMigration as OnRuntimeUpgrade>::on_runtime_upgrade();
        frame_support::log::info!("🚀 CouncilStoragePrefixMigration end ⛓");

        // 8. TechnicalCommitteeStoragePrefixMigration
        frame_support::log::info!("🔍️ TechnicalCommitteeStoragePrefixMigration start ⛏");
        frame_support::traits::StorageVersion::new(0).put::<TechnicalCommittee>();
        weight +=
            <TechnicalCommitteeStoragePrefixMigration as OnRuntimeUpgrade>::on_runtime_upgrade();
        frame_support::log::info!("🚀 TechnicalCommitteeStoragePrefixMigration end ⛓");

        // 9. TechnicalMembershipStoragePrefixMigration
        frame_support::log::info!("🔍️ TechnicalMembershipStoragePrefixMigration start ⛏");
        frame_support::traits::StorageVersion::new(0).put::<TechnicalMembership>();
        weight +=
            <TechnicalMembershipStoragePrefixMigration as OnRuntimeUpgrade>::on_runtime_upgrade();
        frame_support::log::info!("🚀 TechnicalMembershipStoragePrefixMigration end ⛓");

        // 10. CouncilStoragePrefixMigration
        frame_support::log::info!("🔍️ CouncilStoragePrefixMigration start ⛏");
        frame_support::traits::StorageVersion::new(0).put::<Council>();
        weight += <CouncilStoragePrefixMigration as OnRuntimeUpgrade>::on_runtime_upgrade();
        frame_support::log::info!("🚀 CouncilStoragePrefixMigration end ⛓");

        // 11. MigrateTipsPalletPrefix
        frame_support::log::info!("🔍️ MigrateTipsPalletPrefix start ⛏");
        frame_support::traits::StorageVersion::new(0).put::<Tips>();
        weight += <MigrateTipsPalletPrefix as OnRuntimeUpgrade>::on_runtime_upgrade();
        frame_support::log::info!("🚀 MigrateTipsPalletPrefix end ⛓");

        // 12. BountiesPrefixMigration
        frame_support::log::info!("🔍️ BountiesPrefixMigration start ⛏");
        frame_support::traits::StorageVersion::new(0).put::<Bounties>();
        weight += <BountiesPrefixMigration as OnRuntimeUpgrade>::on_runtime_upgrade();
        frame_support::log::info!("🚀 BountiesPrefixMigration end ⛓");

        // 13. ElectionsPrefixMigration
        frame_support::log::info!("🔍️ ElectionsPrefixMigration start ⛏");
        frame_support::traits::StorageVersion::new(3).put::<Elections>();
        weight += <ElectionsPrefixMigration as OnRuntimeUpgrade>::on_runtime_upgrade();
        frame_support::log::info!("🚀 ElectionsPrefixMigration end ⛓");

        // 14. SchedulerMigrationV3
        frame_support::log::info!("🔍️ SchedulerMigrationV3 start ⛏");
        frame_support::traits::StorageVersion::new(0).put::<Scheduler>();
        weight += <SchedulerMigrationV3 as OnRuntimeUpgrade>::on_runtime_upgrade();
        frame_support::log::info!("🚀 SchedulerMigrationV3 end ⛓");

        // 15. EthereumMigration
        frame_support::log::info!("🔍️ EthereumMigration start ⛏");
        frame_support::traits::StorageVersion::new(0).put::<Ethereum>();
        weight += <EthereumMigration as OnRuntimeUpgrade>::on_runtime_upgrade();
        frame_support::log::info!("🚀 EthereumMigration end ⛓");

        // 16. SessionHistoricalPalletPrefixMigration
        frame_support::log::info!("🔍️ SessionHistoricalPalletPrefixMigration start ⛏");
        frame_support::traits::StorageVersion::new(0).put::<Historical>();
        weight +=
            <SessionHistoricalPalletPrefixMigration as OnRuntimeUpgrade>::on_runtime_upgrade();
        frame_support::log::info!("🚀 SessionHistoricalPalletPrefixMigration end ⛓");

        // 17. VestingMigration
        frame_support::log::info!("🔍️ VestingMigration start ⛏");
        weight +=
            <VestingMigration as OnRuntimeUpgrade>::on_runtime_upgrade();
        frame_support::log::info!("🚀 VestingMigration end ⛓");

        weight
    }
}
