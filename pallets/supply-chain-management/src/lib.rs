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
    
        // Get raw material units for a specific raw material ID
        pub fn get_units(&self, raw_material_id: &T::UniqueId) -> Option<&Vec<T::UniqueId>> {
            self.raw_materials.get(raw_material_id)
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

        pub fn can_pickup_from(&self, pickup_location: (Vec<u8>, Vec<u8>)) -> bool {
            self.pickup_locations.contains(&pickup_location)
        }

        pub fn can_deliver_to(&self, delivery_location: (Vec<u8>, Vec<u8>)) -> bool {
            self.delivery_locations.contains(&delivery_location)
        }

        pub fn can_ship(&self, pickup_location: (Vec<u8>, Vec<u8>), delivery_location: (Vec<u8>, Vec<u8>)) -> bool {
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
        fn get_data(&self, actor_type: AccessLevel) -> ProprietaryDataResult<T>;
    
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
        fn get_data(&self, actor_type: AccessLevel) -> ProprietaryDataResult<T> {
            if self.data.can_view(actor_type) {
                // TODO: Return owner, start_time, end_time as a single object
                ProprietaryDataResult::Owner(self.owner.clone())
            } else {
                ProprietaryDataResult::Restricted(b"You do not have access to this data".to_vec())  
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
        fn get_data(&self, actor_type: AccessLevel) -> ProprietaryDataResult<T> {
            if self.data.can_view(actor_type) {
                ProprietaryDataResult::Custodian(self.custodian.clone())
            } else {
                ProprietaryDataResult::Restricted(b"You do not have access to this data".to_vec())
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
        fn get_data(&self, actor_type: AccessLevel) -> ProprietaryDataResult<T> {
            if self.data.can_view(actor_type) {
                ProprietaryDataResult::Location((self.longitude.clone(), self.latitude.clone()))
            } else {
                ProprietaryDataResult::Restricted(b"You do not have access to this data".to_vec())
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
        pub fn new(pickup_location: (Vec<u8>, Vec<u8>)) -> Self {
            Self {
                pickup_location,
                shipping_agents: Vec::new(),
            }
        }

        pub fn add_shipping_agent(&mut self, shipping_agent: T::AccountId) {
            self.shipping_agents.push(shipping_agent);
        }

        pub fn get_shipping_agents(&self) -> Vec<T::AccountId> {
            self.shipping_agents.clone()
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
                    self.owners.iter().map(|owner| owner.get_data(AccessLevel::Owner)).collect()
                },
                ProprietaryDataInput::Custodian(_custodian) => {
                    self.custodians.iter().map(|custodian| custodian.get_data(AccessLevel::Owner)).collect()
                },
                ProprietaryDataInput::Location(_location) => {
                    self.locations.iter().map(|location| location.get_data(AccessLevel::Owner)).collect()
                },
            }
        }

        pub fn get_proprietary_data_history_for_actor(
            &self, actor_type: AccessLevel, input: ProprietaryDataInput<T>
        ) -> Vec<ProprietaryDataResult<T>> {
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
            }
        }

        pub fn get_current_proprietary_data(&self, input: ProprietaryDataInput<T>
        ) -> ProprietaryDataResult<T> {
            match input {
                ProprietaryDataInput::Owner(_) => {
                    // Find the current (active) owner
                    if let Some(current_owner) = self.owners.iter().find(|owner| owner.is_active()) {
                        current_owner.get_data(AccessLevel::Owner)
                    } else {
                        ProprietaryDataResult::NotFound(b"No active owner found".to_vec())
                    }
                },
                ProprietaryDataInput::Custodian(_) => {
                    // Find the current (active) custodian
                    if let Some(current_custodian) = self.custodians.iter().find(|custodian| custodian.is_active()) {
                        current_custodian.get_data(AccessLevel::Owner)
                    } else {
                        ProprietaryDataResult::NotFound(b"No active custodian found".to_vec())
                    }
                },
                ProprietaryDataInput::Location(_) => {
                    // Find the current (active) location
                    if let Some(current_location) = self.locations.iter().find(|location| location.is_active()) {
                        current_location.get_data(AccessLevel::Owner)
                    } else {
                        ProprietaryDataResult::NotFound(b"No active location found".to_vec())
                    }
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
        
        
        pub fn set_visibility(
            &mut self, input: ProprietaryDataInput<T>, 
            visibility: AccessLevel, actor_id: T::AccountId,
            index: u32

        ) -> bool{
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
    }
    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
    pub struct Order<T: Config> {
        pub order_id: T::UniqueId,
        pub supplier: T::AccountId,
        pub manufacturer: T::AccountId,
        pub raw_material_units: Vec<T::UniqueId>,
        pub shipping_agent: T::AccountId,
        pub shipping_manifest: T::UniqueId,
        pub delivery_token: T::UniqueId,
        pub status: OrderStatus,
    }

    impl <T: Config> Order<T> {
        pub fn new(
            order_id: T::UniqueId, supplier: T::AccountId, 
            manufacturer: T::AccountId, 
            shipping_agent: T::AccountId, shipping_manifest: T::UniqueId,
            delivery_token:T::UniqueId, status: OrderStatus
        ) -> Self {
            Self {
                order_id,
                supplier,
                manufacturer,
                raw_material_units: Vec::new(),
                shipping_agent,
                shipping_manifest,
                delivery_token,
                status,
            }
        }

        pub fn get_order_id(&self) -> T::UniqueId {
            self.order_id.clone()
        }

        pub fn get_supplier(&self) -> T::AccountId {
            self.supplier.clone()
        }

        pub fn is_supplier(&self, supplier_id: T::AccountId) -> bool {
            self.supplier == supplier_id
        }

        pub fn is_manufacturer(&self, manufacturer_id: T::AccountId) -> bool {
            self.manufacturer == manufacturer_id
        }

        pub fn is_shipping_agent(&self, shipping_agent_id: T::AccountId) -> bool {
            self.shipping_agent == shipping_agent_id
        }

        pub fn get_manufacturer(&self) -> T::AccountId {
            self.manufacturer.clone()
        }

        pub fn get_raw_material_units(&self) -> Vec<T::UniqueId> {
            self.raw_material_units.clone()
        }

        pub fn add_raw_material_units(&mut self, raw_material_unit: Vec<T::UniqueId>) {
            self.raw_material_units.extend(raw_material_unit);
        }

        pub fn get_shipping_agent(&self) -> T::AccountId {
            self.shipping_agent.clone()
        }

        pub fn get_shipping_manifest(&self) -> T::UniqueId {
            self.shipping_manifest.clone()
        }

        pub fn get_status(&self) -> OrderStatus {
            self.status.clone()
        }

        pub fn set_status(&mut self, status: OrderStatus) {
            self.status = status;
        }

        pub fn get_delivery_token(&self, actor: T::AccountId) -> Option<T::UniqueId> {
            if self.is_manufacturer(actor) {
                Some(self.delivery_token.clone())
            } else {
                None
                
            }
        }

        pub fn is_delivery_token(&self, delivery_token: T::UniqueId) -> bool {

            self.delivery_token == delivery_token
        }
    }

    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
    pub struct ShippingManifest<T: Config> {
        pub manifest_id: T::UniqueId,
        pub order_id: Option<T::UniqueId>,
        pub shipping_agent: T::AccountId,
        pub pickup_location: (Vec<u8>, Vec<u8>),
        pub delivery_location: (Vec<u8>, Vec<u8>),
        pub route_history: Vec<(Vec<u8>, Vec<u8>)>,
        pub raw_material_units: Vec<T::UniqueId>,
        pub status: OrderStatus,
    }

    impl <T: Config> ShippingManifest<T> {
        pub fn new(
            manifest_id: T::UniqueId, shipping_agent: T::AccountId, 
            pickup_location: (Vec<u8>, Vec<u8>), delivery_location: (Vec<u8>, Vec<u8>),
            raw_material_units: Vec<T::UniqueId>, status: OrderStatus
        ) -> Self {
            Self {
                manifest_id,
                order_id: None,
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

        pub fn get_order_id(&self) -> Option<T::UniqueId> {
            self.order_id.clone()
        }

        pub fn set_order_id(&mut self, order_id: T::UniqueId) {
            self.order_id = Some(order_id);
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

        pub fn is_delivery_location(&self, delivery_location: (Vec<u8>, Vec<u8>)) -> bool {
            self.delivery_location == delivery_location
        }

        pub fn is_shipping_agent(&self, shipping_agent_id: T::AccountId) -> bool {
            self.shipping_agent == shipping_agent_id
        }

        pub fn add_route_history(&mut self, location: (Vec<u8>, Vec<u8>), actor_id: T::AccountId) {
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

    }


    #[pallet::error]
	pub enum Error<T> {
        ActorAlreadyExists,
        ResourceDoesNotExist,
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
            origin: OriginFor<T>, name: Vec<u8>, price: u32
        ) -> DispatchResult{
            let account_id = ensure_signed(origin)?;
            let material_id = Self::generate_unique_id();
            let raw_material = RawMaterial::new(material_id.clone(), name, price);
            let raw_material_shipping = RawMaterialShipping::new((b"pickup".to_vec(), b"location".to_vec()));
            RawMaterials::<T>::insert(material_id.clone(), (raw_material, raw_material_shipping));
            Ok(())
        }

        #[pallet::call_index(12)]
		#[pallet::weight(T::WeightInfo::do_something())]
		pub fn order_raw_material(
			_origin: OriginFor<T>
		) -> DispatchResult{

            Ok(())
        }

        #[pallet::call_index(13)]
        #[pallet::weight(T::WeightInfo::do_something())]
        pub fn get_orders(
            _origin: OriginFor<T>
        ) -> DispatchResult{
            
            Ok(())
        }

        #[pallet::call_index(14)]
        #[pallet::weight(T::WeightInfo::do_something())]
        pub fn create_shipping(
            _origin: OriginFor<T>
        ) -> DispatchResult{
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
            _origin: OriginFor<T>
        ) -> DispatchResult{
            Ok(())
        }

        #[pallet::call_index(17)]
        #[pallet::weight(T::WeightInfo::do_something())]
        pub fn complete_shipment(
            _origin: OriginFor<T>
        ) -> DispatchResult{
            Ok(())
        }

        #[pallet::call_index(18)]
        #[pallet::weight(T::WeightInfo::do_something())]
        pub fn get_raw_material_unit_data(
            _origin: OriginFor<T>
        ) -> DispatchResult{
            Ok(())
        }

        #[pallet::call_index(19)]
        #[pallet::weight(T::WeightInfo::do_something())]
        pub fn set_raw_material_unit_access_level(
            _origin: OriginFor<T>
        ) -> DispatchResult{
            Ok(())
        }

        



    }
}