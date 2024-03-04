#![cfg_attr(not(feature = "std"), no_std)]
//! # Communities Pallet
//!
//! Part of the People Local Interactions Protocol, this pallet enables people
//! to unite and create local communities that share a common interest or
//! economic activity. In simpler terms, it can be considered a DAO Factory.
//!
//! - [`Call`]
//! - [`Config`]
//!
//! ## Overview
//!
//! The Communities pallet provides functionality for managing communities,
//! facilitating its participants to have governance over the community entity
//! (and its associated account) which can interect with other systems:
//!
//! - Community registration and removal.
//! - Validating a community challenge.
//! - Enrolling/removing members from a community.
//! - Promoting/demoting members within the community.
//! - Voting on proposals to enable community governance.
//!
//! ## Terminology
//!
//! - **Community:** An entity comprised of _members_ —each one defined by their
//!   [`AccountId`][1]— with a given _description_ who can vote on _proposals_
//!   and actively take decisions on behalf of it. Communities are given a
//!   _treasury account_ and can issue tokens. Communities can be challenged to
//!   decide if they should stay active.
//! - **Community Description:** A set of metadata used to identify a community
//!   distinctively. Typically, a name, a description and a URL.
//! - **Community Status:** A community can be either `awaiting`, `active`, or
//!   `frozen` depending on whether the community has proven via a challenge
//!   it's actively contributing to the network with infrastructure provisioning
//!   (i.e. a [collator][3] node) or by depositing funds.
//! - **Validity Challenge:** A proof that a community is actively contributing
//!   to the network. It's possible for a trusted origin to manually mark a
//!   community challenge as passed, effectively changing the status of the
//!   community to `active`.
//! - **Admin:** An [`AccountId`][1] registered into the community that is set
//!   as such. Can call [privileged functions](#privileged-functions) within the
//!   community.
//! - **Member:** An [`AccountId`][1] registered into the community as such. Can
//!   have a rank within it and vote in the community's polls.
//! - **Member Rank:** Members could have a rank within the community. This can
//!   determine a voting weight depending on the community's voting mechanism.
//! - **Proposal:** A poll with an optionally set duration that executes a
//!   [call][4] dispatch if approved when it's closed.
//! - **Treasury Account:** A keyless [`AccountId`][1] generated on behalf of
//!   the community. Can receive [payments][5], transfers, or payment [fees][6].
//!   It can transfer funds via a privileged call executed by the community
//!   _admin_ or a call dispatched from a proposal.
//! - **Voting Method:** Can be either rank weighed, member-counted, or
//!   asset-weighed and determines how the votes of proposals will be tallied.
//!
//! ## Lifecycle
//!
//! ```ignore
//! [       ] --> [Awaiting]              --> [Active]            --> [Frozen]      --> [Blocked]
//! apply_for     set_metadata                set_metadata            set_metadata      unblock
//!               fulfill_challenge           block                   block
//!               force_complete_challenge    add_member              thaw
//!                                           remove_member
//!                                           promote_member
//!                                           demote_member
//!                                           open_proposal
//!                                           vote_proposal
//!                                           close_proposal
//!                                           set_admin
//!                                           set_voting_mechanism
//!                                           freeze
//! ```
//!
//! ## Implementations
//!
//! > TODO: Define which traits we are defining/implementing.
//!
//! ## Interface
//!
//! ### Permissionless Functions
//!
//! - [`apply_for`][c00]: Registers an appliation as a new community, taking an
//!   [existential deposit][8] used to create the community account.
//!
//! ### Permissioned Functions
//!
//! Calling these functions requires being a member of the community.
//!
//! - `fulfill_challenge`: Submit the challenge proof to validate the
//!   contribution status of the community.
//! - [`add_member`][c02]: Enroll an account as a community member. In theory,
//!   any community member should be able to add a member. However, this can be
//!   changed to ensure it is a privileged function.
//! - `open_proposal`: Creates a proposal to be voted by the community. At
//! this point, there can only be a single proposal at a time.
//! - `vote_proposal`: Adds a vote into a community proposal.
//!
//! ### Privileged Functions
//!
//! These functions can be called either by the community _admin_ or
//! dispatched through an approved proposal. !
//! - [`set_metadata`][c01]: Sets some [`CommunityMetadata`][t01] to describe
//!   the
//! community.
//! - [`remove_member`][c03]: Removes an account as a community member. While
//!   enrolling a member into the community can be an action taken by any
//!   member, the decision to remove a member should not be taken arbitrarily by
//!   any community member. Also, it shouldn't be possible to arbitrarily remove
//!   the community admin, as some privileged calls would be impossible execute
//!   thereafter.
//! - `promote_member`: Increases the rank of a member in the community. ! -
//!   `demote_member`: Decreases the rank of a member in the community.
//! - `issue_token`: Creates a token that is either governance (only one per
//!   community allowed) or economic. While the first economic token is
//!   _"free"_," further ones would be subject to network-wide referenda.
//! - `close_proposal`: Forcefully closes a proposal, dispatching the call when
//!   approved.
//! - `set_sufficient_asset`: Marks an [asset][7] issued by the community as
//!   sufficient. Only one asset at a time can be marked as such.
//! - `set_admin`: Sets an [`AccountId`][1] of the _admin_ of the community.
//!   Ensures that the specified account is a member of the community.
//! - `set_voting_mechanism`: Transfers funds from the treasury account to a
//!   beneficiary.
//!
//! ### Root Functions
//!
//! - `force_complete_challenge`: Marks a challenge as passed. This can lead to
//!   the activation of a community if all challenges are passed.
//! - `force_increase_economic_token_limit`: Increases the amount of economic
//!   tokens a community can issue.
//!
//! ### Public Functions
//!
//! - [`community`][g00]: Stores the basic information of the community. If a
//!   value exists for a specified [`ComumunityId`][t00], this means a community
//!   exists.
//! - [`metadata`][g01]: Stores the metadata regarding a community.
//! - [`membership`][g02]: Stores the information of a community (specified by
//!   its [`CommunityId`][t00]) member (specified by it's [`AccountId`][1]).
//! - [`members_count`][g03]: Stores the count of community members. This
//!   simplifies the process of keeping track of members' count.
//!
//! <!-- References -->
//! [1]: `frame_system::Config::AccountId`
//! [2]: https://h3geo.org/docs/highlights/indexing
//! [3]: https://docs.substrate.io/reference/glossary/#collator
//! [4]: https://docs.substrate.io/reference/glossary/#call
//! [5]: https://github.com/virto-network/virto-node/tree/master/pallets/payments
//! [6]: https://github.com/virto-network/virto-node/pull/282
//! [7]: https://paritytech.github.io/substrate/master/pallet_assets/index.html#terminology
//! [8]: https://docs.substrate.io/reference/glossary/#existential-deposit
//!
//! [t00]: `Config::CommunityId`
//! [t01]: `types::CommunityMetadata`
//!
//! [c00]: `crate::Pallet::apply`
//! [c01]: `crate::Pallet::set_metadata`
//! [c02]: `crate::Pallet::add_member`
//! [c03]: `crate::Pallet::remove_member`
//!
//! [g00]: `crate::Pallet::community`
//! [g01]: `crate::Pallet::metadata`
//! [g02]: `crate::Pallet::membership`
//! [g03]: `crate::Pallet::members_count`
pub use pallet::*;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
#[cfg(test)]
mod tests;

mod functions;
mod impls;

pub mod types;
pub mod weights;
pub use weights::*;
pub mod origin;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{
		pallet_prelude::*,
		traits::{
			fungible, fungibles,
			membership::{self, Inspect, Membership, Mutate, WithRank},
			EnsureOrigin, Polling,
		},
		Blake2_128Concat, Parameter,
	};
	use frame_system::pallet_prelude::{OriginFor, *};
	use sp_runtime::traits::StaticLookup;
	use types::{CommunityIdOf, MembershipIdOf, PollIndexOf, *};

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it
	/// depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// This type represents an unique ID for the community
		type CommunityId: Parameter + MaxEncodedLen + Copy + From<MembershipIdOf<Self>>;

		/// The type holding relevant information of a membership
		type Membership: Membership + membership::WithRank;

		/// Means to manage memberships of a community
		type MemberMgmt: membership::Inspect<Self::AccountId, MembershipInfo = Self::Membership, MembershipId = MembershipIdOf<Self>>
			+ membership::Mutate<Self::AccountId>;

		/// Origin authorized to manage memeberships of an active community
		type MemberMgmtOrigin: EnsureOrigin<Self::RuntimeOrigin, Success = Self::CommunityId>;

		/// Origin authorized to manage memeberships of an active community
		type CommunityMgmtOrigin: EnsureOrigin<Self::RuntimeOrigin>;

		type Polls: Polling<Tally<Self>, Votes = VoteWeight, Moment = BlockNumberFor<Self>>;

		/// The asset used for governance
		type Assets: fungibles::Inspect<Self::AccountId> + fungibles::hold::Mutate<Self::AccountId>;

		/// Type represents interactions between fungibles (i.e. assets)
		type Balances: fungible::Inspect<Self::AccountId>
			+ fungible::Mutate<Self::AccountId>
			+ fungible::hold::Mutate<Self::AccountId, Reason = Self::RuntimeHoldReason>;

		/// The overarching hold reason.
		type RuntimeHoldReason: From<HoldReason>;

		type VoteHoldReason: Get<<Self::Assets as fungibles::hold::Inspect<Self::AccountId>>::Reason>;

		/// Because this pallet emits events, it depends on the runtime's
		/// definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Type representing the weight of this pallet
		type WeightInfo: WeightInfo;

		/// The pallet id used for deriving sovereign account IDs.
		#[pallet::constant]
		type PalletId: Get<frame_support::PalletId>;
	}

	/// The origin of the pallet
	#[pallet::origin]
	pub type Origin<T> = origin::RawOrigin<T>;

	/// A reason for the pallet communities placing a hold on funds.
	#[pallet::composite_enum]
	pub enum HoldReason {
		// A vote has been casted on a poll
		VoteCasted(u32),
	}

	/// Stores the basic information of the community. If a value exists for a
	/// specified [`ComumunityId`][`Config::CommunityId`], this means a
	/// community exists.
	#[pallet::storage]
	#[pallet::getter(fn community)]
	pub(super) type Info<T> = StorageMap<_, Blake2_128Concat, CommunityIdOf<T>, CommunityInfo>;

	#[pallet::storage]
	pub(super) type CommunityIdFor<T> = StorageMap<_, Blake2_128Concat, PalletsOriginOf<T>, CommunityIdOf<T>>;

	/// Stores the metadata regarding a community.
	#[pallet::storage]
	#[pallet::getter(fn metadata)]
	pub(super) type Metadata<T: Config> =
		StorageMap<_, Blake2_128Concat, CommunityIdOf<T>, CommunityMetadata, ValueQuery>;

	/// Stores the count of members, managed by add_member and removefly_member
	#[pallet::storage]
	pub(super) type CommunityMembersCount<T> = StorageMap<_, Blake2_128Concat, CommunityIdOf<T>, u32, ValueQuery>;

	/// Stores the decision method for a community
	#[pallet::storage]
	pub(super) type CommunityDecisionMethod<T> =
		StorageMap<_, Blake2_128Concat, CommunityIdOf<T>, DecisionMethodFor<T>, ValueQuery>;

	/// Stores the list of votes for a community.
	#[pallet::storage]
	#[pallet::getter(fn community_vote_of)]
	pub(super) type CommunityVotes<T> =
		StorageDoubleMap<_, Blake2_128Concat, AccountIdOf<T>, Blake2_128Concat, PollIndexOf<T>, VoteOf<T>>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A [`Commmunity`][`types::Community`] has been created.
		CommunityCreated {
			id: T::CommunityId,
			origin: PalletsOriginOf<T>,
		},
		/// Some [`CommmuniMetadata`][`types::CommunityMetadata`] has been set
		/// for a community.
		MetadataSet {
			id: T::CommunityId,
			name: Option<ConstSizedField<64>>,
			description: Option<ConstSizedField<256>>,
			main_url: Option<ConstSizedField<256>>,
		},
		DecisionMethodSet {
			id: T::CommunityId,
		},
		MemberAdded {
			who: AccountIdOf<T>,
			membership_id: MembershipIdOf<T>,
		},
		MemberRemoved {
			who: AccountIdOf<T>,
			membership_id: MembershipIdOf<T>,
		},
		MembershipRankUpdated {
			membership_id: MembershipIdOf<T>,
			rank: membership::GenericRank,
		},
	}

	// Errors inform users that something worked or went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// The community doesn't exist in storage, nor they have members.
		CommunityDoesNotExist,
		/// A community with the same [`CommunityId`][`Config::CommunityId`]
		/// already exists, therefore cannot be applied for.
		CommunityAlreadyExists,
		/// The community can't introduce new members at the moment
		CommunityAtCapacity,
		/// The specified [`AccountId`][`frame_system::Config::AccountId`] is
		/// not a member of the community
		NotAMember,
		/// The indicated index corresponds to a poll that is not ongoing
		NotOngoing,
		/// The track for the poll voted for does not correspond to the community ID
		InvalidTrack,
		/// The vote type does not correspond with the community's selected
		/// [`DecisionMethod`][`origin::DecisionMethod`]
		InvalidVoteType,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke
	// state changes. These functions materialize as "extrinsics", which are often
	// compared to transactions. Dispatchable functions must be annotated with a
	// weight and must return a DispatchResult.
	#[pallet::call(weight(<T as Config>::WeightInfo))]
	impl<T: Config> Pallet<T> {
		/// Creates a new community managed by the given origin
		#[pallet::call_index(0)]
		pub fn create(
			origin: OriginFor<T>,
			admin_origin: PalletsOriginOf<T>,
			community_id: T::CommunityId,
		) -> DispatchResult {
			T::CommunityMgmtOrigin::ensure_origin(origin)?;

			Self::do_register_community(&admin_origin, &community_id)?;
			Self::deposit_event(Event::CommunityCreated {
				id: community_id,
				origin: admin_origin,
			});
			Ok(())
		}

		/// Sets some [`CommunityMetadata`][11] to describe the
		/// community.
		///
		/// [11]: `types::CommunityMetadata`
		#[pallet::call_index(1)]
		pub fn set_metadata(
			origin: OriginFor<T>,
			community_id: T::CommunityId,
			name: Option<ConstSizedField<64>>,
			description: Option<ConstSizedField<256>>,
			url: Option<ConstSizedField<256>>,
		) -> DispatchResult {
			T::CommunityMgmtOrigin::ensure_origin(origin)?;

			Self::do_set_metadata(&community_id, &name, &description, &url);
			Self::deposit_event(Event::MetadataSet {
				id: community_id,
				name,
				description,
				main_url: url,
			});

			Ok(())
		}

		// === Memberships management ===

		/// Enroll an account as a community member that receives a membership
		/// from the available pool of memberships of the community.
		#[pallet::call_index(2)]
		pub fn add_member(origin: OriginFor<T>, who: AccountIdLookupOf<T>) -> DispatchResult {
			let community_id = T::MemberMgmtOrigin::ensure_origin(origin)?;
			let who = T::Lookup::lookup(who)?;
			let account = Self::community_account(&community_id);
			// assume the community has memberships to give out to the new member
			let membership_id = T::MemberMgmt::account_memberships(&account)
				.next()
				.ok_or(Error::<T>::CommunityAtCapacity)?;
			T::MemberMgmt::update(
				membership_id.clone(),
				T::Membership::new(membership_id.clone()),
				Some(who.clone()),
			)?;

			Self::deposit_event(Event::MemberAdded { who, membership_id });
			Ok(())
		}

		/// Removes an account as a community member. While
		/// enrolling a member into the community can be an action taken by any
		/// member, the decision to remove a member should not be taken
		/// arbitrarily by any community member. Also, it shouldn't be possible
		/// to arbitrarily remove the community admin, as some privileged calls
		/// would be impossible to execute thereafter.
		#[pallet::call_index(3)]
		pub fn remove_member(
			origin: OriginFor<T>,
			who: AccountIdLookupOf<T>,
			membership_id: MembershipIdOf<T>,
		) -> DispatchResult {
			let community_id = T::MemberMgmtOrigin::ensure_origin(origin)?;
			let who = T::Lookup::lookup(who)?;
			let info = T::MemberMgmt::get_membership(membership_id.clone(), &who).ok_or(Error::<T>::NotAMember)?;
			ensure!(
				CommunityIdOf::<T>::from(info.id()) == community_id,
				Error::<T>::CommunityDoesNotExist
			);

			let account = Self::community_account(&community_id);
			// Move the membership back to the community resetting any previous stored info
			T::MemberMgmt::update(
				membership_id.clone(),
				T::Membership::new(membership_id.clone()),
				Some(account),
			)?;

			Self::deposit_event(Event::MemberRemoved { who, membership_id });
			Ok(())
		}

		/// Increases the rank of a member in the community
		#[pallet::call_index(5)]
		pub fn promote_member(
			origin: OriginFor<T>,
			who: AccountIdLookupOf<T>,
			membership_id: MembershipIdOf<T>,
		) -> DispatchResult {
			let _community_id = T::MemberMgmtOrigin::ensure_origin(origin)?;
			let who = T::Lookup::lookup(who)?;

			let mut m = T::MemberMgmt::get_membership(membership_id.clone(), &who).ok_or(Error::<T>::NotAMember)?;
			let rank = m.rank();
			m.set_rank(rank.promote_by(1.try_into().expect("can promote by 1")));
			T::MemberMgmt::update(membership_id.clone(), m, None)?;

			Self::deposit_event(Event::MembershipRankUpdated { membership_id, rank });
			Ok(())
		}

		/// Decreases the rank of a member in the community
		#[pallet::call_index(6)]
		pub fn demote_member(
			origin: OriginFor<T>,
			who: AccountIdLookupOf<T>,
			membership_id: MembershipIdOf<T>,
		) -> DispatchResult {
			let _community_id = T::MemberMgmtOrigin::ensure_origin(origin)?;
			let who = T::Lookup::lookup(who)?;

			let mut m = T::MemberMgmt::get_membership(membership_id.clone(), &who).ok_or(Error::<T>::NotAMember)?;
			let rank = m.rank();
			m.set_rank(rank.demote_by(1.try_into().expect("can promote by 1")));
			T::MemberMgmt::update(membership_id.clone(), m, None)?;

			Self::deposit_event(Event::MembershipRankUpdated { membership_id, rank });
			Ok(())
		}

		// === Governance ===

		///
		#[pallet::call_index(7)]
		pub fn set_decision_method(
			origin: OriginFor<T>,
			community_id: T::CommunityId,
			decision_method: DecisionMethodFor<T>,
		) -> DispatchResult {
			T::CommunityMgmtOrigin::ensure_origin(origin)?;
			CommunityDecisionMethod::<T>::set(community_id, decision_method);

			Self::deposit_event(Event::DecisionMethodSet {
				id: community_id.clone(),
			});
			Ok(())
		}

		///
		#[pallet::call_index(4)]
		pub fn vote(
			origin: OriginFor<T>,
			membership_id: MembershipIdOf<T>,
			#[pallet::compact] poll_index: PollIndexOf<T>,
			vote: VoteOf<T>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let _info = T::MemberMgmt::get_membership(membership_id.clone(), &who).ok_or(Error::<T>::NotAMember)?;
			let community_id = CommunityIdOf::<T>::from(membership_id);

			Self::do_vote(&who, &community_id, poll_index, vote)
		}
	}
}
