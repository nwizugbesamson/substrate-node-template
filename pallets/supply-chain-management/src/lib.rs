//! ## Overview
//! Run `cargo doc --package pallet-template --open` to view this pallet's documentation.

// We make sure this pallet uses `no_std` for compiling to Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;
use sp_std::prelude::*;


#[cfg(test)]
mod mock;

//  https://docs.substrate.io/test/unit-testing/
#[cfg(test)]
mod tests;

// https://docs.substrate.io/test/benchmark/
#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
	use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;
	use scale_info::TypeInfo;
    use sp_std::collections::btree_map::BTreeMap;
    use sp_std::marker::PhantomData;

	
	// #[pallet::generate_store(pub(super) trait Store)]
	#[pallet::pallet]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	
	#[pallet::config]
	pub trait Config: frame_system::Config + TypeInfo {
		//  + Sized
		/// The overarching runtime event type.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// A type representing the weights required by the dispatchables of this pallet.
		type WeightInfo: WeightInfo;

		type UniqueId: Parameter + Member + MaybeSerializeDeserialize + AsRef<[u8]> + Ord + Default;
        // type RawMaterialId: Parameter + Member + AtLeast32BitUnsigned + Default + Copy;
        // type RawMaterialUnitId: Parameter + Member + AtLeast32BitUnsigned + Default + Copy;
		
	}

    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
	pub struct Actor<T: Config> {
		pub account: T::AccountId,
		pub company_name: Vec<u8>,
        pub ceritification_authority: Vec<u8>,
        pub ceritification_number: Vec<u8>,
	}

    impl <T: Config> Actor<T> {
        fn new(
            account: T::AccountId, 
            company_name: Vec<u8>, 
            ceritification_authority: Vec<u8>, 
            ceritification_number: Vec<u8>) -> Self {
            Self {
                account,
                company_name,
                ceritification_authority,
                ceritification_number,
            }
        }

        fn get_account(&self) -> T::AccountId {
            self.account.clone()
        }

        fn owns_account(&self, account: T::AccountId) -> bool {
            self.account == account
        }

        fn get_company_name(&self) -> Vec<u8> {
            self.company_name.clone()
        }

        fn get_ceritification_authority(&self) -> Vec<u8> {
            self.ceritification_authority.clone()
        }

        fn get_ceritification_number(&self) -> Vec<u8> {
            self.ceritification_number.clone()
        }
    }

    pub trait HasActor<T: Config> {
        fn get_account(&self) -> T::AccountId{
            self.get_actor().get_account()
        }

        fn owns_account(&self, account: T::AccountId) -> bool {
            self.get_actor().owns_account(account)
        }

        fn get_company_name(&self) -> Vec<u8> {
            self.get_actor().get_company_name()
        }

        fn get_ceritification_authority(&self) -> Vec<u8> {
            self.get_actor().get_ceritification_authority()
        }

        fn get_ceritification_number(&self) -> Vec<u8> {
            self.get_actor().get_ceritification_number()
        }

        fn get_actor(&self) -> Actor<T>;
    }


    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
    pub struct OwnsRawMaterials<T: Config> {
        pub raw_materials: BTreeMap<T::UniqueId, Vec<T::UniqueId>>,
    }

    impl <T: Config> OwnsRawMaterials<T> {
        
        pub fn new() -> Self {
            Self {
                raw_materials: BTreeMap::new(),
            }
        }

        pub fn get_raw_materials(&self) -> BTreeMap<T::UniqueId, Vec<T::UniqueId>> {
            self.raw_materials.clone()
        }

        pub fn add_units_to_raw_material(
            &mut self,
            raw_material_id: T::UniqueId,
            new_units: Vec<T::UniqueId>,
        ) {
            // Check if the raw material already exists in the BTreeMap
            if let Some(existing_units) = self.raw_materials.get_mut(&raw_material_id) {
                // Append new units to the existing units
                existing_units.extend(new_units);
            } else {
                // If raw material does not exist, insert it as a new entry
                self.raw_materials.insert(raw_material_id, new_units);
            }
        }
    
        pub fn get_units(&self, raw_material_id: &T::UniqueId) -> Option<&Vec<T::UniqueId>> {
            self.raw_materials.get(raw_material_id)
        }

        pub fn has_material_in_inventory(&self, raw_material_id: &T::UniqueId, quantity: u32) -> bool {
            self.raw_materials.contains_key(raw_material_id) && 
            self.raw_materials.get(raw_material_id).unwrap().len() >= quantity as usize
        }

        pub fn get_unit_from_material(&self, raw_material_id: &T::UniqueId) -> Option<T::UniqueId> {
            if let Some(units) = self.raw_materials.get(raw_material_id) {
                units.last().cloned()
            } else {
                None
            }
        }

        pub fn remove_unit_from_material(
            &mut self, 
            raw_material_id: &T::UniqueId,
            raw_material_unit_id: &T::UniqueId
        ) -> bool {
            if let Some(units) = self.raw_materials.get_mut(raw_material_id) {
                if units.contains(raw_material_unit_id) {
                    units.retain(|unit| unit != raw_material_unit_id);
                    true
                } else {
                    false
                }
            } else {
                false
            }
        }
        
    }



    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
    pub struct HasOrders<T: Config> {
        pub orders: Vec<T::UniqueId>,
    }

    impl <T: Config> HasOrders<T> {
        pub fn new() -> Self {
            Self {
                orders: Vec::new(),
            }
        }

        pub fn get_orders(&self) -> Vec<T::UniqueId> {
            self.orders.clone()
        }

        pub fn add_order(&mut self, order_id: T::UniqueId) {
            self.orders.push(order_id);
        }

        pub fn has_order(&self, order_id: &T::UniqueId) -> bool {
            self.orders.contains(order_id)
        }
    }

    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
    pub struct Supplier<T: Config> {
        pub actor: Actor<T>,
        pub material_manager: OwnsRawMaterials<T>,
        pub order_manager: HasOrders<T>,
    }

    impl <T: Config> HasActor<T> for Supplier<T> {
        fn get_actor(&self) -> Actor<T> {
            self.actor.clone()
        }
    }

    impl <T: Config> Supplier<T> {
        pub fn new(
            account_id: T::AccountId, company_name: Vec<u8>, 
            ceritification_authority: Vec<u8>, ceritification_number: Vec<u8>
        ) -> Self {
            Self {
                actor: Actor::new(
                    account_id, company_name, 
                    ceritification_authority, ceritification_number
                ),
                material_manager: OwnsRawMaterials::new(),
                order_manager: HasOrders::new(),
            }
        }
        
    }

    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
    pub struct Manufacturer<T: Config> {
        pub actor: Actor<T>,
        pub material_manager: OwnsRawMaterials<T>,
        pub order_manager: HasOrders<T>,
    }

    impl <T: Config> HasActor<T> for Manufacturer<T> {
        fn get_actor(&self) -> Actor<T> {
            self.actor.clone()
        }
        
    }
    impl <T: Config> Manufacturer<T> {
        pub fn new(
            account_id: T::AccountId, company_name: Vec<u8>, 
            ceritification_authority: Vec<u8>, ceritification_number: Vec<u8>
        ) -> Self {
            Self {
                actor: Actor::new(
                    account_id, company_name, 
                    ceritification_authority, ceritification_number
                ),
                material_manager: OwnsRawMaterials::new(),
                order_manager: HasOrders::new(),
            }
        }
    }

    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
    pub struct ShippingAgent<T: Config> {
        pub actor: Actor<T>,
        pub manifests: Vec<T::UniqueId>,
        pub pickup_locations: Vec<(Vec<u8>, Vec<u8>)>,
        pub delivery_locations: Vec<(Vec<u8>, Vec<u8>)>,
    }
    

    impl <T: Config> HasActor<T> for ShippingAgent<T> {
        fn get_actor(&self) -> Actor<T> {
            self.actor.clone()
        }
    }

    impl <T: Config> ShippingAgent<T> {
        pub fn new(
            account_id: T::AccountId, company_name: Vec<u8>, 
            ceritification_authority: Vec<u8>, ceritification_number: Vec<u8>
        ) -> Self {
            Self {
                actor: Actor::new(
                    account_id, company_name, 
                    ceritification_authority, ceritification_number
                ),
                manifests: Vec::new(),
                pickup_locations: Vec::new(),
                delivery_locations: Vec::new(),
            }
        }

        pub fn get_pickup_locations(&self) -> Vec<(Vec<u8>, Vec<u8>)> {
            self.pickup_locations.clone()
        }

        pub fn get_delivery_locations(&self) -> Vec<(Vec<u8>, Vec<u8>)> {
            self.delivery_locations.clone()
        }

        pub fn add_manifest(&mut self, manifest_id: T::UniqueId) {
            self.manifests.push(manifest_id);
        }

        pub fn add_pickup_locations(&mut self, pickup_locations: Vec<(Vec<u8>, Vec<u8>)>) {
            self.pickup_locations.extend(pickup_locations);
        }

        pub fn add_delivery_locations(&mut self, delivery_locations: Vec<(Vec<u8>, Vec<u8>)>) {
            self.delivery_locations.extend(delivery_locations);
        }

        pub fn get_manifests(&self) -> Vec<T::UniqueId> {
            self.manifests.clone()
        }

        pub fn remove_pickup_location(&mut self, pickup_location: (Vec<u8>, Vec<u8>)) {
            self.pickup_locations.retain(|location| location != &pickup_location);
        }

        pub fn remove_delivery_location(&mut self, delivery_location: (Vec<u8>, Vec<u8>)) {
            self.delivery_locations.retain(|location| location != &delivery_location);
        }

        pub fn can_pickup_from(&self, pickup_location: &(Vec<u8>, Vec<u8>)) -> bool {
            self.pickup_locations.contains(pickup_location)
        }

        pub fn can_deliver_to(&self, delivery_location: &(Vec<u8>, Vec<u8>)) -> bool {
            self.delivery_locations.contains(delivery_location)
        }

        pub fn can_ship(&self, pickup_location: &(Vec<u8>, Vec<u8>), delivery_location: &(Vec<u8>, Vec<u8>)) -> bool {
            self.can_pickup_from(pickup_location) && self.can_deliver_to(delivery_location)
        }
    }

    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
    pub enum AccessLevel {
        Owner,
        Custodian,
        AllUsers,
    }

    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
    pub struct ProprietaryData {
        index: Option<u32>,
        start_time: u32,
        end_time: Option<u32>,
        visibility: AccessLevel,
    }

    impl ProprietaryData {
        pub fn new( start_time: u32) -> Self {
            Self {
                index: None,
                start_time,
                end_time: None,
                visibility: AccessLevel::AllUsers,
            }
        }

        pub fn set_index(&mut self, index: u32) {
            self.index = Some(index);
        }

        pub fn get_index(&self) -> u32 {
            self.index.unwrap()
        }

        pub fn has_index(&self, index: u32) -> bool {
            self.index == Some(index)
        }

        pub fn get_start_time(&self) -> u32 {
            self.start_time
        }

        pub fn get_end_time(&self) -> Option<u32> {
            self.end_time
        }

        pub fn get_visibility(&self) -> AccessLevel {
            self.visibility.clone()
        }

        // Function to set the end_time
        pub fn expire(&mut self, end_time: u32) {
            self.end_time = Some(end_time);
        }

        // Function to check if a specific actor can view the data
        pub fn can_view(&self, actor_type: AccessLevel) -> bool {
            match self.visibility {
                AccessLevel::Owner => actor_type == AccessLevel::Owner,
                AccessLevel::Custodian => actor_type == AccessLevel::Owner || actor_type == AccessLevel::Custodian,
                AccessLevel::AllUsers => true,
            }
        }

        // Function to check if the end_time is set or not
        pub fn is_active(&self) -> bool {
            self.end_time.is_none()  // Returns true if end_time is None (not set)
        }

        pub fn set_visibility(&mut self, visibility: AccessLevel) {
            self.visibility = visibility;
        }
        
    }

    #[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug, TypeInfo)]
    pub enum ProprietaryDataResult<T: Config> {
        Owner(T::AccountId),
        Custodian(T::AccountId),
        Location((Vec<u8>, Vec<u8>)),
        Restricted(Vec<u8>),
        NotFound(Vec<u8>),
    }

    pub trait HasProprietaryData<T: Config> {
    
        // Method to be implemented by structs
        fn get_data(&self, actor_type: AccessLevel
        ) -> (ProprietaryDataResult<T>, Option<u32>, Option<u32>, Option<u32>);

        fn get_data_result(&self, data: ProprietaryDataResult<T>
        ) -> (ProprietaryDataResult<T>, Option<u32>, Option<u32>, Option<u32>) {
            (
                data,
                Some(self.get_index()),
                Some(self.get_data_manager().get_start_time()),
                self.get_data_manager().get_end_time(),
            )
        }

        fn get_restricted_data(&self
        ) -> (ProprietaryDataResult<T>, Option<u32>, Option<u32>, Option<u32>) {
            (
                ProprietaryDataResult::Restricted(b"Data is restricted".to_vec()),
                Some(self.get_index()),
                None,
                None,
            )
        }

        fn get_not_found_data(&self
        ) -> (ProprietaryDataResult<T>, Option<u32>, Option<u32>, Option<u32>) {
            (
                ProprietaryDataResult::NotFound(b"Data not found".to_vec()),
                None,
                None,
                None,
            )
        }
    
        // Default implementation for checking if data is active
        fn is_active(&self) -> bool {
            self.get_data_manager().is_active()
        }
    
        // Default implementation for expiring data
        fn expire(&mut self, end_time: u32) {
            self.get_data_manager_mut().expire(end_time);
        }

        fn has_index(&self, index: u32) -> bool {
            self.get_data_manager().has_index(index)
        }

        fn get_index(&self) -> u32 {
            self.get_data_manager().get_index()
        }

        fn set_index(&mut self, index: u32) {
            self.get_data_manager_mut().set_index(index);
        }

        fn set_visibility(&mut self, visibility: AccessLevel) {
            self.get_data_manager_mut().set_visibility(visibility);
        }
    
        // Helper method to get a reference to the proprietary data (to be implemented by structs)
        fn get_data_manager(&self) -> &ProprietaryData;
    
        // Helper method to get a mutable reference to the proprietary data (to be implemented by structs)
        fn get_data_manager_mut(&mut self) -> &mut ProprietaryData;
    }

    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
    pub struct Owner<T: Config> {
        pub owner: T::AccountId,
        pub data: ProprietaryData,
    }

    

    impl <T: Config> Owner<T> {
        pub fn new(owner: T::AccountId, start_time: u32) -> Self {
            Self {
                owner,
                data: ProprietaryData::new( start_time),
            }
        }
        
    }

    impl <T: Config> HasProprietaryData<T> for Owner<T> {
        fn get_data(&self, actor_type: AccessLevel
        )-> (ProprietaryDataResult<T>, Option<u32>, Option<u32>, Option<u32>)
        {
            if self.data.can_view(actor_type) {
                self.get_data_result(ProprietaryDataResult::Owner(self.owner.clone()))
            } else {
                 self.get_restricted_data() 
            }
        }

        fn get_data_manager(&self) -> &ProprietaryData {
            &self.data
        }

        fn get_data_manager_mut(&mut self) -> &mut ProprietaryData {
            &mut self.data
        }
    }


    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
    pub struct Custodian<T: Config> {
        pub custodian: T::AccountId,
        pub data: ProprietaryData,
    }

    impl <T: Config> Custodian<T> {
        pub fn new(custodian: T::AccountId, start_time: u32) -> Self {
            Self {
                custodian,
                data: ProprietaryData::new( start_time),
            }
        }
    }

    impl <T: Config> HasProprietaryData<T> for Custodian<T> {
        fn get_data(&self, actor_type: AccessLevel
        ) -> (ProprietaryDataResult<T>, Option<u32>, Option<u32>, Option<u32>) 
        {
            if self.data.can_view(actor_type) {
                self.get_data_result(ProprietaryDataResult::Custodian(self.custodian.clone()))
            } else {
                self.get_restricted_data()
            }
        }

        
        fn get_data_manager(&self) -> &ProprietaryData {
            &self.data
        }

        fn get_data_manager_mut(&mut self) -> &mut ProprietaryData {
            &mut self.data
        }
    }

    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
    pub struct Location<T: Config> {
        pub longitude: Vec<u8>,
        pub latitude: Vec<u8>,
        pub _marker: Option<PhantomData<T>>,
        pub data: ProprietaryData,
        
    }

    impl <T: Config> Location<T> {
        pub fn new(longitude: Vec<u8>, latitude: Vec<u8>,  start_time: u32) -> Self {
            Self {
                longitude,
                latitude,
                _marker: None,
                data: ProprietaryData::new(start_time),
            }
        }
    }

    impl <T: Config> HasProprietaryData<T> for Location<T> {
        fn get_data(&self, actor_type: AccessLevel
        ) -> (ProprietaryDataResult<T>, Option<u32>, Option<u32>, Option<u32>) 
        {
            if self.data.can_view(actor_type) {
                self.get_data_result(
                    ProprietaryDataResult::Location((self.longitude.clone(), self.latitude.clone()))
                )
            } else {
                self.get_restricted_data()
            }
        }

        fn get_data_manager(&self) -> &ProprietaryData {
            &self.data
        }

        fn get_data_manager_mut(&mut self) -> &mut ProprietaryData {
            &mut self.data
        }
    }
    
    
    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
    pub struct RawMaterial<T: Config> {
        pub material_id: T::UniqueId,
        pub name: Vec<u8>,
        pub price: u32,
        pub units: Vec<T::UniqueId>,
    }

    impl <T: Config> RawMaterial<T> {
        pub fn new(material_id: T::UniqueId, name: Vec<u8>, price: u32) -> Self {
            Self {
                material_id,
                name,
                price,
                units: Vec::new(),
            }
        }

        pub fn get_material_id(&self) -> T::UniqueId {
            self.material_id.clone()
        }

        pub fn get_name(&self) -> Vec<u8> {
            self.name.clone()
        }

        pub fn get_price(&self) -> u32 {
            self.price
        }

        pub fn get_units(&self) -> Vec<T::UniqueId> {
            self.units.clone()
        }

        pub fn add_units(&mut self, new_units: Vec<T::UniqueId>) {
            self.units.extend(new_units);
        }

        pub fn remove_unit(&mut self) -> Option<T::UniqueId> {
            self.units.pop()
        }
    }

    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
    pub struct RawMaterialShipping<T: Config> {
        pub pickup_location: (Vec<u8>, Vec<u8>),
        pub shipping_agents: Vec<T::AccountId>,
    }

    impl <T: Config> RawMaterialShipping<T> {
        pub fn new(
            pickup_location: (Vec<u8>, Vec<u8>), shipping_agents: Vec<T::AccountId>
        ) -> Self {
            Self {
                pickup_location,
                shipping_agents,
            }
        }

        pub fn add_shipping_agent(&mut self, shipping_agent: T::AccountId) {
            self.shipping_agents.push(shipping_agent);
        }

        pub fn get_shipping_agents(&self) -> Vec<T::AccountId> {
            self.shipping_agents.clone()
        }

        pub fn has_shipping_agent(&self, shipping_agent: &T::AccountId) -> bool {
            self.shipping_agents.contains(&shipping_agent)
        }

        pub fn get_pickup_location(&self) -> (Vec<u8>, Vec<u8>) {
            self.pickup_location.clone()
        }
    }

    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
    pub enum ProprietaryDataInput<T: Config> {
        Owner(Option<Owner<T>>),
        Custodian(Option<Custodian<T>>),
        Location(Option<Location<T>>),
        Invalid(Option<Vec<u8>>),
        
    }

    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
    pub struct RawMaterialUnit<T: Config> {
        pub unit_id: T::UniqueId,
        pub material_id: T::UniqueId,
        pub locations: Vec<Location<T>>,
        pub owners: Vec<Owner<T>>,
        pub custodians: Vec<Custodian<T>>,
    }

    impl <T: Config> RawMaterialUnit<T> {
        pub fn new(unit_id: T::UniqueId, material_id: T::UniqueId) -> Self {
            Self {
                unit_id,
                material_id,
                locations: Vec::new(),
                owners: Vec::new(),
                custodians: Vec::new(),
            }
        }

        pub fn get_unit_id(&self) -> T::UniqueId {
            self.unit_id.clone()
        }

        pub fn get_material_id(&self) -> T::UniqueId {
            self.material_id.clone()
        }

        pub fn get_proprietary_data_history(
            &self, input: ProprietaryDataInput<T>
        ) -> Vec<ProprietaryDataResult<T>> {
            match input {
                ProprietaryDataInput::Owner(_owner) => {
                    self.owners.iter().map(|owner| owner.get_data(AccessLevel::Owner).0).collect()
                },
                ProprietaryDataInput::Custodian(_custodian) => {
                    self.custodians.iter().map(|custodian| custodian.get_data(AccessLevel::Owner).0).collect()
                },
                ProprietaryDataInput::Location(_location) => {
                    self.locations.iter().map(|location| location.get_data(AccessLevel::Owner).0).collect()
                },
                ProprietaryDataInput::Invalid(_) => {
                    Vec::new()
                },
            }
        }

        pub fn get_proprietary_data_history_for_actor(
            &self, actor_type: AccessLevel, input: ProprietaryDataInput<T>
        ) -> Vec<(ProprietaryDataResult<T>, Option<u32>, Option<u32>, Option<u32>)> {
            match input {
                ProprietaryDataInput::Owner(_owner) => {
                    self.owners.iter().map(|owner| {
                        owner.get_data(actor_type.clone())
                        
                    }).collect()
                },
                ProprietaryDataInput::Custodian(_custodian) => {
                    self.custodians.iter().map(|custodian| {
                        custodian.get_data(actor_type.clone())
                    }).collect()
                },
                ProprietaryDataInput::Location(_location) => {
                    self.locations.iter().map(
                        |location| location.get_data(actor_type.clone())
                    ).collect()
                },
                ProprietaryDataInput::Invalid(_) => {
                    Vec::new()
                },
            }
        }

        pub fn get_current_proprietary_data(&self, input: ProprietaryDataInput<T>
        ) -> ProprietaryDataResult<T> {
            match input {
                ProprietaryDataInput::Owner(_) => {
                    // Find the current (active) owner
                    if let Some(current_owner) = self.owners.iter().find(|owner| owner.is_active()) {
                        current_owner.get_data(AccessLevel::Owner).0
                    } else {
                        ProprietaryDataResult::NotFound(b"No active owner found".to_vec())
                    }
                },
                ProprietaryDataInput::Custodian(_) => {
                    // Find the current (active) custodian
                    if let Some(current_custodian) = self.custodians.iter().find(|custodian| custodian.is_active()) {
                        current_custodian.get_data(AccessLevel::Owner).0
                    } else {
                        ProprietaryDataResult::NotFound(b"No active custodian found".to_vec())
                    }
                },
                ProprietaryDataInput::Location(_) => {
                    // Find the current (active) location
                    if let Some(current_location) = self.locations.iter().find(|location| location.is_active()) {
                        current_location.get_data(AccessLevel::Owner).0
                    } else {
                        ProprietaryDataResult::NotFound(b"No active location found".to_vec())
                    }
                },
                ProprietaryDataInput::Invalid(_) => {
                    ProprietaryDataResult::NotFound(b"Invalid input".to_vec())
                },
            }
        }

        pub fn update_proprietary_data(
            &mut self, input: ProprietaryDataInput<T>,
            current_block: u32
        ) {
            match input {
                ProprietaryDataInput::Owner(Some(mut new_data)) => {
                    
                    // get last owner bylooping through owners and getting owner where is active is true
                    // get the index of the last owner
                    // set the index of the new owner to the index of the last owner + 1
                    // expire the last owner
                    // push the new owner
                    if let Some(
                         last_owner
                    ) = self.owners
                            .iter_mut()
                            .find(|owner| owner.is_active())
                    {
                        new_data.set_index(last_owner.get_index() + 1);
                        last_owner.expire(current_block);
                    } else {
                        new_data.set_index(0);
                    }
                    self.owners.push(new_data);
                    
                },
                ProprietaryDataInput::Owner(None) => {},
                ProprietaryDataInput::Custodian(Some(mut new_data)) => {
                    if let Some(
                        last_custodian
                    ) = self.custodians
                            .iter_mut()
                            .find(|custodian| custodian.is_active())
                    {
                        new_data.set_index(last_custodian.get_index() + 1);
                        last_custodian.expire(current_block);
                    } else {
                        new_data.set_index(0);
                    }
                    self.custodians.push(new_data);
                },
                ProprietaryDataInput::Custodian(None) => {},
                ProprietaryDataInput::Location(Some(mut new_data)) => {
                    if let Some(
                        last_location
                    ) = self.locations
                            .iter_mut()
                            .find(|location| location.is_active())
                    {
                        new_data.set_index(last_location.get_index() + 1);
                        last_location.expire(current_block);
                    } else {
                        new_data.set_index(0);
                    }
                    self.locations.push(new_data);
                },
                ProprietaryDataInput::Location(None) => {},
                ProprietaryDataInput::Invalid(_) => {},

            }
        }


        pub fn is_owner(&self, actor_id: T::AccountId) -> bool {
            // Use get_current_proprietary_data to retrieve the current owner data
            let result = self.get_current_proprietary_data(ProprietaryDataInput::Owner(None));
            
            match result {
                ProprietaryDataResult::Owner(current_owner_id) => {
                    current_owner_id == actor_id
                },
                _ => false, // Either restricted or no valid owner found
            }
        }
    
        pub fn is_custodian(&self, actor_id: T::AccountId) -> bool {
            // Use get_current_proprietary_data to retrieve the current custodian data
            let result = self.get_current_proprietary_data(ProprietaryDataInput::Custodian(None));
            
            match result {
                ProprietaryDataResult::Custodian(current_custodian_id) => {
                    current_custodian_id == actor_id
                },
                _ => false, // Either restricted or no valid custodian found
            }
        }
    
        pub fn is_location(&self, location: (Vec<u8>, Vec<u8>)) -> bool {
            // Use get_current_proprietary_data to retrieve the current location data
            let result = self.get_current_proprietary_data(ProprietaryDataInput::Location(None));
            
            match result {
                ProprietaryDataResult::Location(current_location) => {
                    current_location == location
                },
                _ => false, // Either restricted or no valid location found
            }
        }
        
        
        pub fn set_access_level(
            &mut self, input: ProprietaryDataInput<T>, 
            visibility: AccessLevel, actor_id: T::AccountId,
            index: u32

        ) -> bool {
            if !self.is_owner(actor_id) {
                // The actor is not the current owner, so they cannot set visibility
                return false;
            }
            match input {
                ProprietaryDataInput::Owner(_) => {
                    // Find the owner with the specified index
                    if let Some(owner) = self.owners.iter_mut().find(|owner| owner.has_index(index)) {
                        // Set the visibility of the owner at the given index
                        owner.set_visibility(visibility);
                        return true;
                    }
                },
                ProprietaryDataInput::Custodian(_) => {
                    // Find the custodian with the specified index
                    if let Some(custodian) = self.custodians.iter_mut().find(|custodian| custodian.has_index(index)) {
                        // Set the visibility of the custodian at the given index
                        custodian.set_visibility(visibility);
                        return true;
                    }
                },
                ProprietaryDataInput::Location(_) => {
                    // Find the location with the specified index
                    if let Some(location) = self.locations.iter_mut().find(|location| location.has_index(index)) {
                        // Set the visibility of the location at the given index
                        location.set_visibility(visibility);
                        return true;
                    }
                },
                ProprietaryDataInput::Invalid(_) => {},
            }
            false

    }

    
    }

    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
    pub enum OrderStatus {
        Failed,
        Pending,
        InProgress,
        Completed,
        Cancelled,
        Confirmed,
    }
    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
    pub struct Order<T: Config> {
        pub order_id: T::UniqueId,
        pub supplier: T::AccountId,
        pub manufacturer: T::AccountId,
        pub material_id: T::UniqueId,
        pub raw_material_units: Vec<T::UniqueId>,
        pub shipping_agent: T::AccountId,
        pub shipping_manifest: Option<T::UniqueId>,
        pub delivery_location: (Vec<u8>, Vec<u8>),
        pub delivery_token: T::UniqueId,
        pub status: Vec<OrderStatus>,
    }

    impl <T: Config> Order<T> {
        pub fn new(
            order_id: T::UniqueId, supplier: T::AccountId, 
            manufacturer: T::AccountId, 
            shipping_agent: T::AccountId, 
            material_id: T::UniqueId, raw_material_units: Vec<T::UniqueId>,
            delivery_location: (Vec<u8>, Vec<u8>),
            delivery_token:T::UniqueId, status: OrderStatus
        ) -> Self {
            Self {
                order_id,
                supplier,
                manufacturer,
                material_id,
                raw_material_units,
                shipping_agent,
                shipping_manifest: None,
                delivery_location,
                delivery_token,
                status: vec![status],
            }
        }

        pub fn get_id(&self) -> T::UniqueId {
            self.order_id.clone()
        }

        pub fn get_supplier(&self) -> T::AccountId {
            self.supplier.clone()
        }

        pub fn get_material_id(&self) -> T::UniqueId {
            self.material_id.clone()
        }

        pub fn get_delivery_location(&self) -> (Vec<u8>, Vec<u8>) {
            self.delivery_location.clone()
        }

        pub fn is_supplier(&self, supplier_id: &T::AccountId) -> bool {
            &self.supplier == supplier_id
        }

        pub fn is_manufacturer(&self, manufacturer_id: &T::AccountId) -> bool {
            &self.manufacturer == manufacturer_id
        }

        pub fn is_shipping_agent(&self, shipping_agent_id: &T::AccountId) -> bool {
            &self.shipping_agent == shipping_agent_id
        }

        pub fn get_manufacturer(&self) -> T::AccountId {
            self.manufacturer.clone()
        }

        pub fn get_raw_material_units(&self) -> Vec<T::UniqueId> {
            self.raw_material_units.clone()
        }

        pub fn set_shipping_manifest(&mut self, shipping_manifest: T::UniqueId) {
            self.shipping_manifest = Some(shipping_manifest);
        }

        pub fn get_shipping_agent(&self) -> T::AccountId {
            self.shipping_agent.clone()
        }

        pub fn get_shipping_manifest(&self) -> Option<T::UniqueId> {
            Some(self.shipping_manifest.clone()?)
        }

        pub fn get_status(&self) -> OrderStatus {
            self.status.last().cloned().unwrap()
        }

        pub fn add_status(&mut self, status: OrderStatus) {
            self.status.push(status);
        }

        pub fn get_delivery_token(&self, actor: T::AccountId) -> Option<T::UniqueId> {
            if self.is_manufacturer(&actor) {
                Some(self.delivery_token.clone())
            } else {
                None
                
            }
        }

        pub fn is_delivery_token(&self, delivery_token: &T::UniqueId) -> bool {
            &self.delivery_token == delivery_token
        }
    }

    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
    pub struct ShippingManifest<T: Config> {
        pub manifest_id: T::UniqueId,
        pub order_id: T::UniqueId,
        pub shipping_agent: T::AccountId,
        pub pickup_location: (Vec<u8>, Vec<u8>),
        pub delivery_location: (Vec<u8>, Vec<u8>),
        pub route_history: Vec<(Vec<u8>, Vec<u8>)>,
        pub raw_material_units: Vec<T::UniqueId>,
        pub status: OrderStatus,
    }

    impl <T: Config> ShippingManifest<T> {
        pub fn new(
            manifest_id: T::UniqueId, order_id: T::UniqueId, 
            shipping_agent: T::AccountId, 
            pickup_location: (Vec<u8>, Vec<u8>), delivery_location: (Vec<u8>, Vec<u8>),
            raw_material_units: Vec<T::UniqueId>, status: OrderStatus
        ) -> Self {
            Self {
                manifest_id,
                order_id,
                shipping_agent,
                pickup_location,
                delivery_location,
                route_history: Vec::new(),
                raw_material_units,
                status,
            }
        }

        pub fn get_manifest_id(&self) -> T::UniqueId{
            self.manifest_id.clone()
        }

        pub fn get_order_id(&self) -> T::UniqueId {
            self.order_id.clone()
        }

        pub fn get_raw_material_units(&self) -> Vec<T::UniqueId> {
            self.raw_material_units.clone()
        }

        pub fn get_shipping_agent(&self) -> T::AccountId {
            self.shipping_agent.clone()
        }

        pub fn get_pickup_location(&self) -> (Vec<u8>, Vec<u8>) {
            self.pickup_location.clone()
        }

        pub fn get_delivery_location(&self) -> (Vec<u8>, Vec<u8>) {
            self.delivery_location.clone()
        }

        pub fn is_delivery_location(&self, delivery_location: &(Vec<u8>, Vec<u8>)) -> bool {
            &self.delivery_location == delivery_location
        }

        pub fn is_shipping_agent(&self, shipping_agent_id: &T::AccountId) -> bool {
            &self.shipping_agent == shipping_agent_id
        }

        pub fn add_route_history(&mut self, location: (Vec<u8>, Vec<u8>), actor_id: &T::AccountId) {
            if self.is_shipping_agent(actor_id) {
                self.route_history.push(location);
            } else {
            }
        }
    
    }

    #[pallet::storage]
	pub type Manufacturers<T: Config> = StorageMap<
		_, Blake2_128Concat, T::AccountId, Manufacturer<T>, OptionQuery
	>;

    #[pallet::storage]
	#[pallet::getter(fn supplier)]
	pub type Suppliers<T: Config> = StorageMap<
		_, Blake2_128Concat, T::AccountId, Supplier<T>, OptionQuery
	>;

    #[pallet::storage]
    #[pallet::getter(fn shipping_agent)]
    pub type ShippingAgents<T: Config> = StorageMap<
        _, Blake2_128Concat, T::AccountId, ShippingAgent<T>, OptionQuery
    >;

    #[pallet::storage]
    #[pallet::getter(fn raw_material)]
    pub type RawMaterials<T: Config> = StorageMap<
        _, Blake2_128Concat, T::UniqueId, (RawMaterial<T>, RawMaterialShipping<T>), OptionQuery
    >;

    #[pallet::storage]
    #[pallet::getter(fn raw_material_unit)]
    pub type RawMaterialUnits<T: Config> = StorageMap<
        _, Blake2_128Concat, T::UniqueId, RawMaterialUnit<T>, OptionQuery
    >;

    #[pallet::storage]
    #[pallet::getter(fn order)]
    pub type Orders<T: Config> = StorageMap<
        _, Blake2_128Concat, T::UniqueId, Order<T>, OptionQuery
    >;

    #[pallet::storage]
    #[pallet::getter(fn shipping_manifest)]
    pub type ShippingManifests<T: Config> = StorageMap<
        _, Blake2_128Concat, T::UniqueId, ShippingManifest<T>, OptionQuery
    >;



    #[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
        ActorRegistered {
            account: T::AccountId,
            actor_type: Vec<u8>
        },

        ActorsRetreived {
            actors: Vec<T::AccountId>
        },

        HasMaterialsActorDetailsRetreived {
            account_id: T::AccountId,
            company_name: Vec<u8>,
            ceritification_authority: Vec<u8>,
            ceritification_number: Vec<u8>,
            raw_materials: Option<BTreeMap<T::UniqueId, Vec<T::UniqueId>>>,
            orders: Option<Vec<T::UniqueId>>,
        },

        ShippingAgentDetailsRetrieved {
            account_id: T::AccountId,
            company_name: Vec<u8>,
            ceritification_authority: Vec<u8>,
            ceritification_number: Vec<u8>,
            pickup_locations: Vec<(Vec<u8>, Vec<u8>)>,
            delivery_locations: Vec<(Vec<u8>, Vec<u8>)>,
            manifests: Option<Vec<T::UniqueId>>,
        },

        ResourcesRetreived {
            name: Vec<u8>,
            resources: Vec<T::UniqueId>
        },

        OrderCreated {
            order_id: T::UniqueId,
            supplier: T::AccountId,
            manufacturer: T::AccountId,
            material_id: T::UniqueId,
            raw_material_units: Vec<T::UniqueId>,
            shipping_agent: T::AccountId,
            status: OrderStatus,
        },

        OrderShipped {
            order_id: T::UniqueId,
            manifest_id: T::UniqueId,
            shipping_agent: T::AccountId,
            status: OrderStatus,
        },

        ShippingLocationUpdated {
            manifest_id: T::UniqueId,
            location: (Vec<u8>, Vec<u8>),
            shipping_agent: T::AccountId,
        },

        OrderCompleted {
            order_id: T::UniqueId,
            manifest_id: T::UniqueId,
            status: OrderStatus,
        },

        RawMaterialUnitDataRetrieved {
            material_id: T::UniqueId,
            material_name: Vec<u8>,
            material_price: u32,
            unit_id: T::UniqueId,
            owner_history: Vec<(Option<T::AccountId>, Option<u32>, Option<u32>, Option<u32>)>,
            custodian_history: Vec<(Option<T::AccountId>, Option<u32>, Option<u32>, Option<u32>)>,
            location_history: Vec<(Option<(Vec<u8>, Vec<u8>)>, Option<u32>, Option<u32>, Option<u32>)>,
        },

        RawMaterialUnitAccessLevelUpdate {
            material_unit_id: T::UniqueId,
            status: bool,
        }

    }


    #[pallet::error]
	pub enum Error<T> {
        ActorAlreadyExists,
        ResourceDoesNotExist,
        UnathorisedRequest,
        InvalidShippingAgents,
        InsufficientUnits,
        UnsupportedShippingAgent,
        PaymentFailed,
        InvalidPropreitaryDataKey,
    }

    impl<T: Config> Pallet<T> {
        
        pub fn generate_unique_id() -> T::UniqueId {
            // let random_seed = <pallet_randomness_collective_flip::Module<T> as Randomness<T::Hash>>::random_seed();
            // let payload = (
            //     b"supply-chain-management",
            //     random_seed,
            //     frame_system::Pallet::<T>::extrinsic_index(),
            // );
            // T::UniqueId::decode(&mut TrailingZeroInput::new(blake2_128(&payload).to_vec())).unwrap_or_default()
            T::UniqueId::default()
        }

        pub fn get_current_block_number() -> u32 {
            // frame_system::Pallet::<T>::block_number();
            0
        }

        pub fn unwrap_account_id(input: ProprietaryDataResult<T>) -> Option<T::AccountId> {
            match input {
                ProprietaryDataResult::Owner(owner) => Some(owner),
                ProprietaryDataResult::Custodian(custodian) => Some(custodian),
                _ => None,
            }
        }

        pub fn unwrap_location(input: ProprietaryDataResult<T>) -> Option<(Vec<u8>, Vec<u8>)> {
            match input {
                ProprietaryDataResult::Location(location) => Some(location),
                _ => None,
            }
        }


    }

    #[pallet::call]
	impl<T: Config> Pallet<T> {
        #[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::do_something())]
		pub fn register_manufacturer(
			origin: OriginFor<T>, company_name: Vec<u8>, 
            ceritification_authority: Vec<u8>, ceritification_number: Vec<u8>
		) -> DispatchResult{
            let account_id = ensure_signed(origin)?;
            ensure!(
                !Manufacturers::<T>::contains_key(&account_id), 
                Error::<T>::ActorAlreadyExists
            );
            let manufacturer = Manufacturer::new(
                account_id.clone(), company_name, 
                ceritification_authority, ceritification_number
            );
            Manufacturers::<T>::insert(account_id.clone(), manufacturer);
            Self::deposit_event(Event::ActorRegistered {
                account: account_id,
                actor_type: b"Manufacturer".to_vec()
            });
            Ok(())
        }
        
        #[pallet::call_index(2)]
		#[pallet::weight(T::WeightInfo::do_something())]
		pub fn register_supplier(
			origin: OriginFor<T>, company_name: Vec<u8>, 
            ceritification_authority: Vec<u8>, ceritification_number: Vec<u8>
		) -> DispatchResult{
            let account_id = ensure_signed(origin)?;
            ensure!(
                !Suppliers::<T>::contains_key(&account_id), 
                Error::<T>::ActorAlreadyExists
            );
            let supplier = Supplier::new(
                account_id.clone(), company_name, 
                ceritification_authority, ceritification_number
            );
            Suppliers::<T>::insert(account_id.clone(), supplier);
            Self::deposit_event(Event::ActorRegistered {
                account: account_id,
                actor_type: b"Supplier".to_vec()
            });
            Ok(())
        }

        #[pallet::call_index(3)]
        #[pallet::weight(T::WeightInfo::do_something())]
        pub fn register_shipping_agent(
            origin: OriginFor<T>, company_name: Vec<u8>,
            ceritification_authority: Vec<u8>, ceritification_number: Vec<u8>,
            pickup_locations: Vec<(Vec<u8>, Vec<u8>)>, delivery_locations: Vec<(Vec<u8>, Vec<u8>)>
        ) -> DispatchResult{
            let account_id = ensure_signed(origin)?;
            ensure!(
                !ShippingAgents::<T>::contains_key(&account_id), 
                Error::<T>::ActorAlreadyExists
            );
            let mut shipping_agent = ShippingAgent::new(
                account_id.clone(), company_name, 
                ceritification_authority, ceritification_number
            );
            shipping_agent.add_pickup_locations(pickup_locations);
            shipping_agent.add_delivery_locations(delivery_locations);
            ShippingAgents::<T>::insert(account_id.clone(), shipping_agent);
            Self::deposit_event(Event::ActorRegistered {
                account: account_id,
                actor_type: b"Shipping Agent".to_vec()
            });
            Ok(())
        }

        #[pallet::call_index(4)]
		#[pallet::weight(T::WeightInfo::do_something())]
		pub fn get_manufacturers(
			origin: OriginFor<T>
		) -> DispatchResult{
            let _sender = ensure_signed(origin)?;
            let manufacturers = Manufacturers::<T>::iter().map(|(account_id, _)| account_id).collect();
            Self::deposit_event(Event::ActorsRetreived {
                actors: manufacturers
            });

            Ok(())
        }

        #[pallet::call_index(5)]
        #[pallet::weight(T::WeightInfo::do_something())]
        pub fn get_manufacturer(
            origin: OriginFor<T>, manufacturer_id: T::AccountId
        ) -> DispatchResult{
            let sender = ensure_signed(origin)?;
            let manufacturer = Manufacturers::<T>::get(&manufacturer_id).ok_or(
                Error::<T>::ResourceDoesNotExist)?;
            let mut raw_materials = None;
            let mut orders = None;
    
            
            if manufacturer.owns_account(sender) {
                raw_materials = Some(manufacturer.material_manager.get_raw_materials());
                orders = Some(manufacturer.order_manager.get_orders());
            }
            Self::deposit_event(Event::HasMaterialsActorDetailsRetreived {
                account_id: manufacturer.get_account(),
                company_name: manufacturer.get_company_name(),
                ceritification_authority:   manufacturer.get_ceritification_authority(),
                ceritification_number: manufacturer.get_ceritification_number(),
                raw_materials: raw_materials,
                orders: orders,
            });
            Ok(())
        }


        #[pallet::call_index(6)]
		#[pallet::weight(T::WeightInfo::do_something())]
		pub fn get_suppliers(
			origin: OriginFor<T>
		) -> DispatchResult{
            let _sender = ensure_signed(origin)?;
            let suppliers = Suppliers::<T>::iter().map(|(account_id, _)| account_id).collect();
            Self::deposit_event(Event::ActorsRetreived {
                actors: suppliers
            });
            Ok(())
        }

        #[pallet::call_index(7)]
        #[pallet::weight(T::WeightInfo::do_something())]
        pub fn get_supplier(
            origin: OriginFor<T>, supplier_id: T::AccountId
        ) -> DispatchResult{
            let sender = ensure_signed(origin)?;
            let supplier = Suppliers::<T>::get(&supplier_id).ok_or(
                Error::<T>::ResourceDoesNotExist)?;
            let mut raw_materials = None;
            let mut orders = None;
    
            
            if supplier.owns_account(sender) {
                raw_materials = Some(supplier.material_manager.get_raw_materials());
                orders = Some(supplier.order_manager.get_orders());
            }
            Self::deposit_event(Event::HasMaterialsActorDetailsRetreived {
                account_id: supplier.get_account(),
                company_name: supplier.get_company_name(),
                ceritification_authority:   supplier.get_ceritification_authority(),
                ceritification_number: supplier.get_ceritification_number(),
                raw_materials: raw_materials,
                orders: orders,
            });
            Ok(())
        }

        #[pallet::call_index(8)]
        #[pallet::weight(T::WeightInfo::do_something())]
        pub fn get_shipping_agent(
            origin: OriginFor<T>, shipping_agent_id: T::AccountId
        ) -> DispatchResult{
            let sender = ensure_signed(origin)?;
            let shipping_agent = ShippingAgents::<T>::get(&shipping_agent_id).ok_or(
                Error::<T>::ResourceDoesNotExist)?;
            let mut manifests = None;
    
            if shipping_agent.owns_account(sender) {
                manifests = Some(shipping_agent.get_manifests());
                
            }
            Self::deposit_event(Event::ShippingAgentDetailsRetrieved {
                account_id: shipping_agent.get_account(),
                company_name: shipping_agent.get_company_name(),
                ceritification_authority:   shipping_agent.get_ceritification_authority(),
                ceritification_number: shipping_agent.get_ceritification_number(),
                manifests: manifests,
                pickup_locations: shipping_agent.get_pickup_locations(),
                delivery_locations: shipping_agent.get_delivery_locations(),
               
            });
            Ok(())
        }

        #[pallet::call_index(9)]
		#[pallet::weight(T::WeightInfo::do_something())]
		pub fn get_shipping_agents(
			origin: OriginFor<T>
		) -> DispatchResult{
            let _sender = ensure_signed(origin)?;
            let shipping_agents = ShippingAgents::<T>::iter().map(|(account_id, _)| account_id).collect();
            Self::deposit_event(Event::ActorsRetreived {
                actors: shipping_agents
            });
            Ok(())
        }

        #[pallet::call_index(10)]
		#[pallet::weight(T::WeightInfo::do_something())]
		pub fn get_raw_materials(
			origin: OriginFor<T>
		) -> DispatchResult{
            let _sender = ensure_signed(origin)?;
            let raw_materials = RawMaterials::<T>::iter().map(|(material_id, _)| material_id).collect();
            Self::deposit_event(Event::ResourcesRetreived {
                name: b"Raw Materials".to_vec(),
                resources: raw_materials
            });
            Ok(())
        }

        #[pallet::call_index(11)]
        #[pallet::weight(T::WeightInfo::do_something())]
        pub fn register_raw_material(
            origin: OriginFor<T>, name: Vec<u8>, price: u32,
            quantity: u32, location: (Vec<u8>, Vec<u8>), shipping_agents: Vec<T::AccountId>
        ) -> DispatchResult{
            let sender = ensure_signed(origin)?;
            let mut supplier = Suppliers::<T>::get(&sender).ok_or(
                Error::<T>::UnathorisedRequest)?;
            
            let eligible_shipping_agents: Vec<T::AccountId> = shipping_agents
                .iter()
                .filter_map(|agent| {
                    // Try to get the shipping agent from storage
                    if let Some(agent_data) = ShippingAgents::<T>::get(agent) {
                        // Check if the agent can pick up from the location
                        if agent_data.can_pickup_from(&location) {
                            // If they can, return the agent
                            Some(agent.clone())
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .collect();
            ensure!(
                eligible_shipping_agents.len() > 0, 
                Error::<T>::InvalidShippingAgents
            );
            let material_id = Self::generate_unique_id();
            let mut raw_material = RawMaterial::<T>::new(
                material_id.clone(), name, price
            );

            for _ in  0..quantity {
                let unit_id = Self::generate_unique_id();
                let mut raw_material_unit = RawMaterialUnit::<T>::new(unit_id.clone(), material_id.clone());
                raw_material_unit.update_proprietary_data(
                    ProprietaryDataInput::Owner(
                        Some(Owner::new(
                            sender.clone(),
                            Self::get_current_block_number()
                        ))
                    ),
                    Self::get_current_block_number()
                );
                raw_material_unit.update_proprietary_data(
                    ProprietaryDataInput::Custodian(
                        Some(Custodian::new(
                            sender.clone(), 
                            Self::get_current_block_number()
                        ))
                    ),
                    Self::get_current_block_number()
                );
                
                raw_material_unit.update_proprietary_data(
                    ProprietaryDataInput::Location(
                        Some(Location::new(
                            location.0.clone(),
                            location.1.clone(),
                            Self::get_current_block_number()
                        ))
                    ),
                    Self::get_current_block_number()
                );

                raw_material.add_units(vec![unit_id.clone()]);

                RawMaterialUnits::<T>::insert(unit_id.clone(), raw_material_unit);
            }
            supplier.material_manager.add_units_to_raw_material(material_id.clone(), raw_material.get_units());

            let raw_material_shipping = RawMaterialShipping::<T>::new(
                location.clone(), eligible_shipping_agents

            );
            RawMaterials::<T>::insert(material_id.clone(), (raw_material, raw_material_shipping));
            // Save the updated supplier back to storage
            Suppliers::<T>::insert(&sender, supplier);

            Ok(())
        }

        #[pallet::call_index(12)]
		#[pallet::weight(T::WeightInfo::do_something())]
		pub fn order_raw_material(
            origin: OriginFor<T>,
			material_id: T::UniqueId,
            supplier_id: T::AccountId,
            quantity: u32,
            shipping_agent_id: T::AccountId,
            shipping_destination: (Vec<u8>, Vec<u8>),
		) -> DispatchResult{
            let buyer_account = ensure_signed(origin)?;

            let mut manufacturer = Manufacturers::<T>::get(&buyer_account).ok_or(
                Error::<T>::UnathorisedRequest)?;
            
            let mut supplier = Suppliers::<T>::get(&supplier_id).ok_or(
                Error::<T>::ResourceDoesNotExist)?;

            ensure!(
                supplier.material_manager.has_material_in_inventory(&material_id, quantity),
                Error::<T>::InsufficientUnits
            );
            
            let (_, raw_material_shipping) = RawMaterials::<T>::get(
                &material_id
            ).ok_or(
                Error::<T>::ResourceDoesNotExist
            )?;

            ensure!(
                raw_material_shipping.has_shipping_agent(&shipping_agent_id),
                Error::<T>::UnsupportedShippingAgent
            );

            let shipping_agent = ShippingAgents::<T>::get(&shipping_agent_id).ok_or(
                Error::<T>::ResourceDoesNotExist)?;
            
            ensure!(
                shipping_agent.can_ship(
                    &raw_material_shipping.get_pickup_location(), 
                    &shipping_destination),
                Error::<T>::UnsupportedShippingAgent
            );

            let mut units = Vec::new();

            for _ in 0..quantity {

                let unit_id = supplier.material_manager.get_unit_from_material(&material_id).ok_or(
                    Error::<T>::InsufficientUnits
                )?;
                let mut raw_material_unit = RawMaterialUnits::<T>::get(&unit_id).ok_or(
                    Error::<T>::ResourceDoesNotExist
                )?;
                // transfer token for payment
                // Self::make_payment(
                //     &buyer_account, &supplier_id, raw_material.get_price()
                // ).map_err(|_| Error::<T>::PaymentFailed)?;

                raw_material_unit.update_proprietary_data(
                    ProprietaryDataInput::Owner(
                        Some(Owner::new(
                            buyer_account.clone(),
                            Self::get_current_block_number()
                        ))
                    ),
                    Self::get_current_block_number()
                );
                units.push(unit_id.clone());
                supplier.material_manager.remove_unit_from_material(&material_id, &unit_id);
                manufacturer.material_manager.add_units_to_raw_material(material_id.clone(), vec![unit_id.clone()]);
                
                RawMaterialUnits::<T>::insert(unit_id.clone(), raw_material_unit);
            }

            let order_id = Self::generate_unique_id();
            let delivery_token = Self::generate_unique_id();
            let order = Order::<T>::new(
                order_id.clone(), supplier_id.clone(), buyer_account.clone(),
                shipping_agent_id.clone(), material_id.clone(),   units.clone(),
                shipping_destination, delivery_token.clone(), 
                OrderStatus::Confirmed
            );

            Manufacturers::<T>::insert(buyer_account.clone(), manufacturer);
            Suppliers::<T>::insert(supplier_id.clone(), supplier);
            Orders::<T>::insert(order_id.clone(), order);

            Self::deposit_event(Event::OrderCreated {
                order_id: order_id,
                supplier: supplier_id,
                manufacturer: buyer_account,
                material_id: material_id,
                raw_material_units: units,
                shipping_agent: shipping_agent_id.clone(),
                status: OrderStatus::Confirmed
            });
            
            

            Ok(())
        }

        #[pallet::call_index(13)]
        #[pallet::weight(T::WeightInfo::do_something())]
        pub fn get_orders(
            _origin: OriginFor<T>
        ) -> DispatchResult{
            let orders = Orders::<T>::iter().map(|(order_id, _)| order_id).collect();
            Self::deposit_event(Event::ResourcesRetreived {
                name: b"Orders".to_vec(),
                resources: orders
            });
            Ok(())
        }

        #[pallet::call_index(14)]
        #[pallet::weight(T::WeightInfo::do_something())]
        pub fn create_shipment(
            origin: OriginFor<T>,
            order_id: T::UniqueId,
        ) -> DispatchResult{
            let sender = ensure_signed(origin)?;
            let mut order = Orders::<T>::get(&order_id).ok_or(
                Error::<T>::ResourceDoesNotExist)?;
            ensure!(
                order.is_shipping_agent(&sender),
                Error::<T>::UnathorisedRequest
            );
            let mut shipping_agent = ShippingAgents::<T>::get(&sender).ok_or(
                Error::<T>::ResourceDoesNotExist)?;
            let (_, material_shipping) = RawMaterials::<T>::get(&order.get_material_id()).ok_or(
                Error::<T>::ResourceDoesNotExist)?;
            let current_block = Self::get_current_block_number();
            for unit_id in order.get_raw_material_units() {
                let mut raw_material_unit = RawMaterialUnits::<T>::get(&unit_id).ok_or(
                    Error::<T>::ResourceDoesNotExist)?;
                let custodian = Some(Custodian::new(
                    sender.clone(), 
                    current_block.clone()
                ));
                raw_material_unit.update_proprietary_data(
                    ProprietaryDataInput::Custodian(custodian),
                    current_block.clone()
                );
                RawMaterialUnits::<T>::insert(unit_id.clone(), raw_material_unit);
            }

            let manifest_id = Self::generate_unique_id();
            
            let shipping_manifest = ShippingManifest::<T>::new(
                manifest_id.clone(), order.get_id(), sender.clone(),
                material_shipping.get_pickup_location(),
                order.get_delivery_location(),
                order.get_raw_material_units(),
                OrderStatus::InProgress
            );
            shipping_agent.add_manifest(manifest_id.clone());
            order.set_shipping_manifest(manifest_id.clone());
            order.add_status(OrderStatus::InProgress);

            Orders::<T>::insert(order_id.clone(), order.clone());
            ShippingManifests::<T>::insert(manifest_id.clone(), shipping_manifest);
            ShippingAgents::<T>::insert(sender.clone(), shipping_agent);

            Self::deposit_event(Event::OrderShipped {
                order_id: order_id,
                manifest_id: manifest_id,
                shipping_agent: order.get_shipping_agent(),
                status: OrderStatus::InProgress
            });
            

            Ok(())
        }

        #[pallet::call_index(15)]
        #[pallet::weight(T::WeightInfo::do_something())]
        pub fn get_shipping_manifests(
            _origin: OriginFor<T>
        ) -> DispatchResult{
            Ok(())
        }

        #[pallet::call_index(16)]
        #[pallet::weight(T::WeightInfo::do_something())]
        pub fn update_shipping_location(
            origin: OriginFor<T>,
            manifest_id: T::UniqueId,
            location: (Vec<u8>, Vec<u8>)
        ) -> DispatchResult{
            let sender = ensure_signed(origin)?;
            let mut shipping_manifest = ShippingManifests::<T>::get(&manifest_id).ok_or(
                Error::<T>::ResourceDoesNotExist)?;
            ensure!(
                shipping_manifest.is_shipping_agent(&sender),
                Error::<T>::UnathorisedRequest
            );
            shipping_manifest.add_route_history(location.clone(), &sender);
            for unit_id in shipping_manifest.get_raw_material_units() {
                let mut raw_material_unit = RawMaterialUnits::<T>::get(&unit_id).ok_or(
                    Error::<T>::ResourceDoesNotExist)?;
                raw_material_unit.update_proprietary_data(
                    ProprietaryDataInput::Location(
                        Some(Location::new(
                            location.0.clone(),
                            location.1.clone(),
                            Self::get_current_block_number()
                        ))
                    ),
                    Self::get_current_block_number()
                );
                RawMaterialUnits::<T>::insert(unit_id.clone(), raw_material_unit);
            }
            ShippingManifests::<T>::insert(manifest_id.clone(), shipping_manifest);
            Self::deposit_event(Event::ShippingLocationUpdated {
                manifest_id: manifest_id,
                location: location,
                shipping_agent: sender
            });
            Ok(())
        }

        #[pallet::call_index(17)]
        #[pallet::weight(T::WeightInfo::do_something())]
        pub fn complete_shipment(
            origin: OriginFor<T>,
            manifest_id: T::UniqueId,
            location: (Vec<u8>, Vec<u8>),
            delivery_token: T::UniqueId
        ) -> DispatchResult{
            let sender = ensure_signed(origin)?;
            let mut shipping_manifest = ShippingManifests::<T>::get(&manifest_id).ok_or(
                Error::<T>::ResourceDoesNotExist)?;
            ensure!(
                shipping_manifest.is_shipping_agent(&sender),
                Error::<T>::UnathorisedRequest
            );
            ensure!(
                shipping_manifest.is_delivery_location(&location),
                Error::<T>::UnathorisedRequest
            );
            let order_id = shipping_manifest.get_order_id();
            let mut order = Orders::<T>::get(&order_id).ok_or(
                Error::<T>::ResourceDoesNotExist)?;
            ensure!(
                order.is_delivery_token(&delivery_token),
                Error::<T>::UnathorisedRequest
            );
            shipping_manifest.add_route_history(location.clone(), &sender);
            
            let custodian = Custodian::new(
                order.get_manufacturer(),
                Self::get_current_block_number()
            );
            for unit_id in shipping_manifest.get_raw_material_units() {
                let mut raw_material_unit = RawMaterialUnits::<T>::get(&unit_id).ok_or(
                    Error::<T>::ResourceDoesNotExist)?;
                raw_material_unit.update_proprietary_data(
                    ProprietaryDataInput::Location(
                        Some(Location::new(
                            location.0.clone(),
                            location.1.clone(),
                            Self::get_current_block_number()
                        ))
                    ),
                    Self::get_current_block_number()
                );
                raw_material_unit.update_proprietary_data(
                    ProprietaryDataInput::Custodian(Some(custodian.clone())),
                    Self::get_current_block_number()
                );
                RawMaterialUnits::<T>::insert(unit_id.clone(), raw_material_unit);
            }

            order.add_status(OrderStatus::Completed);

            Orders::<T>::insert(order_id.clone(), order);
            ShippingManifests::<T>::insert(manifest_id.clone(), shipping_manifest);
            Self::deposit_event(Event::OrderCompleted {
                order_id: order_id,
                manifest_id: manifest_id,
                status: OrderStatus::Completed
            });
            Ok(())
        }

        #[pallet::call_index(18)]
        #[pallet::weight(T::WeightInfo::do_something())]
        pub fn get_raw_material_unit_data(
            origin: OriginFor<T>,
            material_unit_id: T::UniqueId
        ) -> DispatchResult{
            let actor_id = ensure_signed(origin)?;
            let raw_material_unit = RawMaterialUnits::<T>::get(&material_unit_id).ok_or(
                Error::<T>::ResourceDoesNotExist)?;
            
            let actor_access_level = if raw_material_unit.is_owner(actor_id.clone()) {
                AccessLevel::Owner
            } else if raw_material_unit.is_custodian(actor_id.clone()) {
                AccessLevel::Custodian
            } else {
                AccessLevel::AllUsers
            };

            let owner_history = raw_material_unit.get_proprietary_data_history_for_actor(
                actor_access_level.clone(), ProprietaryDataInput::Owner(None)
            );
            let custodian_history = raw_material_unit.get_proprietary_data_history_for_actor(
                actor_access_level.clone(), ProprietaryDataInput::Custodian(None));

            let location_history = raw_material_unit.get_proprietary_data_history_for_actor(
                actor_access_level, ProprietaryDataInput::Location(None)
            );
            
            let (material, _) = RawMaterials::<T>::get(&raw_material_unit.get_material_id()).ok_or(
                Error::<T>::ResourceDoesNotExist)?;


            Self::deposit_event(Event::RawMaterialUnitDataRetrieved {
                material_id: raw_material_unit.get_material_id(),
                material_name: material.get_name(),
                material_price: material.get_price(),
                unit_id: material_unit_id,
                owner_history: owner_history.into_iter().map(
                    |(result, index, start, end)| {
                    (Self::unwrap_account_id(result), index, start, end)
                }).collect(),

                custodian_history: custodian_history.into_iter()
                .map(|(result, index, start, end)| {
                        (Self::unwrap_account_id(result), index, start, end)
                    }).collect(),

                location_history : location_history.into_iter()
                .map(|(result, index, start, end)| {
                        (Self::unwrap_location(result), index, start, end)
                    }).collect(),
            });

            Ok(())
        }

        #[pallet::call_index(19)]
        #[pallet::weight(T::WeightInfo::do_something())]
        pub fn set_raw_material_unit_access_level(
            origin: OriginFor<T>, material_unit_id: T::UniqueId,
            access_level: AccessLevel, proprietary_data: Vec<u8>,
            index: u32
        ) -> DispatchResult{
            let sender = ensure_signed(origin)?;
            
            let proprietary_data_input = match proprietary_data.as_slice() {
                b"Owner" => ProprietaryDataInput::Owner(None),
                b"Custodian" => ProprietaryDataInput::Custodian(None),
                b"Location" => ProprietaryDataInput::Location(None),
                _ => ProprietaryDataInput::Invalid(None),
            };
            ensure!(
                proprietary_data_input != ProprietaryDataInput::Invalid(None),
                Error::<T>::InvalidPropreitaryDataKey
            );

            let mut raw_material_unit = RawMaterialUnits::<T>::get(&material_unit_id).ok_or(
                Error::<T>::ResourceDoesNotExist)?;

            let result = raw_material_unit.set_access_level(
                proprietary_data_input, access_level, sender.clone(),
                index
            );

            RawMaterialUnits::<T>::insert(material_unit_id.clone(), raw_material_unit);

            Self::deposit_event(Event::RawMaterialUnitAccessLevelUpdate {
                material_unit_id: material_unit_id,
                status: result
            });
            Ok(())
        }

        



    }
}