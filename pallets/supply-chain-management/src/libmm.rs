//! ## Overview
//! Run `cargo doc --package pallet-template --open` to view this pallet's documentation.

// We make sure this pallet uses `no_std` for compiling to Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
// use alloc::vec::Vec;

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
		type ActorID: frame_system::AccountId;
		type SupplierAccountID: ActorID;
		type ManufacturerAccountID: ActorID;
		type ShippingAgentAccountID: ActorID;

		// type MaterialUnit: Parameter + Member + TypeInfo + 'static;

	}

	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
	pub struct Actor<T: Config> {
		pub account: T::ActorID,
		pub company_name: Vec<u8>,
		pub 
	}


	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
	pub struct Manufacturer {
		pub location: Vec<u8>,
	}

	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
	pub struct MaterialAdvert<T: Config> {
		pub identifier: Vec<u8>,
		pub available_quantity: u32,
		pub unit: MaterialUnit,
		pub price: u32,
		pub location: Vec<u8>,
		pub manufacturer: T::AccountId,
		pub shipping_agents: Vec<T::AccountId>,
	}

	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
	pub struct Producer {
		pub location: Vec<u8>
	}

	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
	pub struct ShippingAgent {
		pub license: Vec<u8>,
		// pub manifests: Vec<ShippingManifest<T>>,
	}

	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
	pub struct Order<T: Config> {
		pub manufacturer: T::AccountId,
		pub producer: T::AccountId,
		pub material_identifier: Vec<u8>,
		pub purchase_quantity: u32,
		pub amount: u32,
		pub status: OrderStatus,
		pub shipping_manifests: Vec<ShippingManifest<T>>,
	}

	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
	pub struct ShippingManifest<T: Config> {
		pub status: ShippingStatus,
		pub shipping_agent: T::AccountId,
		pub source_location: Vec<u8>,
		pub destination_location: Vec<u8>,
		pub current_location: Vec<u8>,
		pub cost: u32,
	}

	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
	pub enum MaterialUnit {
		Carton,
		Piece,
		Dozen
	}

	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
	pub enum OrderStatus {
		Pending,
		Completed,
		Cancelled,
	}

	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
	pub enum ShippingStatus {
		Accepted,
		InTransit,
		Delivered,
		Cancelled,
	}

	/// <https://docs.substrate.io/build/runtime-storage/>

	#[pallet::storage]
	#[pallet::getter(fn manufacturer)]
	pub type Manufacturers<T: Config> = StorageMap<
		_, Blake2_128Concat, T::AccountId, Manufacturer, OptionQuery
	>;

	#[pallet::storage]
	#[pallet::getter(fn producer)]
	pub type Producers<T: Config> = StorageMap<
		_, Blake2_128Concat, T::AccountId, Producer, OptionQuery
	>;

	#[pallet::storage]
	#[pallet::getter(fn shipping_agent)]
	pub type ShippingAgents<T: Config> = StorageMap<
		_, Blake2_128Concat, T::AccountId, ShippingAgent, OptionQuery
	>;

	#[pallet::storage]
	#[pallet::getter(fn order)]
	pub type Orders<T: Config> = StorageMap<
		_, Blake2_128Concat, Vec<u8>, Order<T> , OptionQuery
	>;



	#[pallet::storage]
	#[pallet::getter(fn producer_order)]
	pub type ProducerOrders<T: Config> = StorageMap<
		_, Blake2_128Concat, T::AccountId, Vec<Order<T>>, OptionQuery
	>;

	#[pallet::storage]
	#[pallet::getter(fn manufacturer_order)]
	pub type ManufacturerOrderRequests<T: Config> = StorageMap<
		_, Blake2_128Concat, T::AccountId, Vec<Order<T>>, OptionQuery
	>;

	#[pallet::storage]
	#[pallet::getter(fn material)]
	pub type Materials<T: Config> = StorageDoubleMap<
		_, 
		Blake2_128Concat, T::AccountId, 
		Blake2_128Concat, Vec<u8>, 
		MaterialAdvert<T>, 
		OptionQuery
	>;

	///  Events
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {

		ManufacturerRegistered {
			account: T::AccountId,
			location: Vec<u8>
		},

		MaterialAdvertPublished {
			manufacturer: T::AccountId,
			identifier: Vec<u8>,
		},

		ProducerRegistered {
			account: T::AccountId,
			location: Vec<u8>,
		},

		ShippingAgentRegistered {
			account: T::AccountId,
			license: Vec<u8>,
		},

		OrderPlaced {
			producer: T::AccountId,
			manufacturer: T::AccountId,
			material_identifier: Vec<u8>,
			order_id: Vec<u8>,
			quantity: u32,
			amount: u32,
		},
	}



	/// Errors
	#[pallet::error]
	pub enum Error<T> {
		/// The value retrieved was `None` as no value was previously set.
		NoneValue,
		/// There was an attempt to increment the value in storage over `u32::MAX`.
		StorageOverflow,
		ManufacturerAlreadyRegistered,
		AccountNotRegisteredAsManufacturer,
		ProducerAlreadyRegistered,
		ShippingAgentAlreadyRegistered,
		RequestOriginIsNotARegisteredProducer,
		MaterialDoesNotExist,
		InsufficientMaterialQuantity,
		UnsupportedShippingAgent,


	}


	/// Dispatchables
	#[pallet::call]
	impl<T: Config> Pallet<T> {



		// #[pallet::call_index(1)]
		// #[pallet::weight(T::WeightInfo::do_something())]
		// pub fn register_manufacturer(
		// 	origin: OriginFor<T>,
		// 	input_location: Vec<u8>
		// ) -> DispatchResult
		

		// 	let who: <T as frame_system::Config>::AccountId = ensure_signed(origin)?;

		// 	// Check if the manufacturer is already registered
		// 	ensure!(
		// 		Manufacturers::<T>::contains_key(&who),
		// 		Error::<T>::ManufacturerAlreadyRegistered
		// 	);

		// 	let manufacturer = Manufacturer {
		// 		location: input_location.clone(),
		// 	};

		// 	// Store the Manufacturer
		// 	Manufacturers::<T>::insert(&who, manufacturer);

		// 	// Emit success event
		// 	Self::deposit_event(Event::ManufacturerRegistered {
		// 		account: who,
		// 		location: input_location,
		// 	});

		// 	Ok(())
		// }
		
		// #[pallet::call_index(2)]
		// #[pallet::weight(T::WeightInfo::do_something())]
		// pub fn publish_material_advert(
		// 	origin: OriginFor<T>,
		// 	identifier: Vec<u8>,
		// 	available_quantity: u32,
		// 	unit: MaterialUnit,
		// 	price: u32,
		// 	shipping_agents: Vec<T::AccountId>
		// ) -> DispatchResult
		// {

			// let who	= ensure_signed(origin)?;

			// // Ensure Origin is a Registered Manufacturer
            // let manufacturer = Manufacturers::<T>::get(&who).ok_or(
			// 	Error::<T>::AccountNotRegisteredAsManufacturer)?;

			// // Verify that Object Is Not In Materials Store
			// ensure!(
			// 	!Materials::<T>::contains_key(&who, &identifier),
			// 	"Material Already Exists"
			// );

			// // Create Material Object
			// let material_advert = MaterialAdvert::<T> {
			// 	identifier: identifier.clone(),
			// 	available_quantity: available_quantity,
			// 	unit: unit,
			// 	price: price,
			// 	location: manufacturer.location.clone(),
			// 	manufacturer: who.clone(),
			// 	shipping_agents: shipping_agents,
			// };

			// // Register Material With Manufacturer
			// Materials::<T>::insert(&who, &identifier, material_advert);

			// // Emit Event
			// Self::deposit_event(Event::MaterialAdvertPublished {
			// 	manufacturer: who,
			// 	identifier: identifier,
			// });
			// Ok(())

		}

		// #[pallet::call_index(3)]
		// #[pallet::weight(T::WeightInfo::do_something())]
		// pub fn list_materials(_origin: OriginFor<T>) -> DispatchResult {
		//
		// 	Ok(())
		// }

		// #[pallet::call_index(4)]
		// #[pallet::weight(T::WeightInfo::do_something())]
		// pub fn register_product_producer(
		// 	origin: OriginFor<T>,
		// 	input_location: Vec<u8>
		// ) -> DispatchResult
		// {
		// 	let who: <T as frame_system::Config>::AccountId = ensure_signed(origin)?;

		// 	// Check if the manufacturer is already registered
		// 	ensure!(
		// 		Producers::<T>::contains_key(&who), !
		// 		Error::<T>::ProducerAlreadyRegistered
		// 	);

		// 	let producer = Producer {
		// 		location: input_location.clone()
		// 	};

		// 	// Store the Manufacturer
		// 	Producers::<T>::insert(&who, producer);

		// 	// Emit success event
		// 	Self::deposit_event(Event::ProducerRegistered {
		// 		account: who,
		// 		location: input_location,
		// 	});

		// 	Ok(())
		// }

		// #[pallet::call_index(5)]
		// #[pallet::weight(T::WeightInfo::do_something())]
		// pub fn register_shipping_agent(
		// 	origin: OriginFor<T>,
		// 	license: Vec<u8>
		// ) -> DispatchResult
		// {
		// 	let who = ensure_signed(origin)?;

		// 	ensure!(
		// 		ShippingAgents::<T>::contains_key(&who),  !
		// 		Error::<T>::ShippingAgentAlreadyRegistered
		// 	);

		// 	let shipping_agent = ShippingAgent {
		// 		license: license.clone()
		// 	};

		// 	// Store the Manufacturer
		// 	ShippingAgents::<T>::insert(&who, shipping_agent);

		// 	// Emit success event
		// 	Self::deposit_event(Event::ShippingAgentRegistered {
		// 		account: who,
		// 		license: license,
		// 	});

		// 	Ok(())
		// }

		// #[pallet::call_index(6)]
		// #[pallet::weight(T::WeightInfo::do_something())]
		// pub fn producer_place_order(
		// 	origin: OriginFor<T>,
		// 	manufacturer: T::AccountId,
		// 	shipping_agent: T::AccountId,
		// 	material_identifier: Vec<u8>,
		// 	quantity: u32,
		// ) -> DispatchResult
		// {
		// 	let who = ensure_signed(origin)?;

		// 	ensure!(
		// 		Producers::<T>::contains_key(&who),
		// 		Error::<T>::RequestOriginIsNotARegisteredProducer
		// 	);

		// 	let mut material_advert = Materials::<T>::get(&manufacturer, &material_identifier)
		// 		.ok_or(Error::<T>::MaterialDoesNotExist)?;

		// 	ensure!(
		// 		material_advert.shipping_agents.contains(&shipping_agent),
		// 		Error::<T>::UnsupportedShippingAgent
		// 	);

		// 	material_advert.available_quantity = material_advert.available_quantity.checked_sub(quantity)
		// 		.ok_or(Error::<T>::InsufficientMaterialQuantity)?;


		// 	// // Calculate order price
		// 	let order_amount = material_advert.price * quantity;
		// 	let shipping_cost = order_amount.checked_mul(6).and_then(|v| v.checked_div(100)).ok_or(
		// 		Error::<T>::NoneValue
		// 	)?;

		// 	let manufacturer_obj = Manufacturers::<T>::get(&manufacturer).ok_or(
		// 		Error::<T>::NoneValue)?;
		// 	let producer_obj = Producers::<T>::get(&who).ok_or(
		// 		Error::<T>::NoneValue
		// 	)?;

		// 	let manifest = ShippingManifest::<T> {
		// 		status: ShippingStatus::Accepted,
		// 		shipping_agent: shipping_agent.clone(),
		// 		source_location: manufacturer_obj.location.clone(),
		// 		destination_location: producer_obj.location,
		// 		current_location: manufacturer_obj.location.clone(),
		// 		cost: shipping_cost,
		// 	};

		// 	let order = Order::<T> {
		// 		producer: who.clone(),
		// 		manufacturer: manufacturer.clone(),
		// 		material_identifier: material_identifier.clone(),
		// 		purchase_quantity: quantity,
		// 		amount: order_amount.clone(),
		// 		status: OrderStatus::Pending,
		// 		shipping_manifests: vec![manifest],
		// 	};

		// 	// update to a more efficient nonce
		// 	let _nonce: u32 = 5;
		// 	// let order_id = T::Hashing::hash(
		// 	// 	&( &order, &who, &manufacturer, &material_identifier)
		// 	// );
		// 	let order_id: Vec<u8> = b"Chan".into();

		// 	Materials::<T>::insert(&manufacturer, &material_identifier, material_advert.clone());

		// 	// Store the manifest identifier in order
		// 	Orders::<T>::insert(&order_id, order.clone());
		// 	//

		// 	// Store order in orders storage map
		// 	ManufacturerOrderRequests::<T>::mutate(&manufacturer, |orders| {
		// 		if let Some(ref mut orders) = orders {
		// 			orders.push(order.clone());
		// 		} else {
		// 			*orders = Some(vec![order.clone()]);
		// 		}
		// 	});

		// 	ProducerOrders::<T>::mutate(&who, |orders| {
		// 		if let Some(ref mut orders) = orders {
		// 			orders.push(order.clone());
		// 		} else {
		// 			*orders = Some(vec![order.clone()]);
		// 		}
		// 	});
		// 	//
		// 	// // Emit success event
		// 	Self::deposit_event(Event::OrderPlaced {
		// 		producer: who,
		// 		manufacturer,
		// 		material_identifier,
		// 		order_id,
		// 		quantity,
		// 		amount: order_amount,
		// 	});

		// 	Ok(())
		// }
	//
	// 	#[pallet::call_index(7)]
	// 	#[pallet::weight(T::WeightInfo::do_something())]
	// 	pub fn shipping_agent_update_status(_origin: OriginFor<T>) -> DispatchResult {
	// 		Ok(())
	// 	}
	//
	// 	#[pallet::call_index(8)]
	// 	#[pallet::weight(T::WeightInfo::do_something())]
	// 	pub fn producer_confirm_order_fulfilment(_origin: OriginFor<T>) -> DispatchResult {
	// 		Ok(())
	// 	}
	//
	//
	// }
}
