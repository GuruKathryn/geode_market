/*
ABOUT THIS CONTRACT...
This contract lets users buy and sell products (digital and physical) and services 
(online and in person) in the Geode ecosystem.
*/ 

#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod geode_marketplace {

    use ink::prelude::vec::Vec;
    use ink::prelude::vec;
    use ink::prelude::string::String;
    use ink::storage::Mapping;
    use ink::env::hash::{Sha2x256, HashOutput};
    use openbrush::{
        contracts::{
            reentrancy_guard::*,
            traits::errors::ReentrancyGuardError,
        },
        traits::{
            Storage,
            ZERO_ADDRESS
        },
    };

    // PRELIMINARY STORAGE STRUCTURES >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

    #[derive(Clone, scale::Decode, scale::Encode)]
    #[cfg_attr(feature = "std",
        derive(ink::storage::traits::StorageLayout, 
            scale_info::TypeInfo, Debug, PartialEq, Eq
        )
    )]
    pub struct AccountVector {
        accountvector: Vec<AccountId>,
    }

    impl Default for AccountVector {
        fn default() -> AccountVector {
            AccountVector {
              accountvector: <Vec<AccountId>>::default(),
            }
        }
    }

    #[derive(Clone, scale::Decode, scale::Encode)]
    #[cfg_attr(feature = "std",
        derive(ink::storage::traits::StorageLayout, 
            scale_info::TypeInfo, Debug, PartialEq, Eq
        )
    )]
    pub struct HashVector {
        hashvector: Vec<Hash>,
    }

    impl Default for HashVector {
        fn default() -> HashVector {
            HashVector {
              hashvector: <Vec<Hash>>::default(),
            }
        }
    }


    #[derive(Clone, scale::Decode, scale::Encode)]
    #[cfg_attr(feature = "std",
        derive(ink::storage::traits::StorageLayout, 
            scale_info::TypeInfo, Debug, PartialEq, Eq
        )
    )]
    pub struct Product { 
        product_id: Hash,
        digital: bool,
        title: Vec<u8>,
        price: Balance,
        brand: Vec<u8>,
        category: Vec<u8>,
        seller_account: AccountId,
        seller_name: Vec<u8>,
        description: Vec<u8>, 
        reviews: Vec<Hash>,
        inventory: u128, 
        photo_or_youtube_link1: Vec<u8>, 
        photo_or_youtube_link2: Vec<u8>,
        photo_or_youtube_link3: Vec<u8>,
        more_info_link: Vec<u8>,
        delivery_info: Vec<u8>,
        product_location: Vec<u8>,
        // include zip code, city, state, country, etc
        digital_file_url: Vec<u8>,
        zeno_percent: u128,
        // must be 0-100, default is 0
        zeno_buyers: Vec<AccountId>
        // tracks the first 20 buyers for zeno's incentive
    }

    impl Default for Product {
        fn default() -> Product {
            Product {
                product_id: Hash::default(),
                digital: bool::default(),
                title: <Vec<u8>>::default(),
                price: Balance::default(),
                brand:<Vec<u8>>::default(),
                category: <Vec<u8>>::default(),
                seller_account: ZERO_ADDRESS.into(),
                seller_name: <Vec<u8>>::default(),
                description: <Vec<u8>>::default(), 
                reviews: <Vec<Hash>>::default(),
                inventory: u128::default(), 
                photo_or_youtube_link1: <Vec<u8>>::default(), 
                photo_or_youtube_link2: <Vec<u8>>::default(),
                photo_or_youtube_link3: <Vec<u8>>::default(),
                more_info_link: <Vec<u8>>::default(),
                delivery_info: <Vec<u8>>::default(),
                product_location: <Vec<u8>>::default(),
                digital_file_url: <Vec<u8>>::default(),
                zeno_percent: 0,
                zeno_buyers: <Vec<AccountId>>::default()
            }
        }
    }

    #[derive(Clone, scale::Decode, scale::Encode)]
    #[cfg_attr(feature = "std",
        derive(ink::storage::traits::StorageLayout, 
            scale_info::TypeInfo, Debug, PartialEq, Eq
        )
    )]
    pub struct Service { 
        service_id: Hash,
        online: bool,
        title: Vec<u8>,
        price: Balance,
        category: Vec<u8>,
        seller_account: AccountId,
        seller_name: Vec<u8>,
        description: Vec<u8>,
        reviews: Vec<Hash>,
        inventory: u128,
        photo_or_youtube_link1: Vec<u8>,
        photo_or_youtube_link2: Vec<u8>,
        photo_or_youtube_link3: Vec<u8>,
        booking_link: Vec<u8>,
        service_location: Vec<u8>,
        // include zip code, city, state, country, etc 
        zeno_percent: u128,
        // must be 0-100, default is 0
        zeno_buyers: Vec<AccountId>
        // tracks the first 20 buyers for zeno's incentive
    }

    impl Default for Service {
        fn default() -> Service {
            Service {
                service_id: Hash::default(),
                online: bool::default(),
                title: <Vec<u8>>::default(),
                price: Balance::default(),
                category:<Vec<u8>>::default(),
                seller_account: ZERO_ADDRESS.into(),
                seller_name: <Vec<u8>>::default(),
                description: <Vec<u8>>::default(),
                reviews: <Vec<Hash>>::default(),
                inventory: u128::default(),
                photo_or_youtube_link1: <Vec<u8>>::default(),
                photo_or_youtube_link2: <Vec<u8>>::default(),
                photo_or_youtube_link3: <Vec<u8>>::default(),
                booking_link: <Vec<u8>>::default(),
                service_location: <Vec<u8>>::default(),
                zeno_percent: 0,
                zeno_buyers: <Vec<AccountId>>::default()
            }
        }
    }

    #[derive(Clone, scale::Decode, scale::Encode)]
    #[cfg_attr(feature = "std",
        derive(ink::storage::traits::StorageLayout, 
            scale_info::TypeInfo, Debug, PartialEq, Eq
        )
    )]
    pub struct UnpaidCartProduct { 
        product_id: Hash,
        quantity: u128,
        digital: bool,
        title: Vec<u8>,
        price: Balance,
        brand: Vec<u8>,
        seller_account: AccountId,
        seller_name: Vec<u8>,
        photo_or_youtube_link1: Vec<u8>, 
        inventory: u128, 
        delivery_info: Vec<u8>,
        product_location: Vec<u8>,
        zeno_percent: u128,
        zeno_buyers: u128
    }

    impl Default for UnpaidCartProduct {
        fn default() -> UnpaidCartProduct {
            UnpaidCartProduct {
                product_id: Hash::default(),
                quantity: u128::default(),
                digital: bool::default(),
                title: <Vec<u8>>::default(),
                price: Balance::default(),
                brand:<Vec<u8>>::default(),
                seller_account: ZERO_ADDRESS.into(),
                seller_name: <Vec<u8>>::default(),
                photo_or_youtube_link1: <Vec<u8>>::default(),
                inventory: u128::default(), 
                delivery_info: <Vec<u8>>::default(),
                product_location: <Vec<u8>>::default(),
                zeno_percent: 0,
                zeno_buyers: 0
            }
        }
    }

    #[derive(Clone, scale::Decode, scale::Encode)]
    #[cfg_attr(feature = "std",
        derive(ink::storage::traits::StorageLayout, 
            scale_info::TypeInfo, Debug, PartialEq, Eq
        )
    )]
    pub struct UnpaidCartService { 
        service_id: Hash,
        quantity: u128,
        online: bool,
        title: Vec<u8>,
        price: Balance,
        seller_account: AccountId,
        seller_name: Vec<u8>,
        photo_or_youtube_link1: Vec<u8>,
        inventory: u128,
        booking_link: Vec<u8>,
        service_location: Vec<u8>,
        zeno_percent: u128,
        zeno_buyers: u128
    }

    impl Default for UnpaidCartService {
        fn default() -> UnpaidCartService {
            UnpaidCartService {
                service_id: Hash::default(),
                quantity: u128::default(),
                online: bool::default(),
                title: <Vec<u8>>::default(),
                price: Balance::default(),
                seller_account: ZERO_ADDRESS.into(),
                seller_name: <Vec<u8>>::default(),
                photo_or_youtube_link1: <Vec<u8>>::default(),
                inventory: u128::default(),
                booking_link: <Vec<u8>>::default(),
                service_location: <Vec<u8>>::default(),
                zeno_percent: 0,
                zeno_buyers: 0
            }
        }
    }

    #[derive(Clone, scale::Decode, scale::Encode)]
    #[cfg_attr(feature = "std",
        derive(ink::storage::traits::StorageLayout, 
            scale_info::TypeInfo, Debug, PartialEq, Eq
        )
    )]
    pub struct PaidCart { 
        cart_id: Hash,
        buyer: AccountId,
        cart_timestamp: u64,
        //(updates to payment timestamp)
        cart_total: Balance,
        deliver_to_address: Vec<u8>,
        deliver_to_account: AccountId,
        orders: Vec<Order>, 
        total_items: u128,
        cart_items: Vec<Hash>
    }

    impl Default for PaidCart {
        fn default() -> PaidCart {
            PaidCart {
                cart_id: Hash::default(),
                buyer: ZERO_ADDRESS.into(),
                cart_timestamp: u64::default(),
                cart_total: Balance::default(),
                deliver_to_address: <Vec<u8>>::default(),
                deliver_to_account: ZERO_ADDRESS.into(),
                orders: <Vec<Order>>::default(), 
                total_items: 0,
                cart_items: <Vec<Hash>>::default()
            }
        }
    }

    #[derive(Clone, scale::Decode, scale::Encode)]
    #[cfg_attr(feature = "std",
        derive(ink::storage::traits::StorageLayout, 
            scale_info::TypeInfo, Debug, PartialEq, Eq
        )
    )]
    pub struct UnpaidCart { 
        buyer: AccountId,
        cart_total: Balance,
        total_items: u128,
        cart_items: Vec<(Hash, u128)>
    }

    impl Default for UnpaidCart {
        fn default() -> UnpaidCart {
            UnpaidCart {
                buyer: ZERO_ADDRESS.into(),
                cart_total: Balance::default(), 
                total_items: 0,
                cart_items: <Vec<(Hash, u128)>>::default()
            }
        }
    }

    #[derive(Clone, scale::Decode, scale::Encode)]
    #[cfg_attr(feature = "std",
        derive(ink::storage::traits::StorageLayout, 
            scale_info::TypeInfo, Debug, PartialEq, Eq
        )
    )]
    pub struct Order {
        order_id: Hash,
        cart_id: Hash,
        order_timestamp: u64,
        buyer: AccountId,
        seller: AccountId,
        seller_name: Vec<u8>,
        image: Vec<u8>,
        item_id: Hash,
        item_name: Vec<u8>,
        quantity: u128,
        price_each: Balance,
        total_order_price: Balance,
        deliver_to_address: Vec<u8>,
        deliver_to_account: AccountId,
        tracking_info: Vec<u8>,
        order_status: u8, 
        time_delivered: u64,
        discussion: Vec<Hash>,
        problem: u8,
        resolution: u8,
        zeno_total: Balance
    }
    // order_status code: 0 = awaiting seller confirmation, 1 = shipped, 2 = delivered, 3 = complete, 4 = problem, 5 = refused
    // a seller refusing an order triggers a refund to the buyer for that item
    // problem code: 0 = none, 1 = damaged, 2 = wrong item, 3 = did not receive
    // resolution code: 0 = none, 1 = refunded, 2 = replaced, 3 = resolution denied

    impl Default for Order {
        fn default() -> Order {
            Order {
                order_id: Hash::default(),
                cart_id: Hash::default(),
                order_timestamp: u64::default(),
                buyer: ZERO_ADDRESS.into(),
                seller: ZERO_ADDRESS.into(),
                seller_name: <Vec<u8>>::default(),
                image: <Vec<u8>>::default(),
                item_id: Hash::default(),
                item_name: <Vec<u8>>::default(),
                quantity: u128::default(),
                price_each: Balance::default(),
                total_order_price: Balance::default(),
                deliver_to_address: <Vec<u8>>::default(),
                deliver_to_account: ZERO_ADDRESS.into(),
                tracking_info: <Vec<u8>>::default(),
                order_status: 0, 
                time_delivered: u64::default(),
                discussion: <Vec<Hash>>::default(),
                problem: 0,
                resolution: 0,
                zeno_total: Balance::default()
            }
        }
    }

    #[derive(Clone, scale::Decode, scale::Encode)]
    #[cfg_attr(feature = "std",
        derive(ink::storage::traits::StorageLayout, 
            scale_info::TypeInfo, Debug, PartialEq, Eq
        )
    )]
    pub struct MessageDetails {
        message_id: Hash,
        from_acct: AccountId,
        to_acct: AccountId,
        order_id: Hash,
        message: Vec<u8>,
        media_url: Vec<u8>,
        timestamp: u64
    }

    impl Default for MessageDetails {
        fn default() -> MessageDetails {
            MessageDetails {
                message_id: Hash::default(),
                from_acct: ZERO_ADDRESS.into(),
                to_acct: ZERO_ADDRESS.into(),
                order_id: Hash::default(),
                message: <Vec<u8>>::default(),
                media_url: <Vec<u8>>::default(),
                timestamp: u64::default()
            }
        }
    }

    #[derive(Clone, scale::Decode, scale::Encode)]
    #[cfg_attr(feature = "std",
        derive(ink::storage::traits::StorageLayout, 
            scale_info::TypeInfo, Debug, PartialEq, Eq
        )
    )]
    pub struct ProductList { 
        owner: AccountId,
        list_id: Hash,
        list_name: Vec<u8>,
        items: Vec<Hash>
    }

    impl Default for ProductList {
        fn default() -> ProductList {
            ProductList {
                owner: ZERO_ADDRESS.into(),
                list_id: Hash::default(),
                list_name: <Vec<u8>>::default(),
                items: <Vec<Hash>>::default()
            }
        }
    }

    #[derive(Clone, scale::Decode, scale::Encode)]
    #[cfg_attr(feature = "std",
        derive(ink::storage::traits::StorageLayout, 
            scale_info::TypeInfo, Debug, PartialEq, Eq
        )
    )]
    pub struct ServiceList { 
        owner: AccountId,
        list_id: Hash,
        list_name: Vec<u8>,
        items: Vec<Hash>
    }

    impl Default for ServiceList {
        fn default() -> ServiceList {
            ServiceList {
                owner: ZERO_ADDRESS.into(),
                list_id: Hash::default(),
                list_name: <Vec<u8>>::default(),
                items: <Vec<Hash>>::default()
            }
        }
    }

    #[derive(Clone, scale::Decode, scale::Encode)]
    #[cfg_attr(feature = "std",
        derive(ink::storage::traits::StorageLayout, 
            scale_info::TypeInfo, Debug, PartialEq, Eq
        )
    )]
    pub struct BuyerProfile { 
        buyer_account: AccountId,
        buyer_name: Vec<u8>,
        buyer_location: Vec<u8>,
        member_since: u64,
        reviews: Vec<Hash>,  
        // reviews sellers have made about this buyer
        total_carts: u128,
        total_orders: u128,
        total_delivered: u128,
        total_damaged: u128,
        total_wrong: u128,
        total_not_received: u128,
        total_resolved: u128,
        total_refused: u128,
    }

    impl Default for BuyerProfile {
        fn default() -> BuyerProfile {
            BuyerProfile {
                buyer_account: ZERO_ADDRESS.into(),
                buyer_name: <Vec<u8>>::default(),
                buyer_location: <Vec<u8>>::default(),
                member_since: u64::default(),
                reviews: <Vec<Hash>>::default(),
                total_carts: u128::default(),
                total_orders: u128::default(),
                total_delivered: u128::default(),
                total_damaged: u128::default(),
                total_wrong: u128::default(),
                total_not_received: u128::default(),
                total_resolved: u128::default(),
                total_refused: u128::default(),
            }
        }
    }
    
    #[derive(Clone, scale::Decode, scale::Encode)]
    #[cfg_attr(feature = "std",
        derive(ink::storage::traits::StorageLayout, 
            scale_info::TypeInfo, Debug, PartialEq, Eq
        )
    )]
    pub struct SellerProfile { 
        seller_account: AccountId,
        seller_name: Vec<u8>,
        store_description: Vec<u8>,
        seller_location: Vec<u8>,
        member_since: u64,
        banner_url: Vec<u8>,
        youtube_url: Vec<u8>,
        external_link: Vec<u8>,
        reviews: Vec<Hash>,  
        // reviews buyers have made about this seller
        total_orders: u128,
        total_delivered: u128,
        total_damaged: u128,
        total_wrong: u128,
        total_not_received: u128,
        total_resolved: u128,
        total_refused: u128,
    }

    impl Default for SellerProfile {
        fn default() -> SellerProfile {
            SellerProfile {
                seller_account: ZERO_ADDRESS.into(),
                seller_name: <Vec<u8>>::default(),
                store_description: <Vec<u8>>::default(),
                seller_location: <Vec<u8>>::default(),
                member_since: u64::default(),
                banner_url: <Vec<u8>>::default(),
                youtube_url: <Vec<u8>>::default(),
                external_link: <Vec<u8>>::default(),
                reviews: <Vec<Hash>>::default(),  
                total_orders: u128::default(),
                total_delivered: u128::default(),
                total_damaged: u128::default(),
                total_wrong: u128::default(),
                total_not_received: u128::default(),
                total_resolved: u128::default(),
                total_refused: u128::default(),
            }
        }
    }

    #[derive(Clone, scale::Decode, scale::Encode)]
    #[cfg_attr(feature = "std",
        derive(ink::storage::traits::StorageLayout, 
            scale_info::TypeInfo, Debug, PartialEq, Eq
        )
    )]
    pub struct ProductServiceReview {
        review_id: Hash,
        item_id: Hash,
        reviewer: AccountId,
        rating: u8,
        // error if not 1-5
        review: Vec<u8>,
        timestamp: u64,
    }

    impl Default for ProductServiceReview {
        fn default() -> ProductServiceReview {
            ProductServiceReview {
                review_id: Hash::default(),
                item_id: Hash::default(),
                reviewer: ZERO_ADDRESS.into(),
                rating: u8::default(),
                review: <Vec<u8>>::default(),
                timestamp: u64::default(),
            }
        }
    }

    #[derive(Clone, scale::Decode, scale::Encode)]
    #[cfg_attr(feature = "std",
        derive(ink::storage::traits::StorageLayout, 
            scale_info::TypeInfo, Debug, PartialEq, Eq
        )
    )]
    pub struct BuyerSellerReview {
        review_id: Hash,
        account_id: AccountId,
        reviewer: AccountId,
        rating: u8,
        // error if not 1-5
        review: Vec<u8>,
        timestamp: u64,
    }

    impl Default for BuyerSellerReview {
        fn default() -> BuyerSellerReview {
            BuyerSellerReview {
                review_id: Hash::default(),
                account_id: ZERO_ADDRESS.into(),
                reviewer: ZERO_ADDRESS.into(),
                rating: u8::default(),
                review: <Vec<u8>>::default(),
                timestamp: u64::default(),
            }
        }
    }
   
    // STORAGE STRUCTURES FOR PRIMARY GET MESSAGES >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

    #[derive(Clone, scale::Decode, scale::Encode)]
    #[cfg_attr(feature = "std",
        derive(ink::storage::traits::StorageLayout, 
            scale_info::TypeInfo, Debug, PartialEq, Eq
        )
    )]
    pub struct ViewProductList { 
        owner: AccountId,
        list_id: Hash,
        list_name: Vec<u8>,
        items: Vec<Product>
    }

    impl Default for ViewProductList {
        fn default() -> ViewProductList {
            ViewProductList {
                owner: ZERO_ADDRESS.into(),
                list_id: Hash::default(),
                list_name: <Vec<u8>>::default(),
                items: <Vec<Product>>::default()
            }
        }
    }

    #[derive(Clone, scale::Decode, scale::Encode)]
    #[cfg_attr(feature = "std",
        derive(ink::storage::traits::StorageLayout, 
            scale_info::TypeInfo, Debug, PartialEq, Eq
        )
    )]
    pub struct ViewServiceList { 
        owner: AccountId,
        list_id: Hash,
        list_name: Vec<u8>,
        items: Vec<Service>
    }

    impl Default for ViewServiceList {
        fn default() -> ViewServiceList {
            ViewServiceList {
                owner: ZERO_ADDRESS.into(),
                list_id: Hash::default(),
                list_name: <Vec<u8>>::default(),
                items: <Vec<Service>>::default()
            }
        }
    }
    
    #[derive(Clone, scale::Decode, scale::Encode)]
    #[cfg_attr(feature = "std",
        derive(ink::storage::traits::StorageLayout, 
            scale_info::TypeInfo, Debug, PartialEq, Eq
        )
    )]
    pub struct ProductSearchResults {
        search: Vec<u8>,
        products: Vec<Product>
    }

    impl Default for ProductSearchResults {
        fn default() -> ProductSearchResults {
            ProductSearchResults {
                search: <Vec<u8>>::default(),
                products: <Vec<Product>>::default()
            }
        }
    }

    #[derive(Clone, scale::Decode, scale::Encode)]
    #[cfg_attr(feature = "std",
        derive(ink::storage::traits::StorageLayout, 
            scale_info::TypeInfo, Debug, PartialEq, Eq
        )
    )]
    pub struct ServiceSearchResults {
        search: Vec<u8>,
        services: Vec<Service>
    }

    impl Default for ServiceSearchResults {
        fn default() -> ServiceSearchResults {
            ServiceSearchResults {
                search: <Vec<u8>>::default(),
                services: <Vec<Service>>::default()
            }
        }
    }

    #[derive(Clone, scale::Decode, scale::Encode)]
    #[cfg_attr(feature = "std",
        derive(ink::storage::traits::StorageLayout, 
            scale_info::TypeInfo, Debug, PartialEq, Eq
        )
    )]
    pub struct StoreSearchResults {
        search: Vec<u8>,
        stores: Vec<SellerProfile>
    }

    impl Default for StoreSearchResults {
        fn default() -> StoreSearchResults {
            StoreSearchResults {
                search: <Vec<u8>>::default(),
                stores: <Vec<SellerProfile>>::default()
            }
        }
    }

    #[derive(Clone, scale::Decode, scale::Encode)]
    #[cfg_attr(feature = "std",
        derive(ink::storage::traits::StorageLayout, 
            scale_info::TypeInfo, Debug, PartialEq, Eq
        )
    )]
    pub struct ViewBuyerOrders {
        buyer: AccountId,
        carts: Vec<PaidCart>
    }

    impl Default for ViewBuyerOrders {
        fn default() -> ViewBuyerOrders {
            ViewBuyerOrders {
                buyer: ZERO_ADDRESS.into(),
                carts: <Vec<PaidCart>>::default()
            }
        }
    }

    #[derive(Clone, scale::Decode, scale::Encode)]
    #[cfg_attr(feature = "std",
        derive(ink::storage::traits::StorageLayout, 
            scale_info::TypeInfo, Debug, PartialEq, Eq
        )
    )]
    pub struct Download {
        product_id: Hash,
        title: Vec<u8>,
        brand: Vec<u8>,
        seller_account: AccountId,
        seller_name: Vec<u8>,
        description: Vec<u8>,
        photo: Vec<u8>,
        more_info: Vec<u8>,
        file_url: Vec<u8>,
    }

    impl Default for Download {
        fn default() -> Download {
            Download {
                product_id: Hash::default(),
                title: <Vec<u8>>::default(),
                brand: <Vec<u8>>::default(),
                seller_account: ZERO_ADDRESS.into(),
                seller_name: <Vec<u8>>::default(),
                description: <Vec<u8>>::default(),
                photo: <Vec<u8>>::default(),
                more_info: <Vec<u8>>::default(),
                file_url: <Vec<u8>>::default(),
            }
        }
    }

    #[derive(Clone, scale::Decode, scale::Encode)]
    #[cfg_attr(feature = "std",
        derive(ink::storage::traits::StorageLayout, 
            scale_info::TypeInfo, Debug, PartialEq, Eq
        )
    )]
    pub struct ViewBuyerAccount {
        buyer: BuyerProfile,
        product_lists: Vec<ViewProductList>,
        service_lists: Vec<ViewServiceList>,
        bookmarked_stores: Vec<SellerProfile>,
        digital_downloads: Vec<Download>
    }

    impl Default for ViewBuyerAccount {
        fn default() -> ViewBuyerAccount {
            ViewBuyerAccount {
                buyer: BuyerProfile::default(),
                product_lists: <Vec<ViewProductList>>::default(),
                service_lists: <Vec<ViewServiceList>>::default(),
                bookmarked_stores: <Vec<SellerProfile>>::default(),
                digital_downloads: <Vec<Download>>::default()
            }
        }
    }

    #[derive(Clone, scale::Decode, scale::Encode)]
    #[cfg_attr(feature = "std",
        derive(ink::storage::traits::StorageLayout, 
            scale_info::TypeInfo, Debug, PartialEq, Eq
        )
    )]
    pub struct ViewUnpaidCart { 
        buyer: AccountId,
        cart_total: Balance,
        total_items: u128,
        cart_products: Vec<UnpaidCartProduct>,
        cart_services: Vec<UnpaidCartService>
    }

    impl Default for ViewUnpaidCart {
        fn default() -> ViewUnpaidCart {
            ViewUnpaidCart {
                buyer: ZERO_ADDRESS.into(),
                cart_total: Balance::default(), 
                total_items: 0,
                cart_products: <Vec<UnpaidCartProduct>>::default(),
                cart_services: <Vec<UnpaidCartService>>::default()
            }
        }
    }

    #[derive(Clone, scale::Decode, scale::Encode)]
    #[cfg_attr(feature = "std",
        derive(ink::storage::traits::StorageLayout, 
            scale_info::TypeInfo, Debug, PartialEq, Eq
        )
    )]
    pub struct ViewStore {
        owner: SellerProfile,
        products: Vec<Product>,
        services: Vec<Service>
    }

    impl Default for ViewStore {
        fn default() -> ViewStore {
            ViewStore {
                owner: SellerProfile::default(),
                products: <Vec<Product>>::default(),
                services: <Vec<Service>>::default()
            }
        }
    }

    #[derive(Clone, scale::Decode, scale::Encode)]
    #[cfg_attr(feature = "std",
        derive(ink::storage::traits::StorageLayout, 
            scale_info::TypeInfo, Debug, PartialEq, Eq
        )
    )]
    pub struct ViewSellerAccount {
        seller: SellerProfile,
        current_orders: Vec<Order>,
        products: Vec<Product>,
        services: Vec<Service>
    }

    impl Default for ViewSellerAccount {
        fn default() -> ViewSellerAccount {
            ViewSellerAccount {
                seller: SellerProfile::default(),
                current_orders: <Vec<Order>>::default(),
                products: <Vec<Product>>::default(),
                services: <Vec<Service>>::default()
            }
        }
    }

    #[derive(Clone, scale::Decode, scale::Encode)]
    #[cfg_attr(feature = "std",
        derive(ink::storage::traits::StorageLayout, 
            scale_info::TypeInfo, Debug, PartialEq, Eq
        )
    )]
    pub struct OrderData {
        timestamp: u64,
        total: Balance,
        status: u8,
        problem: u8,
        resolution: u8,
    }

    impl Default for OrderData {
        fn default() -> OrderData {
            OrderData {
                timestamp: u64::default(),
                total: Balance::default(),
                status: 0,
                problem: 0,
                resolution: 0,
            }
        }
    }

    #[derive(Clone, scale::Decode, scale::Encode)]
    #[cfg_attr(feature = "std",
        derive(ink::storage::traits::StorageLayout, 
            scale_info::TypeInfo, Debug, PartialEq, Eq
        )
    )]
    pub struct MarketStatistics {
        called_by: AccountId,
        timestamp: u64,
        number_of_sellers: u128,
        number_of_buyers: u128,
        number_of_products: u128,
        number_of_services: u128,
        number_of_orders: u128,
        orders: Vec<OrderData>
    }

    impl Default for MarketStatistics {
        fn default() -> MarketStatistics {
            MarketStatistics {
                called_by: ZERO_ADDRESS.into(),
                timestamp: u64::default(),
                number_of_sellers: u128::default(),
                number_of_buyers: u128::default(),
                number_of_products: u128::default(),
                number_of_services: u128::default(),
                number_of_orders: u128::default(),
                orders: <Vec<OrderData>>::default()
            }
        }
    }

    #[derive(Clone, scale::Decode, scale::Encode)]
    #[cfg_attr(feature = "std",
        derive(ink::storage::traits::StorageLayout, 
            scale_info::TypeInfo, Debug, PartialEq, Eq
        )
    )]
    pub struct ViewZeno {
        products: Vec<Product>,
        services: Vec<Service>
    }

    impl Default for ViewZeno {
        fn default() -> ViewZeno {
            ViewZeno {
                products: <Vec<Product>>::default(),
                services: <Vec<Service>>::default()
            }
        }
    }

    
    // EVENT DEFINITIONS >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>> 

    #[ink(event)]
    // writes a new order to the blockchain 
    pub struct OrderPlaced {
        #[ink(topic)]
        order_id: Hash,
        order_timestamp: u64,
        #[ink(topic)]
        buyer: AccountId,
        #[ink(topic)]
        seller: AccountId,
        total_order_price: Balance,
    }




    // ERROR DEFINITIONS >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
    pub enum Error {
        // if the payment fails
        PayoutFailed,
        // Reentrancy Guard error
        ReentrancyError(ReentrancyGuardError),
        // naming an account that was not there
        NonexistentAccount,
        // naming an item that does not exist
        ItemDoesNotExist,
        // adding an item or seller to a list twice
        Duplicate,
        // not paying enough for your cart 
        InsufficientPayment,
        // trying to review an item or seller you didn't buy / buy from
        NotEligibleToReview,
        // rating number outside of 1-5
        RatingOutOfBounds,
        // reporting a problem on an order you did not place
        NotYourOrder,
        // reporting a problem too late
        NotEligibleToReport,
        // editing a list you do not own
        NotYourList,
        // refusing an order that has already shipped or delivered
        CannotRefuse,
        // updating tracking on the wrong kind of item
        NotAPhysicalProduct,
        // trying to resolve an order that is not in problem status
        CannotResolve,
        // updating a product you do not own
        NotYourProduct,
    }

    impl From<ReentrancyGuardError> for Error {
        fn from(error:ReentrancyGuardError) -> Self {
            Error::ReentrancyError(error)
        }
    }


    // ACTUAL CONTRACT STORAGE STRUCT >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct ContractStorage {
        #[storage_field]
        guard: reentrancy_guard::Data,
        all_sellers: Vec<AccountId>,
        all_buyers: Vec<AccountId>,
        all_products: Vec<Hash>,
        all_services: Vec<Hash>,
        all_orders: Vec<Hash>,
        account_profile_seller: Mapping<AccountId, SellerProfile>,
        account_profile_buyer: Mapping<AccountId, BuyerProfile>,
        account_product_lists: Mapping<AccountId, HashVector>,
        account_service_lists: Mapping<AccountId, HashVector>,
        account_store_bookmarks: Mapping<AccountId, AccountVector>,
        buyer_store_history: Mapping<AccountId, AccountVector>,
        seller_customer_history: Mapping<AccountId, AccountVector>,
        account_buyer_orders: Mapping<AccountId, HashVector>,
        account_buyer_items_bought: Mapping<AccountId, HashVector>,
        account_buyer_items_reviewed: Mapping<AccountId, HashVector>,
        account_buyer_sellers_reviewed: Mapping<AccountId, AccountVector>,
        account_seller_buyers_reviewed: Mapping<AccountId, AccountVector>,
        account_owned_digital_items: Mapping<AccountId, HashVector>,
        account_seller_orders: Mapping<AccountId, HashVector>,
        account_paid_carts: Mapping<AccountId, HashVector>,
        account_current_cart: Mapping<AccountId, UnpaidCart>,
        account_seller_products: Mapping<AccountId, HashVector>,
        account_seller_services: Mapping<AccountId, HashVector>,
        message_details: Mapping<Hash, MessageDetails>,
        paid_cart_details: Mapping<Hash, PaidCart>,
        product_details: Mapping<Hash, Product>,
        service_details: Mapping<Hash, Service>,
        order_details: Mapping<Hash, Order>,
        product_list_details: Mapping<Hash, ProductList>,
        service_list_details: Mapping<Hash, ServiceList>,
        item_review_details: Mapping<Hash, ProductServiceReview>,
        account_review_details: Mapping<Hash, BuyerSellerReview>,
    }


    // BEGIN CONTRACT LOGIC >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

    impl ContractStorage {
        
        // CONSTRUCTORS >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
        // Constructors are implicitly payable.

        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                guard: Default::default(),
                all_sellers: <Vec<AccountId>>::default(),
                all_buyers: <Vec<AccountId>>::default(),
                all_products: <Vec<Hash>>::default(),
                all_services: <Vec<Hash>>::default(),
                all_orders: <Vec<Hash>>::default(),
                account_profile_seller: Mapping::default(),
                account_profile_buyer: Mapping::default(),
                account_product_lists: Mapping::default(),
                account_service_lists: Mapping::default(),
                account_store_bookmarks: Mapping::default(),
                buyer_store_history: Mapping::default(),
                seller_customer_history: Mapping::default(),
                account_buyer_orders: Mapping::default(),
                account_buyer_items_bought: Mapping::default(),
                account_buyer_items_reviewed: Mapping::default(),
                account_buyer_sellers_reviewed: Mapping::default(),
                account_seller_buyers_reviewed: Mapping::default(),
                account_owned_digital_items: Mapping::default(),
                account_seller_orders: Mapping::default(),
                account_paid_carts: Mapping::default(),
                account_current_cart: Mapping::default(),
                account_seller_products: Mapping::default(),
                account_seller_services: Mapping::default(),
                message_details: Mapping::default(),
                paid_cart_details: Mapping::default(),
                product_details: Mapping::default(),
                service_details: Mapping::default(),
                order_details: Mapping::default(),
                product_list_details: Mapping::default(),
                service_list_details: Mapping::default(),
                item_review_details: Mapping::default(),
                account_review_details: Mapping::default(),
            }
        }


        // >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
        // MESSAGE FUNCTIONS THAT CHANGE DATA IN THE CONTRACT STORAGE >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
        // >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>


        // 1 🟢 Add Item To Cart
        #[ink(message)]
        pub fn add_item_to_cart (&mut self, 
            add_item_id: Hash, 
            quantity: u128
        ) -> Result<(), Error> {
            // set up the caller
            let caller = Self::env().caller();
            // get the current unpaid cart for this caller from account_current_cart
            let mut cart = self.account_current_cart.get(&caller).unwrap_or_default();
            // if the cart is empty, add this item
            if cart.total_items == 0 {
                // push the item and quantity onto the vector of cart_items
                cart.cart_items.push((add_item_id, quantity));
            }
            // if the cart is not empty...
            else {
                let mut new_quantity = quantity;
                let mut old_quantity: u128 = 0;
                // is this item already in the cart?
                for (item, number) in &cart.cart_items {
                    if *item == add_item_id {
                        // increase the quantity
                        old_quantity = *number;
                        new_quantity = number + quantity;
                    }
                }
                // remove the old entry
                cart.cart_items.retain(|value| *value != (add_item_id,old_quantity));
                // add this item
                cart.cart_items.push((add_item_id, new_quantity));
            }
            // get the total number of items in the cart
            let totalitems: u128 = cart.cart_items.len().try_into().unwrap();
            // iterate through the cart items to get the total price of the cart
            let mut carttotal: Balance = 0;
            for (item, number) in &cart.cart_items {
                // get the price for that item
                let mut item_price: Balance = 0;
                if self.product_details.contains(&item) {
                    item_price = self.product_details.get(&item).unwrap_or_default().price;
                }
                else {
                    if self.service_details.contains(&item) {
                        item_price = self.service_details.get(&item).unwrap_or_default().price;
                    }
                }
                // add the price to the total price
                carttotal += item_price * number;
            }

            // perpare the updated UnpaidCart
            let updated_cart = UnpaidCart {
                buyer: caller,
                cart_total: carttotal,
                total_items: totalitems,
                cart_items: cart.cart_items
            };

            // update mappings
            self.account_current_cart.insert(&caller, &updated_cart);
            
            Ok(())
        }


        // 2 🟢 Add Item To Product List 
        #[ink(message)]
        pub fn add_item_to_product_list (&mut self, 
            product_id: Hash, 
            list_name: Vec<u8>,
        ) -> Result<(), Error> {
            // set up clones
            let list_name_clone1 = list_name.clone();
            // set up the caller
            let caller = Self::env().caller();
            // does a list by this name exist? 
            // hash the list name
            let encodable = list_name; // Implements `scale::Encode`
            let mut new_id_u8 = <Sha2x256 as HashOutput>::Type::default(); // 256-bit buffer
            ink::env::hash_encoded::<Sha2x256, _>(&encodable, &mut new_id_u8);
            let new_id: Hash = Hash::from(new_id_u8);
            // is this list id in the caller's account_product_list mapping?
            let mut my_lists = self.account_product_lists.get(&caller).unwrap_or_default();
            if my_lists.hashvector.contains(&new_id) {
                // if the list exsists, check to see if the product is already there
                let mut list = self.product_list_details.get(&new_id).unwrap_or_default();
                if list.items.contains(&product_id) {
                    // error, already on the list
                    return Err(Error::Duplicate)
                }
                else {
                    // add the product to the ProductList
                    list.items.push(product_id);
                }
                let update = ProductList {
                    owner: caller,
                    list_id: list.list_id,
                    list_name: list.list_name,
                    items: list.items
                };
                // update mappings
                self.product_list_details.insert(&new_id, &update);

            }
            else {
                // make a new list and put this product in the list
                let newlist = ProductList {
                    owner: caller,
                    list_id: new_id,
                    list_name: list_name_clone1,
                    items: vec![product_id]
                };
                // update mappings
                // product_list_details: Mapping<Hash, ProductList>
                self.product_list_details.insert(&new_id, &newlist);
                // account_product_lists: Mapping<AccountId, HashVector>
                my_lists.hashvector.push(new_id);
                self.account_product_lists.insert(&caller, &my_lists);
            }
            
            Ok(())
        }


        // 3 🟢 Add Item To Service List
        #[ink(message)]
        pub fn add_item_to_service_list (&mut self, 
            service_id: Hash, 
            list_name: Vec<u8>,
        ) -> Result<(), Error> {
            // set up clones
            let list_name_clone1 = list_name.clone();
            // set up the caller
            let caller = Self::env().caller();
            // does a list by this name exist? 
            // hash the list name
            let encodable = list_name; // Implements `scale::Encode`
            let mut new_id_u8 = <Sha2x256 as HashOutput>::Type::default(); // 256-bit buffer
            ink::env::hash_encoded::<Sha2x256, _>(&encodable, &mut new_id_u8);
            let new_id: Hash = Hash::from(new_id_u8);
            // is this list id in the caller's account_service_list mapping?
            let mut my_lists = self.account_service_lists.get(&caller).unwrap_or_default();
            if my_lists.hashvector.contains(&new_id) {
                // if the list exsists, check to see if the service is already there
                let mut list = self.service_list_details.get(&new_id).unwrap_or_default();
                if list.items.contains(&service_id) {
                    // error, already on the list
                    return Err(Error::Duplicate)
                }
                else {
                    // add the service to the ServiceList
                    list.items.push(service_id);
                }
                let update = ServiceList {
                    owner: caller,
                    list_id: list.list_id,
                    list_name: list.list_name,
                    items: list.items
                };
                // update mappings
                self.service_list_details.insert(&new_id, &update);

            }
            else {
                // make a new list and put this service in the list
                
                let newlist = ServiceList {
                    owner: caller,
                    list_id: new_id,
                    list_name: list_name_clone1,
                    items: vec![service_id]
                };
                // update mappings
                // service_list_details: Mapping<Hash, ServiceList>
                self.service_list_details.insert(&new_id, &newlist);
                // account_service_lists: Mapping<AccountId, HashVector>
                my_lists.hashvector.push(new_id);
                self.account_service_lists.insert(&caller, &my_lists);
            }
            
            Ok(())
        }


        // 4 🟢 Bookmark A Store
        #[ink(message)]
        pub fn bookmark_a_store (&mut self, 
            seller: AccountId,
        ) -> Result<(), Error> {
            // set up clones
            // set up the caller
            let caller = Self::env().caller();
            // get the account_store_boookmarks list
            let mut my_list = self.account_store_bookmarks.get(&caller).unwrap_or_default();
            if my_list.accountvector.contains(&seller) {
                // if this seller is already there, error
                return Err(Error::Duplicate)
            }
            else {
                // if the seller is not there already, add them to the vector
                my_list.accountvector.push(seller);
                // update mapping account_store_bookmarks: Mapping<AccountId, AccountVector>
                self.account_store_bookmarks.insert(&caller, &my_list);

                Ok(())
            }
        }

        
        // 5 🟢 Remove Item From Cart
        #[ink(message)]
        pub fn remove_item_from_cart (&mut self, 
            item_id: Hash,
        ) -> Result<(), Error> {
            // set up the caller
            let caller = Self::env().caller();
            // get the caller's current unpaid cart id
            let mut cart = self.account_current_cart.get(&caller).unwrap_or_default();
            // get the quantity for that item in the cart
            let mut quantity: u128 = 0;
            for (item, number) in &cart.cart_items {
                if *item == item_id {
                    quantity = *number;
                }
            }
            // remove item from cart_items vector
            cart.cart_items.retain(|value| *value != (item_id, quantity));

            // get the total number of items in the cart
            let totalitems: u128 = cart.cart_items.len().try_into().unwrap();
            // iterate through the cart items to get the total price of the cart
            let mut carttotal: Balance = 0;
            for (item, number) in &cart.cart_items {
                // get the price for that item
                let mut item_price: Balance = 0;
                if self.product_details.contains(&item) {
                    item_price = self.product_details.get(&item).unwrap_or_default().price;
                }
                else {
                    if self.service_details.contains(&item) {
                        item_price = self.service_details.get(&item).unwrap_or_default().price;
                    }
                }
                // add the price to the total price
                carttotal += item_price * number;
            }
 
            // perpare the updated UnpaidCart
            let updated_cart = UnpaidCart {
                buyer: caller,
                cart_total: carttotal,
                total_items: totalitems,
                cart_items: cart.cart_items
            };
 
            // update mappings
            self.account_current_cart.insert(&caller, &updated_cart);           
            
            Ok(())
        }
        
    
        // 6 🟢 Update Cart Item Quantity
        #[ink(message)]
        pub fn update_cart_item_quantity (&mut self, 
            item_id: Hash,
            new_quantity: u128
        ) -> Result<(), Error> {
            // set up the caller
            let caller = Self::env().caller();
            // get the caller's current unpaid cart id
            let mut cart = self.account_current_cart.get(&caller).unwrap_or_default();
            // get the quantity for that item in the cart
            let mut quantity: u128 = 0;
            for (item, number) in &cart.cart_items {
                if *item == item_id {
                    quantity = *number;
                }
            }
            // remove item and old quantity from the cart_items vector
            cart.cart_items.retain(|value| *value != (item_id, quantity));
            // add the item with the new quantity to the cart_items vector
            cart.cart_items.push((item_id, new_quantity));

            // get the total number of items in the cart
            let totalitems: u128 = cart.cart_items.len().try_into().unwrap();
            // iterate through the cart items to get the total price of the cart
            let mut carttotal: Balance = 0;
            for (item, number) in &cart.cart_items {
                // get the price for that item
                let mut item_price: Balance = 0;
                if self.product_details.contains(&item) {
                    item_price = self.product_details.get(&item).unwrap_or_default().price;
                }
                else {
                    if self.service_details.contains(&item) {
                        item_price = self.service_details.get(&item).unwrap_or_default().price;
                    }
                }
                // add the price to the total price
                carttotal += item_price * number;
            }
 
            // perpare the updated UnpaidCart
            let updated_cart = UnpaidCart {
                buyer: caller,
                cart_total: carttotal,
                total_items: totalitems,
                cart_items: cart.cart_items
            };
 
            // update mappings
            self.account_current_cart.insert(&caller, &updated_cart);   
            
            Ok(())
        }

        
        // 7 🟢 Checkout Cart
        #[ink(message, payable)]
        #[openbrush::modifiers(non_reentrant)]
        pub fn checkout_cart (&mut self, 
            deliver_to_address: Vec<u8>,
            deliver_to_account: AccountId
        ) -> Result<(), Error> {
            // set up the caller and timestamp
            let caller = Self::env().caller();
            let rightnow = self.env().block_timestamp();

            // get the caller's unpaid cart
            let current_cart = self.account_current_cart.get(&caller).unwrap_or_default();

            // UPDATE THE CART TOTAL AND REMOVE ITEMS THAT DO NOT HAVE ENOUGH INVENTORY
            // make a new cart items vector to work with
            let mut final_cart_items = <Vec<(Hash, u128)>>::default();
            let mut item_inventory: u128 = 0;
            let mut item_price: Balance = 0;
            let mut carttotal: Balance = 0;
            // iterate through the cart to keep only items that have enough inventory
            for (item, number) in &current_cart.cart_items {
                // get the inventory and price for that item
                if self.product_details.contains(&item) {
                    item_inventory = self.product_details.get(&item).unwrap_or_default().inventory;
                    item_price = self.product_details.get(&item).unwrap_or_default().price;
                }
                else {
                    if self.service_details.contains(&item) {
                        item_inventory = self.service_details.get(&item).unwrap_or_default().inventory;
                        item_price = self.service_details.get(&item).unwrap_or_default().price;
                    }
                }
                // if the item has enough inventory, add it to the official cart items
                if item_inventory >= *number {
                    // add this item to the total price
                    carttotal += item_price * *number;
                    // add this item and quantity to the final cart items vector
                    final_cart_items.push((*item, *number));
                } 
            } 
            
            // COLLECT PAYMENT FROM THE CALLER
            // the 'payable' tag on this message allows the user to send any amount
            let amount_paid: Balance = self.env().transferred_value();
            if amount_paid < carttotal {
                // error, did not pay enough
                return Err(Error::InsufficientPayment);
            }
            else {
                // make the cart_id hash
                let encodable = (caller, rightnow); // Implements `scale::Encode`
                let mut new_id_u8 = <Sha2x256 as HashOutput>::Type::default(); // 256-bit buffer
                ink::env::hash_encoded::<Sha2x256, _>(&encodable, &mut new_id_u8);
                let new_cart_id: Hash = Hash::from(new_id_u8);

                // set up the vector of orders and items that will be referenced later
                let mut all_cart_orders = <Vec<Order>>::default();
                let mut all_cart_items = <Vec<Hash>>::default();
                let mut total_items_count = u128::default();
                let mut total_orders_count = u128::default();

                // update all_buyers: Vec<AcountId>
                if self.all_buyers.contains(&caller) {
                    // do nothing
                }
                else {
                    self.all_buyers.push(caller);
                }

                // FOR EACH ITEM IN THE CART ...
                for (item, number) in &final_cart_items {

                    // set up clones
                    let deliver_to_address_clone1 = deliver_to_address.clone();
                    
                    // CREATE THE ORDER STRUCT FOR THIS ITEM...

                    let mut item_seller: AccountId = ZERO_ADDRESS.into();
                    let mut item_seller_name: Vec<u8> = <Vec<u8>>::default();
                    let mut item_image:Vec<u8> = <Vec<u8>>::default();
                    let mut item_name: Vec<u8> = <Vec<u8>>::default();
                    let mut item_price: Balance = 0;
                    let mut item_zeno_percent: u128 = 0;
                    let mut item_is_digital: bool = false;
                    let mut item_is_service: bool = false;

                    // get the details for this item
                    if self.product_details.contains(item) {
                        let mut details = self.product_details.get(item).unwrap_or_default();
                        item_seller = details.seller_account;
                        item_seller_name = details.seller_name.clone();
                        item_image = details.photo_or_youtube_link1.clone();
                        item_name = details.title.clone();
                        item_price = details.price;
                        item_zeno_percent = details.zeno_percent;
                        item_is_digital = details.digital; 

                        // update zeno buyers information for the item...
                        // if there are fewer than 20 zeno buyers on the list
                        if details.zeno_buyers.len() < 20 {
                            // add this buyer to the zeno buyers list
                            details.zeno_buyers.push(caller);
                        }

                        // reduce the inventory on this item by the quantity bought
                        details.inventory -= *number;

                        // update the product details map
                        self.product_details.insert(item, &details);

                    }
                    else {
                        if self.service_details.contains(item) {
                            item_is_service = true;
                            let mut details = self.service_details.get(item).unwrap_or_default();
                            item_seller = details.seller_account;
                            item_seller_name = details.seller_name.clone();
                            item_image = details.photo_or_youtube_link1.clone();
                            item_name = details.title.clone();
                            item_price = details.price;
                            item_zeno_percent = details.zeno_percent;

                            // update zeno information for the item, 
                            // if there are fewer than 20 zeno buyers on the list
                            if details.zeno_buyers.len() < 20 {
                                // add this buyer to the zeno buyers list
                                details.zeno_buyers.push(caller);
                            }

                            // reduce the inventory on this item by the quantity bought
                            details.inventory -= *number;

                            // update the service details map
                            self.service_details.insert(item, &details);

                        }
                        else {
                            // error, item does not exist
                            return Err(Error::ItemDoesNotExist)
                        }
                    }

                    // make the order_id hash
                    let encodable = (caller, rightnow, item); // Implements `scale::Encode`
                    let mut new_id_u8 = <Sha2x256 as HashOutput>::Type::default(); // 256-bit buffer
                    ink::env::hash_encoded::<Sha2x256, _>(&encodable, &mut new_id_u8);
                    let new_order_id: Hash = Hash::from(new_id_u8);
                    
                    // calculate the order total
                    let item_order_total: Balance = number * item_price;

                    // calculate the zeno total
                    let item_zeno_total: Balance = item_order_total * item_zeno_percent / 100; 

                    // account for alternate order status when the product is digital
                    let mut status: u8 = 0;
                    if item_is_digital == true || item_is_service == true {
                        status = 2;
                    }

                    // set up the Order structure
                    let new_order = Order {
                        order_id: new_order_id,
                        cart_id: new_cart_id,
                        order_timestamp: rightnow,
                        buyer: caller,
                        seller: item_seller,
                        seller_name: item_seller_name,
                        image: item_image,
                        item_id: *item,
                        item_name: item_name,
                        quantity: *number,
                        price_each: item_price,
                        total_order_price: item_order_total,
                        deliver_to_address: deliver_to_address_clone1,
                        deliver_to_account: deliver_to_account,
                        tracking_info: <Vec<u8>>::default(),
                        order_status: status, 
                        time_delivered: u64::default(),
                        discussion: <Vec<Hash>>::default(),
                        problem: 0,
                        resolution: 0,
                        zeno_total: item_zeno_total
                    };

                    // update order_details: Mapping<Hash, Order>
                    self.order_details.insert(new_order_id, &new_order);
                    // update all_orders: Vec<Hash>
                    self.all_orders.push(new_order_id);

                    // update all_cart_orders, all_cart_items, total_items_count, and total_orders_count
                    all_cart_orders.push(new_order);
                    all_cart_items.push(*item);
                    total_items_count += number;
                    total_orders_count += 1;

                    // SPECIAL ACTIONS FOR DIGITAL PRODUCTS...
                    // if the item is a digital product, send ownership to the buyer and pay the seller
                    if item_is_digital == true {
                        // get this account's set of owned digital items
                        let mut owned = self.account_owned_digital_items.get(&caller).unwrap_or_default();
                        // is this item already in the owned list?
                        if owned.hashvector.contains(&item) {
                            // do nothing
                        }
                        else {
                            owned.hashvector.push(*item);
                            // update account_owned_digital_items: Mapping<AccountId, HashVector>
                            self.account_owned_digital_items.insert(&caller, &owned);
                        }
                        // payout the seller for the digital product
                        self.env().transfer(item_seller, item_order_total).expect("payout failed");
                        if self.env().transfer(item_seller, item_order_total).is_err() {
                            return Err(Error::PayoutFailed);
                        }
                    }

                    // PAYOUT SERVICES
                    if item_is_service == true {
                        // payout the seller for the service
                        self.env().transfer(item_seller, item_order_total).expect("payout failed");
                        if self.env().transfer(item_seller, item_order_total).is_err() {
                            return Err(Error::PayoutFailed);
                        }
                    }
                                        
                    // update account_buyer_orders: Mapping<AccountId, HashVector>
                    let mut buyer_orders = self.account_buyer_orders.get(&caller).unwrap_or_default();
                    buyer_orders.hashvector.push(new_order_id);
                    self.account_buyer_orders.insert(&caller, &buyer_orders);

                    // update account_buyer_items_bought: Mapping<AccountId, HashVector>
                    let mut buyer_items = self.account_buyer_items_bought.get(&caller).unwrap_or_default();
                    if buyer_items.hashvector.contains(&item) {
                        // do nothing
                    }
                    else {
                        buyer_items.hashvector.push(*item);
                        self.account_buyer_items_bought.insert(&caller, &buyer_items);
                    }

                    // update buyer_store_history: Mapping<AccountId, AccountVector>
                    let mut store_history = self.buyer_store_history.get(&caller).unwrap_or_default();
                    if store_history.accountvector.contains(&item_seller) {
                        // do nothing
                    }
                    else {
                        store_history.accountvector.push(item_seller);
                        self.buyer_store_history.insert(&caller, &store_history);
                    }

                    // update all_sellers: Vec<AccountId>
                    if self.all_sellers.contains(&item_seller) {
                        // do nothing
                    }
                    else {
                        self.all_sellers.push(item_seller);
                    }

                    // update account_seller_orders: Mapping<AccountId, HashVector>
                    let mut seller_orders = self.account_seller_orders.get(&item_seller).unwrap_or_default();
                    seller_orders.hashvector.push(new_order_id);
                    self.account_seller_orders.insert(&item_seller, &seller_orders);

                    // update account_profile_seller: Mapping<AccountId, SellerProfile>
                    let mut seller_profile = self.account_profile_seller.get(&item_seller).unwrap_or_default();
                    // increment total_orders
                    seller_profile.total_orders += 1;
                    if item_is_digital == true || item_is_service == true {
                        seller_profile.total_delivered += 1;
                    }
                    self.account_profile_seller.insert(&item_seller, &seller_profile);

                    // update account_profile_buyer: Mapping<AccountId, BuyerProfile>
                    let mut buyer_profile = self.account_profile_buyer.get(&caller).unwrap_or_default();
                    // increment total_orders
                    buyer_profile.total_orders += 1;
                    if item_is_digital == true || item_is_service == true {
                        buyer_profile.total_delivered += 1;
                    }
                    // if this is the first order, set the member_since timestamp
                    if buyer_profile.member_since == u64::default() {
                        buyer_profile.member_since = rightnow;
                    }
                    self.account_profile_buyer.insert(&caller, &buyer_profile);
                    
                    // update seller_customer_history: Mapping<AccountId, AccountVector>
                    let mut customer_history = self.seller_customer_history.get(&item_seller).unwrap_or_default();
                    if customer_history.accountvector.contains(&caller) {
                        // do nothing
                    }
                    else {
                        customer_history.accountvector.push(caller);
                        self.seller_customer_history.insert(&item_seller, &customer_history);
                    }

                    // EMIT EVENT to register the order to the chain
                    Self::env().emit_event(OrderPlaced {
                        order_id: new_order_id,
                        order_timestamp: rightnow,
                        buyer: caller,
                        seller: item_seller,
                        total_order_price: item_order_total,
                    });
                }

                // UPDATE CART RELATED STORAGE MAPPINGS...
                
                // update paid_cart_details: Mapping<Hash, PaidCart>
                // create a PaidCart structure
                let new_paid_cart = PaidCart {
                    cart_id: new_cart_id,
                    buyer: caller,
                    cart_timestamp: rightnow,
                    cart_total: carttotal,
                    deliver_to_address: deliver_to_address,
                    deliver_to_account: deliver_to_account,
                    orders: all_cart_orders, 
                    total_items: total_items_count,
                    cart_items: all_cart_items
                };
                // update the mapping
                self.paid_cart_details.insert(&new_cart_id, &new_paid_cart);

                // update account_paid_carts: Mapping<AccountId, HashVector>
                let mut my_carts = self.account_paid_carts.get(&caller).unwrap_or_default();
                my_carts.hashvector.push(new_cart_id);
                self.account_paid_carts.insert(&caller, &my_carts);

                // update account_profile_buyer: Mapping<AccountId, BuyerProfile>
                let mut buyer_profile = self.account_profile_buyer.get(&caller).unwrap_or_default();
                // increment total_carts and total_orders
                buyer_profile.total_carts += 1;
                self.account_profile_buyer.insert(&caller, &buyer_profile);

                // delete caller's unpaid cart in account_current_cart: Mapping<AccountId, UnpaidCart>
                self.account_current_cart.remove(&caller);

                Ok(())
            }
        }

        
        // 8 🟢 Rate A Product or Service
        #[ink(message)]
        pub fn rate_a_product_or_service (&mut self, 
            item_id: Hash,
            rating: u8,
            review: Vec<u8>
        ) -> Result<(), Error> {
            // if the rating is between 1 and 5
            if rating > 0 && rating < 6 {
                // set up the caller
                let caller = Self::env().caller();
                let now = self.env().block_timestamp();
                // account_buyer_items_bought: Mapping<AccountId, HashVector>
                let bought = self.account_buyer_items_bought.get(&caller).unwrap_or_default();
                // account_buyer_items_reviewed: Mapping<AccountId, HashVector>
                let mut reviewed = self.account_buyer_items_reviewed.get(&caller).unwrap_or_default();
                if bought.hashvector.contains(&item_id) {
                    // did you already review it?, if so, error
                    if reviewed.hashvector.contains(&item_id) {
                        return Err(Error::NotEligibleToReview)
                    }
                    else {
                        // make the review_id hash
                        let encodable = (caller, item_id); // Implements `scale::Encode`
                        let mut new_id_u8 = <Sha2x256 as HashOutput>::Type::default(); // 256-bit buffer
                        ink::env::hash_encoded::<Sha2x256, _>(&encodable, &mut new_id_u8);
                        let new_review_id: Hash = Hash::from(new_id_u8);

                        // make the ProductServiceReview structure
                        let review = ProductServiceReview {
                            review_id: new_review_id,
                            item_id: item_id,
                            reviewer: caller,
                            rating: rating,
                            review: review,
                            timestamp: now,
                        };

                        // update mappings...

                        // item_review_details: Mapping<Hash, ProductServiceReview>
                        self.item_review_details.insert(&new_review_id, &review);
                        // account_buyer_items_reviewed: Mapping<AccountId, HashVector>
                        reviewed.hashvector.push(item_id);
                        self.account_buyer_items_reviewed.insert(&caller, &reviewed);
                        
                        if self.product_details.contains(&item_id) {
                            // update product_details: Mapping<Hash, Product>
                            let mut details = self.product_details.get(&item_id).unwrap_or_default();
                            details.reviews.push(new_review_id);
                            self.product_details.insert(&item_id, &details);
                        }
                        else {
                            if self.service_details.contains(&item_id) {
                                // update service_details: Mapping<Hash, Service>
                                let mut details = self.service_details.get(&item_id).unwrap_or_default();
                                details.reviews.push(new_review_id);
                                self.service_details.insert(&item_id, &details);
                            }
                            else {
                                return Err(Error::ItemDoesNotExist)
                            }
                        }
                    }
                }
                else {
                    // if you did not buy it, error
                    return Err(Error::NotEligibleToReview)
                }
            }
            else {
                return Err(Error::RatingOutOfBounds)
            }
            Ok(())
        }
        

        // 9 🟢 Rate A Seller
        #[ink(message)]
        pub fn rate_a_seller (&mut self, 
            seller: AccountId,
            rating: u8,
            review: Vec<u8>
        ) -> Result<(), Error> {
            // if the rating is between 1 and 5
            if rating > 0 && rating < 6 {
                // set up the caller
                let caller = Self::env().caller();
                let now = self.env().block_timestamp();
                let review_clone = review.clone();

                // buyer_store_history: Mapping<AccountId, AccountVector>
                let sellers = self.buyer_store_history.get(&caller).unwrap_or_default();
                // account_buyer_sellers_reviewed: Mapping<AccountId, HashVector>
                let mut reviewed = self.account_buyer_sellers_reviewed.get(&caller).unwrap_or_default();

                // did you actually buy something from this seller?
                if sellers.accountvector.contains(&seller) {
                    // have you already reviewed this seller?
                    if reviewed.accountvector.contains(&seller) {
                        return Err(Error::NotEligibleToReview)
                    }
                    else {
                        // make the review_id hash
                        let encodable = (caller, seller, review); // Implements `scale::Encode`
                        let mut new_id_u8 = <Sha2x256 as HashOutput>::Type::default(); // 256-bit buffer
                        ink::env::hash_encoded::<Sha2x256, _>(&encodable, &mut new_id_u8);
                        let new_review_id: Hash = Hash::from(new_id_u8);

                        // make the BuyerSellerReview structure
                        let review = BuyerSellerReview {
                            review_id: new_review_id,
                            account_id: seller,
                            reviewer: caller,
                            rating: rating,
                            review: review_clone,
                            timestamp: now,
                        };

                        // update mappings...
                        // account_review_details: Mapping<Hash, BuyerSellerReview>
                        self.account_review_details.insert(&new_review_id, &review);
                        // account_buyer_sellers_reviewed: Mapping<AccountId, AccountVector>
                        reviewed.accountvector.push(seller);
                        self.account_buyer_sellers_reviewed.insert(&caller, &reviewed);
                        // account_profile_seller: Mapping<AccountId, SellerProfile>
                        let mut profile = self.account_profile_seller.get(&seller).unwrap_or_default();
                        profile.reviews.push(new_review_id);
                        self.account_profile_seller.insert(&seller, &profile);
                    }
                }
                else {
                    // if you did not buy from this seller, error
                    return Err(Error::NotEligibleToReview)
                }
            }
            else {
                return Err(Error::RatingOutOfBounds)
            }
            Ok(())
        }


        // 10 🟢 Report Problem Damaged
        #[ink(message)]
        pub fn report_problem_damaged (&mut self, 
            order_id: Hash,
            problem_photo_or_youtube_url: Vec<u8>,
            message: Vec<u8>,
        ) -> Result<(), Error> {
            // set up clones
            let message_clone = message.clone();
            // set up the caller
            let caller = Self::env().caller();
            // is this your order? account_buyer_orders: Mapping<AccountId, HashVector>
            let myorders = self.account_buyer_orders.get(&caller).unwrap_or_default();
            if myorders.hashvector.contains(&order_id) {
                // get the order details order_details: Mapping<Hash, Order>
                let mut details = self.order_details.get(&order_id).unwrap_or_default();
                let seller = details.seller;
                // Check the status. To report a problem, the status must be delivered
                // and it must have been less than 24 hours since it was marked delivered
                let now = self.env().block_timestamp();
                let time_since_delivered = now - details.time_delivered;
                if time_since_delivered < 86400000 && details.order_status == 2 {
                    // make the message_id hash
                    let encodable = (caller, now, order_id, message); // Implements `scale::Encode`
                    let mut new_id_u8 = <Sha2x256 as HashOutput>::Type::default(); // 256-bit buffer
                    ink::env::hash_encoded::<Sha2x256, _>(&encodable, &mut new_id_u8);
                    let new_message_id: Hash = Hash::from(new_id_u8);
                    // update order details
                    details.order_status = 4;
                    details.problem = 1;
                    details.discussion.push(new_message_id);
                    // update order_details: Mapping<Hash, Order>
                    self.order_details.insert(&order_id, &details);
                    // update message_details: Mapping<Hash, MessageDetails>
                    let message_details = MessageDetails {
                        message_id: new_message_id,
                        from_acct: caller,
                        to_acct: seller,
                        order_id: order_id,
                        message: message_clone,
                        media_url: problem_photo_or_youtube_url,
                        timestamp: now
                    };
                    self.message_details.insert(&new_message_id, &message_details);

                    // update Buyer profile
                    // account_profile_buyer: Mapping<AccountId, BuyerProfile>
                    let mut buyerprofile = self.account_profile_buyer.get(&caller).unwrap_or_default();
                    buyerprofile.total_damaged += 1;
                    buyerprofile.total_delivered -= 1;
                    self.account_profile_buyer.insert(&caller, &buyerprofile);

                    // update Seller profile
                    // account_profile_seller: Mapping<AccountId, SellerProfile>
                    let mut sellerprofile = self.account_profile_seller.get(&seller).unwrap_or_default();
                    sellerprofile.total_damaged += 1;
                    sellerprofile.total_delivered -= 1;
                    self.account_profile_seller.insert(&seller, &sellerprofile);

                }
                else {
                    return Err(Error::NotEligibleToReport)
                }
            }
            else {
                return Err(Error::NotYourOrder)
            }
            Ok(())
        }


        // 11 🟢 Report Problem Wrong Item
        #[ink(message)]
        pub fn report_problem_wrong_item (&mut self, 
            order_id: Hash,
            problem_photo_or_youtube_url: Vec<u8>,
            message: Vec<u8>,
        ) -> Result<(), Error> {
            // set up clones
            let message_clone = message.clone();
            // set up the caller
            let caller = Self::env().caller();
            // is this your order? account_buyer_orders: Mapping<AccountId, HashVector>
            let myorders = self.account_buyer_orders.get(&caller).unwrap_or_default();
            if myorders.hashvector.contains(&order_id) {
                // get the order details order_details: Mapping<Hash, Order>
                let mut details = self.order_details.get(&order_id).unwrap_or_default();
                let seller = details.seller;
                // Check the status. To report a problem, the status must be delivered
                // and it must have been less than 24 hours since it was marked delivered
                let now = self.env().block_timestamp();
                let time_since_delivered = now - details.time_delivered;
                if time_since_delivered < 86400000 && details.order_status == 2 {
                    // make the message_id hash
                    let encodable = (caller, now, order_id, message); // Implements `scale::Encode`
                    let mut new_id_u8 = <Sha2x256 as HashOutput>::Type::default(); // 256-bit buffer
                    ink::env::hash_encoded::<Sha2x256, _>(&encodable, &mut new_id_u8);
                    let new_message_id: Hash = Hash::from(new_id_u8);
                    // update order details
                    details.order_status = 4;
                    details.problem = 2;
                    details.discussion.push(new_message_id);
                    // update order_details: Mapping<Hash, Order>
                    self.order_details.insert(&order_id, &details);
                    // update message_details: Mapping<Hash, MessageDetails>
                    let message_details = MessageDetails {
                        message_id: new_message_id,
                        from_acct: caller,
                        to_acct: seller,
                        order_id: order_id,
                        message: message_clone,
                        media_url: problem_photo_or_youtube_url,
                        timestamp: now
                    };
                    self.message_details.insert(&new_message_id, &message_details);

                    // update Buyer profile
                    // account_profile_buyer: Mapping<AccountId, BuyerProfile>
                    let mut buyerprofile = self.account_profile_buyer.get(&caller).unwrap_or_default();
                    buyerprofile.total_wrong += 1;
                    buyerprofile.total_delivered -= 1;
                    self.account_profile_buyer.insert(&caller, &buyerprofile);

                    // update Seller profile
                    // account_profile_seller: Mapping<AccountId, SellerProfile>
                    let mut sellerprofile = self.account_profile_seller.get(&seller).unwrap_or_default();
                    sellerprofile.total_wrong += 1;
                    sellerprofile.total_delivered -= 1;
                    self.account_profile_seller.insert(&seller, &sellerprofile);

                }
                else {
                    return Err(Error::NotEligibleToReport)
                }
            }
            else {
                return Err(Error::NotYourOrder)
            }
            Ok(())
        }
        

        // 12 🟢 Report Problem Not Received
        #[ink(message)]
        pub fn report_problem_not_received (&mut self, 
            order_id: Hash,
            problem_photo_or_youtube_url: Vec<u8>,
            message: Vec<u8>,
        ) -> Result<(), Error> {
            // set up clones
            let message_clone = message.clone();
            // set up the caller
            let caller = Self::env().caller();
            // is this your order? account_buyer_orders: Mapping<AccountId, HashVector>
            let myorders = self.account_buyer_orders.get(&caller).unwrap_or_default();
            if myorders.hashvector.contains(&order_id) {
                // get the order details order_details: Mapping<Hash, Order>
                let mut details = self.order_details.get(&order_id).unwrap_or_default();
                let seller = details.seller;
                // Check the status. To report a problem, the status must be delivered
                // and it must have been less than 24 hours since it was marked delivered
                let now = self.env().block_timestamp();
                let time_since_delivered = now - details.time_delivered;
                if time_since_delivered < 86400000 && details.order_status == 2 {
                    // make the message_id hash
                    let encodable = (caller, now, order_id, message); // Implements `scale::Encode`
                    let mut new_id_u8 = <Sha2x256 as HashOutput>::Type::default(); // 256-bit buffer
                    ink::env::hash_encoded::<Sha2x256, _>(&encodable, &mut new_id_u8);
                    let new_message_id: Hash = Hash::from(new_id_u8);
                    // update order details
                    details.order_status = 4;
                    details.problem = 3;
                    details.discussion.push(new_message_id);
                    // update order_details: Mapping<Hash, Order>
                    self.order_details.insert(&order_id, &details);
                    // update message_details: Mapping<Hash, MessageDetails>
                    let message_details = MessageDetails {
                        message_id: new_message_id,
                        from_acct: caller,
                        to_acct: seller,
                        order_id: order_id,
                        message: message_clone,
                        media_url: problem_photo_or_youtube_url,
                        timestamp: now
                    };
                    self.message_details.insert(&new_message_id, &message_details);

                    // update Buyer profile
                    // account_profile_buyer: Mapping<AccountId, BuyerProfile>
                    let mut buyerprofile = self.account_profile_buyer.get(&caller).unwrap_or_default();
                    buyerprofile.total_not_received += 1;
                    buyerprofile.total_delivered -= 1;
                    self.account_profile_buyer.insert(&caller, &buyerprofile);

                    // update Seller profile
                    // account_profile_seller: Mapping<AccountId, SellerProfile>
                    let mut sellerprofile = self.account_profile_seller.get(&seller).unwrap_or_default();
                    sellerprofile.total_not_received += 1;
                    sellerprofile.total_delivered -= 1;
                    self.account_profile_seller.insert(&seller, &sellerprofile);

                }
                else {
                    return Err(Error::NotEligibleToReport)
                }
            }
            else {
                return Err(Error::NotYourOrder)
            }
            Ok(())
        }


        // 13 🟢 Message The Seller
        #[ink(message)]
        pub fn message_the_seller (&mut self, 
            order_id: Hash,
            photo_or_youtube_url: Vec<u8>,
            message: Vec<u8>,
        ) -> Result<(), Error> {
            // set up clones
            let message_clone = message.clone();
            // set up the caller
            let caller = Self::env().caller();
            // is this your order? account_buyer_orders: Mapping<AccountId, HashVector>
            let myorders = self.account_buyer_orders.get(&caller).unwrap_or_default();
            if myorders.hashvector.contains(&order_id) {
                // get the order details order_details: Mapping<Hash, Order>
                let mut details = self.order_details.get(&order_id).unwrap_or_default();
                let now = self.env().block_timestamp();
                // make the message_id hash
                let encodable = (caller, now, order_id, message); // Implements `scale::Encode`
                let mut new_id_u8 = <Sha2x256 as HashOutput>::Type::default(); // 256-bit buffer
                ink::env::hash_encoded::<Sha2x256, _>(&encodable, &mut new_id_u8);
                let new_message_id: Hash = Hash::from(new_id_u8);
                // update order discussion
                details.discussion.push(new_message_id);
                // update order_details: Mapping<Hash, Order>
                self.order_details.insert(&order_id, &details);
                // update message_details: Mapping<Hash, MessageDetails>
                let message_details = MessageDetails {
                    message_id: new_message_id,
                    from_acct: caller,
                    to_acct: details.seller,
                    order_id: order_id,
                    message: message_clone,
                    media_url: photo_or_youtube_url,
                    timestamp: now
                };
                self.message_details.insert(&new_message_id, &message_details);
            }
            else {
                return Err(Error::NotYourOrder)
            }
            Ok(())
        }


        // 14 🟢 Update Buyer Account Settings
        #[ink(message)]
        pub fn update_buyer_account_settings (&mut self, 
            name: Vec<u8>,
            location: Vec<u8>
        ) -> Result<(), Error> {
            // set up the caller
            let caller = Self::env().caller();
            // get the caller's Buyer Profile
            // account_profile_buyer: Mapping<AccountId, BuyerProfile>
            let mut profile = self.account_profile_buyer.get(&caller).unwrap_or_default();
            // update specific aspects of the BuyerProfile
            profile.buyer_name = name;
            profile.buyer_location = location;
            // update mappings
            self.account_profile_buyer.insert(&caller, &profile);
            Ok(())
        }
 
        
        // 15 🟢 Remove Product From List
        #[ink(message)]
        pub fn remove_product_from_list (&mut self, 
            item_id: Hash,
            list_id: Hash
        ) -> Result<(), Error> {
            // set up the caller
            let caller = Self::env().caller();
            // make sure the caller owns the list
            // account_product_lists: Mapping<AccountId, HashVector>
            let mylists = self.account_product_lists.get(&caller).unwrap_or_default();
            if mylists.hashvector.contains(&list_id) {
                // get the list details
                // product_list_details: Mapping<Hash, ProductList>
                let mut details = self.product_list_details.get(&list_id).unwrap_or_default();
                // keep all other items but this item
                details.items.retain(|value| *value != item_id);
                // update mappings
                self.product_list_details.insert(&list_id, &details);
            }
            else {
                return Err(Error::NotYourList)
            }
            Ok(())
        }


        // 16 🟢 Remove Service From List
        #[ink(message)]
        pub fn remove_service_from_list (&mut self, 
            item_id: Hash,
            list_id: Hash
        ) -> Result<(), Error> {
            // set up the caller
            let caller = Self::env().caller();
            // make sure the caller owns the list
            // account_service_lists: Mapping<AccountId, HashVector>
            let mylists = self.account_service_lists.get(&caller).unwrap_or_default();
            if mylists.hashvector.contains(&list_id) {
                // get the list details
                // service_list_details: Mapping<Hash, ServiceList>
                let mut details = self.service_list_details.get(&list_id).unwrap_or_default();
                // keep all other items but this item
                details.items.retain(|value| *value != item_id);
                // update mappings
                self.service_list_details.insert(&list_id, &details);
            }
            else {
                return Err(Error::NotYourList)
            }
            Ok(())
        }


        // 17 🟢 Delete Product List
        #[ink(message)]
        pub fn delete_product_list (&mut self, 
            list_id: Hash
        ) -> Result<(), Error> {
            // set up the caller
            let caller = Self::env().caller();
            // make sure the caller owns the list
            // account_product_lists: Mapping<AccountId, HashVector>
            let mut mylists = self.account_product_lists.get(&caller).unwrap_or_default();
            if mylists.hashvector.contains(&list_id) {
                // remove the list from product_list_details: Mapping<Hash, ProdcutList>
                self.product_list_details.remove(&list_id);
                // remove the list from account_product_lists: Mapping<AccountId, HashVector>
                mylists.hashvector.retain(|value| *value != list_id);
                self.account_product_lists.insert(&caller, &mylists);
            }
            else {
                return Err(Error::NotYourList)
            }
            Ok(())
        }


        // 18 🟢 Delete Service List
        #[ink(message)]
        pub fn delete_service_list (&mut self, 
            list_id: Hash
        ) -> Result<(), Error> {
            // set up the caller
            let caller = Self::env().caller();
            // make sure the caller owns the list
            // account_service_lists: Mapping<AccountId, HashVector>
            let mut mylists = self.account_service_lists.get(&caller).unwrap_or_default();
            if mylists.hashvector.contains(&list_id) {
                // remove the list from service_list_details: Mapping<Hash, ServiceList>
                self.service_list_details.remove(&list_id);
                // remove the list from account_service_lists: Mapping<AccountId, HashVector>
                mylists.hashvector.retain(|value| *value != list_id);
                self.account_service_lists.insert(&caller, &mylists);
            }
            else {
                return Err(Error::NotYourList)
            }
            Ok(())
        }


        // 19 🟢 Update Seller Account Settings
        #[ink(message)]
        pub fn update_seller_account_settings (&mut self, 
            name: Vec<u8>,
            location: Vec<u8>,
            description: Vec<u8>,
            banner_url: Vec<u8>,
            youtube_url: Vec<u8>,
            external_link: Vec<u8>
        ) -> Result<(), Error> {
            // set up the caller
            let caller = Self::env().caller();
            // get the caller's Seller Profile
            // account_profile_seller: Mapping<AccountId, SellerProfile>
            let mut profile = self.account_profile_seller.get(&caller).unwrap_or_default();
            // update specific aspects of the SellerProfile
            profile.seller_name = name;
            profile.seller_location = location;
            profile.store_description = description;
            profile.banner_url = banner_url;
            profile.youtube_url = youtube_url;
            profile.external_link = external_link;
            // update mappings
            self.account_profile_seller.insert(&caller, &profile);
            Ok(())
        }


        // 20 🟢 Update Order Tracking Information 
        #[ink(message)]
        #[openbrush::modifiers(non_reentrant)]
        pub fn update_order_tracking_information (&mut self, 
            order_id: Hash,
            tracking_update: Vec<u8>,
            shipped: bool,
            delivered: bool
        ) -> Result<(), Error> {
            // set up the caller
            let caller = Self::env().caller();
            // make sure the caller is the seller on this order
            // account_seller_orders: Mapping<AccountId, HashVector>
            let myorders = self.account_seller_orders.get(&caller).unwrap_or_default();
            if myorders.hashvector.contains(&order_id) {
                // make sure the order is a physical product (not a service, or digital product)
                let mut details = self.order_details.get(&order_id).unwrap_or_default();
                let itemid = details.item_id;
                if self.product_details.contains(&itemid) {
                    let item = self.product_details.get(&itemid).unwrap_or_default();
                    if item.digital == false {

                        details.tracking_info = tracking_update;

                        if details.order_status == 1 {
                            // seller can mark delivered, or can leave it as shipped
                            if delivered == true {
                                details.order_status = 2;
                                details.time_delivered = self.env().block_timestamp();

                                // update Buyer profile
                                // account_profile_buyer: Mapping<AccountId, BuyerProfile>
                                let buyer = details.buyer;
                                let mut buyerprofile = self.account_profile_buyer.get(&buyer).unwrap_or_default();
                                buyerprofile.total_delivered += 1;
                                self.account_profile_buyer.insert(&buyer, &buyerprofile);

                                // update Seller profile
                                // account_profile_seller: Mapping<AccountId, SellerProfile>
                                let mut sellerprofile = self.account_profile_seller.get(&caller).unwrap_or_default();
                                sellerprofile.total_delivered += 1;
                                self.account_profile_seller.insert(&caller, &sellerprofile);
                                
                            }
                        }

                        if details.order_status == 0 {
                            // seller can mark shipped but not delivered
                            if shipped == true {
                                details.order_status = 1;

                                // calculate payments to seller and zeno buyers
                                let seller = details.seller;
                                let total_price: Balance = details.total_order_price;
                                let zeno_total: Balance = details.zeno_total;
                                let zeno_buyers = item.zeno_buyers;
                                let seller_payout: Balance = total_price - zeno_total;
                                let length = zeno_buyers.len();

                                // pay the seller 
                                self.env().transfer(seller, seller_payout).expect("payout failed");
                                if self.env().transfer(seller, seller_payout).is_err() {
                                    return Err(Error::PayoutFailed);
                                }

                                // If the zeno_total is not zero, initiate the payouts
                                if zeno_total > 0 {
                                    // pay all zeno buyers
                                    let mut remainder: Balance = zeno_total;
                                    for n in 0..(length - 1) {
                                        let affiliate: AccountId = zeno_buyers[n];
                                        let m: u128 = n.try_into().unwrap();
                                        let payment: Balance = zeno_total / 2^(m + 1);
                                        self.env().transfer(affiliate, payment).expect("payout failed");
                                        if self.env().transfer(affiliate, payment).is_err() {
                                            return Err(Error::PayoutFailed);
                                        }
                                        remainder -= payment;
                                    }
                                    // pay the seller any remainder from the zeno payouts
                                    self.env().transfer(seller, remainder).expect("payout failed");
                                    if self.env().transfer(seller, remainder).is_err() {
                                        return Err(Error::PayoutFailed);
                                    }
                                }
                                
                            }
                        }
                       
                        // update order_details: Mapping<Hash, Order> 
                        self.order_details.insert(&order_id, &details);

                    }
                    else {
                        return Err(Error::NotAPhysicalProduct)
                    }
                }
                else {
                    return Err(Error::NotAPhysicalProduct)
                }                
            }
            else {
                return Err(Error::NotYourOrder)
            }
            Ok(())
        }


        // 21 🟢 Refuse An Order
        #[ink(message)]
        #[openbrush::modifiers(non_reentrant)]
        pub fn refuse_an_order (&mut self, 
            order_id: Hash
        ) -> Result<(), Error> {
            // set up the caller
            let caller = Self::env().caller();
            // make sure the caller is the seller on this order
            // account_seller_orders: Mapping<AccountId, HashVector>
            let myorders = self.account_seller_orders.get(&caller).unwrap_or_default();
            if myorders.hashvector.contains(&order_id) {
                // you can only refuse an order that has not yet shipped. Status must be 0.
                let mut details = self.order_details.get(&order_id).unwrap_or_default();
                if details.order_status == 0 {
                    
                    // issue a refund to the buyer for this order
                    let buyer = details.buyer;
                    let refund: Balance = details.total_order_price;
                    self.env().transfer(buyer, refund).expect("payout failed");
                    if self.env().transfer(buyer, refund).is_err() {
                        return Err(Error::PayoutFailed);
                    }

                    // update order_details: Mapping<Hash, Order>
                    details.order_status = 5;
                    self.order_details.insert(&order_id, &details);

                    // update Buyer profile
                    // account_profile_buyer: Mapping<AccountId, BuyerProfile>
                    let mut buyerprofile = self.account_profile_buyer.get(&buyer).unwrap_or_default();
                    buyerprofile.total_refused += 1;
                    self.account_profile_buyer.insert(&buyer, &buyerprofile);

                    // update Seller profile
                    // account_profile_seller: Mapping<AccountId, SellerProfile>
                    let mut sellerprofile = self.account_profile_seller.get(&caller).unwrap_or_default();
                    sellerprofile.total_refused += 1;
                    self.account_profile_seller.insert(&caller, &sellerprofile);

                }
                else {
                    return Err(Error::CannotRefuse)
                }
            }
            else {
                return Err(Error::NotYourOrder)
            }
            Ok(())
        }


        // 22 🟢 Issue Refund
        // note that refunds are issued as a resolution to a problem
        // refunds do not include any zeno percent paid when the item shipped
        #[ink(message)]
        #[openbrush::modifiers(non_reentrant)]
        pub fn issue_refund (&mut self, 
            order_id: Hash,
        ) -> Result<(), Error> {
            // set up the caller
            let caller = Self::env().caller();

            // make sure the caller is the seller on this order
            // account_seller_orders: Mapping<AccountId, HashVector>
            let myorders = self.account_seller_orders.get(&caller).unwrap_or_default();
            if myorders.hashvector.contains(&order_id) {
                // you can only refund an order that has a problem and no resolution. 
                // Status must be 4 and resolution must be 0.
                let mut details = self.order_details.get(&order_id).unwrap_or_default();
                if details.order_status == 4 && details.resolution == 0 {
                    
                    // issue a refund to the buyer for this order
                    let buyer = details.buyer;
                    let refund: Balance = details.total_order_price - details.zeno_total;
                    self.env().transfer(buyer, refund).expect("payout failed");
                    if self.env().transfer(buyer, refund).is_err() {
                        return Err(Error::PayoutFailed);
                    }

                    // update order_details: Mapping<Hash, Order>
                    details.resolution = 1;
                    self.order_details.insert(&order_id, &details);

                    // update Buyer profile
                    // account_profile_buyer: Mapping<AccountId, BuyerProfile>
                    let mut buyerprofile = self.account_profile_buyer.get(&buyer).unwrap_or_default();
                    buyerprofile.total_resolved += 1;
                    self.account_profile_buyer.insert(&buyer, &buyerprofile);

                    // update Seller profile
                    // account_profile_seller: Mapping<AccountId, SellerProfile>
                    let mut sellerprofile = self.account_profile_seller.get(&caller).unwrap_or_default();
                    sellerprofile.total_resolved += 1;
                    self.account_profile_seller.insert(&caller, &sellerprofile);

                }
                else {
                    return Err(Error::CannotResolve)
                }
            }
            else {
                return Err(Error::NotYourOrder)
            }
            
            Ok(())
        }
        
        
        // 23 🟢 Issue Replacement
        #[ink(message)]
        pub fn issue_replacement (&mut self, 
            order_id: Hash,
            tracking: Vec<u8>
        ) -> Result<(), Error> {
            // set up the caller
            let caller = Self::env().caller();

            // make sure the caller is the seller on this order
            // account_seller_orders: Mapping<AccountId, HashVector>
            let myorders = self.account_seller_orders.get(&caller).unwrap_or_default();
            if myorders.hashvector.contains(&order_id) {
                // you can only replace an order that has a problem. Status must be 4.
                let mut details = self.order_details.get(&order_id).unwrap_or_default();
                if details.order_status == 4 && details.resolution == 0 {
                    
                    // update order_details: Mapping<Hash, Order>
                    // update the shipping details with the replacement item
                    details.tracking_info = tracking;
                    // update the resolution code
                    details.resolution = 2;
                    // update the mapping
                    self.order_details.insert(&order_id, &details);

                    // update Buyer profile
                    let buyer = details.buyer;
                    // account_profile_buyer: Mapping<AccountId, BuyerProfile>
                    let mut buyerprofile = self.account_profile_buyer.get(&buyer).unwrap_or_default();
                    buyerprofile.total_resolved += 1;
                    self.account_profile_buyer.insert(&buyer, &buyerprofile);

                    // update Seller profile
                    // account_profile_seller: Mapping<AccountId, SellerProfile>
                    let mut sellerprofile = self.account_profile_seller.get(&caller).unwrap_or_default();
                    sellerprofile.total_resolved += 1;
                    self.account_profile_seller.insert(&caller, &sellerprofile);

                }
                else {
                    return Err(Error::CannotResolve)
                }
            }
            else {
                return Err(Error::NotYourOrder)
            }
            
            Ok(())
        }
       
       
        // 24 🟢 Deny Resolution Request
        #[ink(message)]
        pub fn deny_resolution_request (&mut self, 
            order_id: Hash,
        ) -> Result<(), Error> {
            // set up the caller
            let caller = Self::env().caller();

            // make sure the caller is the seller on this order
            // account_seller_orders: Mapping<AccountId, HashVector>
            let myorders = self.account_seller_orders.get(&caller).unwrap_or_default();
            if myorders.hashvector.contains(&order_id) {
                // you can only deny resolution on an order that has a problem. Status must be 4.
                let mut details = self.order_details.get(&order_id).unwrap_or_default();
                if details.order_status == 4 && details.resolution == 0 {
                    
                    // update order_details: Mapping<Hash, Order>
                    details.resolution = 3;
                    self.order_details.insert(&order_id, &details);

                }
                else {
                    return Err(Error::CannotResolve)
                }
            }
            else {
                return Err(Error::NotYourOrder)
            }
            
            Ok(())
        }
        
        
        // 25 🟢 Message The Buyer
        #[ink(message)]
        pub fn message_the_buyer (&mut self, 
            order_id: Hash,
            photo_or_youtube_url: Vec<u8>,
            message: Vec<u8>,
        ) -> Result<(), Error> {
            // set up clones
            let message_clone = message.clone();
            // set up the caller
            let caller = Self::env().caller();
            // is this your order? account_seller_orders: Mapping<AccountId, HashVector>
            let myorders = self.account_seller_orders.get(&caller).unwrap_or_default();
            if myorders.hashvector.contains(&order_id) {
                // get the order details from order_details: Mapping<Hash, Order>
                let mut details = self.order_details.get(&order_id).unwrap_or_default();
                let now = self.env().block_timestamp();
                // make the message_id hash
                let encodable = (caller, now, order_id, message); // Implements `scale::Encode`
                let mut new_id_u8 = <Sha2x256 as HashOutput>::Type::default(); // 256-bit buffer
                ink::env::hash_encoded::<Sha2x256, _>(&encodable, &mut new_id_u8);
                let new_message_id: Hash = Hash::from(new_id_u8);
                // update order discussion
                details.discussion.push(new_message_id);
                // update order_details: Mapping<Hash, Order>
                self.order_details.insert(&order_id, &details);
                // update message_details: Mapping<Hash, MessageDetails>
                let message_details = MessageDetails {
                    message_id: new_message_id,
                    from_acct: caller,
                    to_acct: details.buyer,
                    order_id: order_id,
                    message: message_clone,
                    media_url: photo_or_youtube_url,
                    timestamp: now
                };
                self.message_details.insert(&new_message_id, &message_details);
            }
            else {
                return Err(Error::NotYourOrder)
            }
            Ok(())
        }


        // 26 🟢 Rate A Buyer
        #[ink(message)]
        pub fn rate_a_buyer (&mut self, 
            buyer: AccountId,
            rating: u8,
            review: Vec<u8>
        ) -> Result<(), Error> {
            // if the rating is between 1 and 5
            if rating > 0 && rating < 6 {
                let caller = Self::env().caller();
                let review_clone = review.clone();
                let now = self.env().block_timestamp();
                
                // seller_customer_history: Mapping<AccountId, AccountVector>
                let buyers = self.seller_customer_history.get(&caller).unwrap_or_default();
                // account_seller_buyers_reviewed: Mapping<AccountId, AccountVector>
                let mut reviewed = self.account_seller_buyers_reviewed.get(&caller).unwrap_or_default();

                // did you actually sell something to this buyer?
                if buyers.accountvector.contains(&buyer) {
                    // have you already reviewed this buyer?
                    if reviewed.accountvector.contains(&buyer) {
                        return Err(Error::NotEligibleToReview)
                    }
                    else {
                        // make the review_id hash
                        let encodable = (caller, buyer, review); // Implements `scale::Encode`
                        let mut new_id_u8 = <Sha2x256 as HashOutput>::Type::default(); // 256-bit buffer
                        ink::env::hash_encoded::<Sha2x256, _>(&encodable, &mut new_id_u8);
                        let new_review_id: Hash = Hash::from(new_id_u8);

                        // make the BuyerSellerReview structure
                        let review = BuyerSellerReview {
                            review_id: new_review_id,
                            account_id: buyer,
                            reviewer: caller,
                            rating: rating,
                            review: review_clone,
                            timestamp: now,
                        };

                        // update mappings...
                        // account_review_details: Mapping<Hash, BuyerSellerReview>
                        self.account_review_details.insert(&new_review_id, &review);
                        // account_seller_buyers_reviewed: Mapping<AccountId, AccountVector>
                        reviewed.accountvector.push(buyer);
                        self.account_seller_buyers_reviewed.insert(&caller, &reviewed);
                        // account_profile_buyer: Mapping<AccountId, BuyerProfile>
                        let mut profile = self.account_profile_buyer.get(&buyer).unwrap_or_default();
                        profile.reviews.push(new_review_id);
                        self.account_profile_buyer.insert(&buyer, &profile);
                    }
                }
                else {
                    // if you did not buy from this seller, error
                    return Err(Error::NotEligibleToReview)
                }
            }
            else {
                return Err(Error::RatingOutOfBounds)
            }
            Ok(())
        }
        
        
        // 27 🟢 Add A Product
        #[ink(message)]
        #[openbrush::modifiers(non_reentrant)]
        pub fn add_a_product (&mut self, 
            digital: bool,
            title: Vec<u8>,
            price: Balance,
            brand: Vec<u8>,
            category: Vec<u8>,
            description: Vec<u8>, 
            inventory: u128, 
            photo_or_youtube_link1: Vec<u8>, 
            photo_or_youtube_link2: Vec<u8>,
            photo_or_youtube_link3: Vec<u8>,
            more_info_link: Vec<u8>,
            delivery_info: Vec<u8>,
            product_location: Vec<u8>,
            digital_file_url: Vec<u8>,
            zeno_percent: u128,
        ) -> Result<(), Error> {
            // set up clones
            let title_clone = title.clone();
            // set up the caller
            let caller = Self::env().caller();
            let now = self.env().block_timestamp();

            // make the product id hash
            let encodable = (caller, title, now); // Implements `scale::Encode`
            let mut new_id_u8 = <Sha2x256 as HashOutput>::Type::default(); // 256-bit buffer
            ink::env::hash_encoded::<Sha2x256, _>(&encodable, &mut new_id_u8);
            let new_product_id: Hash = Hash::from(new_id_u8);

            // get the seller profile 
            let sellerprofile = self.account_profile_seller.get(&caller).unwrap_or_default();

            // set up the product details
            let product = Product {
                product_id: new_product_id,
                digital: digital,
                title: title_clone,
                price: price,
                brand: brand,
                category: category,
                seller_account: caller,
                seller_name: sellerprofile.seller_name,
                description: description, 
                reviews: <Vec<Hash>>::default(),
                inventory: inventory, 
                photo_or_youtube_link1: photo_or_youtube_link1, 
                photo_or_youtube_link2: photo_or_youtube_link2,
                photo_or_youtube_link3: photo_or_youtube_link3,
                more_info_link: more_info_link,
                delivery_info: delivery_info,
                product_location: product_location,
                digital_file_url: digital_file_url,
                zeno_percent: zeno_percent,
                zeno_buyers: <Vec<AccountId>>::default(),
            };

            // update mappings...
            // all_products: Vec<Hash>
            self.all_products.push(new_product_id);
            // account_seller_products: Mapping<AccountId, HashVector>
            let mut seller_products = self.account_seller_products.get(&caller).unwrap_or_default();
            seller_products.hashvector.push(new_product_id);
            self.account_seller_products.insert(&caller, &seller_products);
            // product_details: Mapping<Hash, Product>
            self.product_details.insert(&new_product_id, &product);
            
            Ok(())
        }

        
        // 28 🟢 Update Product Details
        #[ink(message)]
        pub fn update_product_details (&mut self,
            product_id: Hash, 
            title: Vec<u8>,
            price: Balance,
            brand: Vec<u8>,
            category: Vec<u8>,
            description: Vec<u8>, 
            inventory: u128, 
            photo_or_youtube_link1: Vec<u8>, 
            photo_or_youtube_link2: Vec<u8>,
            photo_or_youtube_link3: Vec<u8>,
            more_info_link: Vec<u8>,
            delivery_info: Vec<u8>,
            product_location: Vec<u8>,
            digital_file_url: Vec<u8>,
        ) -> Result<(), Error> {
            // set up the caller
            let caller = Self::env().caller();

            // is this your product? Check the product id
            // account_seller_products: Mapping<AccountId, HashVector>
            let seller_products = self.account_seller_products.get(&caller).unwrap_or_default();
            if seller_products.hashvector.contains(&product_id) {
                
                // get the current product details
                let details = self.product_details.get(&product_id).unwrap_or_default();

                // set up the product details update
                let update = Product {
                    product_id: product_id,
                    digital: details.digital,
                    title: title,
                    price: price,
                    brand: brand,
                    category: category,
                    seller_account: caller,
                    seller_name: details.seller_name,
                    description: description, 
                    reviews: details.reviews,
                    inventory: inventory, 
                    photo_or_youtube_link1: photo_or_youtube_link1, 
                    photo_or_youtube_link2: photo_or_youtube_link2,
                    photo_or_youtube_link3: photo_or_youtube_link3,
                    more_info_link: more_info_link,
                    delivery_info: delivery_info,
                    product_location: product_location,
                    digital_file_url: digital_file_url,
                    zeno_percent: details.zeno_percent,
                    zeno_buyers: details.zeno_buyers,
                };

                // update product_details: Mapping<Hash, Product>
                self.product_details.insert(&product_id, &update);

            }
            
            else {
                return Err(Error::NotYourProduct)
            }

            Ok(())
        }
        

        // 29 🟢 Add A Service
        #[ink(message)]
        #[openbrush::modifiers(non_reentrant)]
        pub fn add_a_service (&mut self, 
            online: bool,
            title: Vec<u8>,
            price: Balance,
            category: Vec<u8>,
            description: Vec<u8>,  
            inventory: u128,
            photo_or_youtube_link1: Vec<u8>, 
            photo_or_youtube_link2: Vec<u8>,
            photo_or_youtube_link3: Vec<u8>,
            booking_link: Vec<u8>,
            service_location: Vec<u8>,
            zeno_percent: u128,
        ) -> Result<(), Error> {
            // set up clones
            let title_clone = title.clone();
            // set up the caller
            let caller = Self::env().caller();
            let now = self.env().block_timestamp();

            // make the product id hash
            let encodable = (caller, title, now); // Implements `scale::Encode`
            let mut new_id_u8 = <Sha2x256 as HashOutput>::Type::default(); // 256-bit buffer
            ink::env::hash_encoded::<Sha2x256, _>(&encodable, &mut new_id_u8);
            let new_service_id: Hash = Hash::from(new_id_u8);

            // get the seller profile 
            let sellerprofile = self.account_profile_seller.get(&caller).unwrap_or_default();

            // set up the service details
            let service = Service {
                service_id: new_service_id,
                online: online,
                title: title_clone,
                price: price,
                category: category,
                seller_account: caller,
                seller_name: sellerprofile.seller_name,
                description: description, 
                reviews: <Vec<Hash>>::default(),
                inventory: inventory,
                photo_or_youtube_link1: photo_or_youtube_link1, 
                photo_or_youtube_link2: photo_or_youtube_link2,
                photo_or_youtube_link3: photo_or_youtube_link3,
                booking_link: booking_link,
                service_location: service_location,
                zeno_percent: zeno_percent,
                zeno_buyers: <Vec<AccountId>>::default(),
            };

            // update mappings...
            // all_services: Vec<Hash>
            self.all_services.push(new_service_id);
            // account_seller_services: Mapping<AccountId, HashVector>
            let mut seller_services = self.account_seller_services.get(&caller).unwrap_or_default();
            seller_services.hashvector.push(new_service_id);
            self.account_seller_services.insert(&caller, &seller_services);
            // service_details: Mapping<Hash, Service>
            self.service_details.insert(&new_service_id, &service);
            
            Ok(())
        }
        

        // 30 🟢 Update Service Details
        #[ink(message)]
        pub fn update_service_details (&mut self,
            service_id: Hash, 
            title: Vec<u8>,
            price: Balance,
            category: Vec<u8>,
            description: Vec<u8>, 
            inventory: u128, 
            photo_or_youtube_link1: Vec<u8>, 
            photo_or_youtube_link2: Vec<u8>,
            photo_or_youtube_link3: Vec<u8>,
            booking_link: Vec<u8>,
            service_location: Vec<u8>,
        ) -> Result<(), Error> {
            // set up the caller
            let caller = Self::env().caller();

            // is this your service? Check the service id
            // account_seller_services: Mapping<AccountId, HashVector>
            let seller_services = self.account_seller_services.get(&caller).unwrap_or_default();
            if seller_services.hashvector.contains(&service_id) {
                
                // get the current service details
                let details = self.service_details.get(&service_id).unwrap_or_default();

                // set up the service details update
                let update = Service {
                    service_id: service_id,
                    online: details.online,
                    title: title,
                    price: price,
                    category: category,
                    seller_account: caller,
                    seller_name: details.seller_name,
                    description: description, 
                    reviews: details.reviews,
                    inventory: inventory,
                    photo_or_youtube_link1: photo_or_youtube_link1, 
                    photo_or_youtube_link2: photo_or_youtube_link2,
                    photo_or_youtube_link3: photo_or_youtube_link3,
                    booking_link: booking_link,
                    service_location: service_location,
                    zeno_percent: details.zeno_percent,
                    zeno_buyers: details.zeno_buyers,
                };

                // update service_details: Mapping<Hash, Service>
                self.service_details.insert(&service_id, &update);

            }
            else {
                return Err(Error::NotYourProduct)
            }

            Ok(())
        }


        // >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
        // >>>>>>>>>>>>>>>>>>>>>>>>>> PRIMARY GET MESSAGES <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<
        // >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
 

        // 31 🟢 Search Products By Keyword
        #[ink(message)]
        pub fn search_products_by_keyword (&self,
            keywords: Vec<u8>
        ) -> ProductSearchResults {

            // set up return structures
            let mut product_results = <Vec<Product>>::default();

            // iterate over all_products: Vec<Hash> to find matching results
            for item in self.all_products.iter() {
                // get the details
                let details = self.product_details.get(item).unwrap_or_default();
                // check to see if the keywords are there
                let title_string = String::from_utf8(details.title.clone()).unwrap_or_default();
                let seller_string = String::from_utf8(details.seller_name.clone()).unwrap_or_default();
                let brand_string = String::from_utf8(details.brand.clone()).unwrap_or_default();
                let category_string = String::from_utf8(details.category.clone()).unwrap_or_default();
                let description_string = String::from_utf8(details.description.clone()).unwrap_or_default();
                let delivery_string = String::from_utf8(details.delivery_info.clone()).unwrap_or_default();
                let location_string = String::from_utf8(details.product_location.clone()).unwrap_or_default();

                let targetvecu8 = keywords.clone();
                let target_string = String::from_utf8(targetvecu8).unwrap_or_default();
                // if the target_string is in the details
                if title_string.contains(&target_string) || brand_string.contains(&target_string) ||
                category_string.contains(&target_string) || description_string.contains(&target_string) ||
                delivery_string.contains(&target_string) || location_string.contains(&target_string) ||
                seller_string.contains(&target_string) {
                    // add it to the results vector
                    product_results.push(details);
                }
                //continue iterating
            }

            // package the results
            let results = ProductSearchResults {
                search: keywords,
                products: product_results
            };

            // return the results
            results
        }


        // 32 🟢 Search Services By Keyword
        #[ink(message)]
        pub fn search_services_by_keyword (&self,
            keywords: Vec<u8>
        ) -> ServiceSearchResults {

            // set up return structures
            let mut service_results = <Vec<Service>>::default();

            // iterate over all_services: Vec<Hash> to find matching results
            for item in self.all_services.iter() {
                // get the details
                let details = self.service_details.get(item).unwrap_or_default();
                // check to see if the keywords are there
                let title_string = String::from_utf8(details.title.clone()).unwrap_or_default();
                let seller_string = String::from_utf8(details.seller_name.clone()).unwrap_or_default();
                let category_string = String::from_utf8(details.category.clone()).unwrap_or_default();
                let description_string = String::from_utf8(details.description.clone()).unwrap_or_default();
                let location_string = String::from_utf8(details.service_location.clone()).unwrap_or_default();

                let targetvecu8 = keywords.clone();
                let target_string = String::from_utf8(targetvecu8).unwrap_or_default();
                // if the target_string is in the details
                if title_string.contains(&target_string) || seller_string.contains(&target_string) ||
                category_string.contains(&target_string) || description_string.contains(&target_string) ||
                location_string.contains(&target_string) {
                    // add it to the results vector
                    service_results.push(details);
                }
                //continue iterating
            }

            // package the results
            let results = ServiceSearchResults {
                search: keywords,
                services: service_results
            };

            // return the results
            results
        }
        
        
        // 33 🟢 Search Stores by Keyword
        #[ink(message)]
        pub fn search_stores_by_keyword (&self,
            keywords: Vec<u8>
        ) -> StoreSearchResults {

            // set up return structures
            let mut store_results = <Vec<SellerProfile>>::default();

            // iterate over all_sellers: Vec<AccountId> to find matching results
            for acct in self.all_sellers.iter() {
                // get the details
                let details = self.account_profile_seller.get(acct).unwrap_or_default();
                // check to see if the keywords are there
                let name_string = String::from_utf8(details.seller_name.clone()).unwrap_or_default();
                let description_string = String::from_utf8(details.store_description.clone()).unwrap_or_default();
                let location_string = String::from_utf8(details.seller_location.clone()).unwrap_or_default();

                let targetvecu8 = keywords.clone();
                let target_string = String::from_utf8(targetvecu8).unwrap_or_default();
                // if the target_string is in the details
                if name_string.contains(&target_string) || description_string.contains(&target_string) ||
                location_string.contains(&target_string) {
                    // add it to the results vector
                    store_results.push(details);
                }
                //continue iterating
            }

            // package the results
            let results = StoreSearchResults {
                search: keywords,
                stores: store_results
            };

            // return the results
            results
        }


        // 34 🟢 View My Orders
        #[ink(message)]
        pub fn view_my_orders (&self) -> ViewBuyerOrders {
            // set the caller
            let caller = Self::env().caller();
            // set up return structures
            let mut my_carts = <Vec<PaidCart>>::default();
            // get paid cart ids from account_paid_carts: Mapping<AccountId, HashVector>
            let paid_cart_ids = self.account_paid_carts.get(&caller).unwrap_or_default();
            // iterate over all cart ids to get the PaidCart structs
            for cart in paid_cart_ids.hashvector.iter() {
                // get the paid cart details from paid_cart_details: Mapping<Hash, PaidCart>
                let cart_details = self.paid_cart_details.get(cart).unwrap_or_default();
                // add the cart details to the vector of paid cart results
                my_carts.push(cart_details);
            }
            // package the results
            let my_orders = ViewBuyerOrders {
                buyer: caller,
                carts: my_carts,
            };

            // return the results
            my_orders
        }
        

        // 35 🟢 View My (Buyer) Account
        // Front end: if the product/service inventory is zero, note as unavailable
        #[ink(message)]
        pub fn view_my_buyer_account (&self) -> ViewBuyerAccount {
            // set the caller
            let caller = Self::env().caller();
            // get the buyer profile from account_profile_buyer: Mapping<AccountId, BuyerProfile>
            let buyerprofile = self.account_profile_buyer.get(&caller).unwrap_or_default();

            // set up return structures
            let mut productlists = <Vec<ViewProductList>>::default();
            let mut servicelists = <Vec<ViewServiceList>>::default();
            let mut bookmarkedstores = <Vec<SellerProfile>>::default();
            let mut downloads = <Vec<Download>>::default();

            // get product list ids from account_product_lists: Mapping<AccountId, HashVector>
            let product_list_ids = self.account_product_lists.get(&caller).unwrap_or_default();
            // for each id, get the product_list_details: Mapping<Hash, ProductList>
            for id in product_list_ids.hashvector.iter() {
                let listdetails = self.product_list_details.get(id).unwrap_or_default();
                let mut listitems = <Vec<Product>>::default();
                // for each item id in the listdetails.items
                for item in listdetails.items.iter() {
                    // get the prodcut details
                    let productdetails = self.product_details.get(item).unwrap_or_default();
                    // add that to the listitems vector of products
                    listitems.push(productdetails);
                }
                // make the ViewProductList struct for this list
                let viewlist = ViewProductList {
                    owner: listdetails.owner,
                    list_id: listdetails.list_id,
                    list_name: listdetails.list_name,
                    items: listitems
                };
                // add the viewlist to the productlists vector
                productlists.push(viewlist);
            }

            // get service list ids from account_service_lists: Mapping<AccountId, HashVector>
            let service_list_ids = self.account_service_lists.get(&caller).unwrap_or_default();
            // for each id, get the service_list_details: Mapping<Hash, ServiceList>
            for id in service_list_ids.hashvector.iter() {
                let listdetails = self.service_list_details.get(id).unwrap_or_default();
                let mut listitems = <Vec<Service>>::default();
                // for each item id in the listdetails.items
                for item in listdetails.items.iter() {
                    // get the prodcut details
                    let servicedetails = self.service_details.get(item).unwrap_or_default();
                    // add that to the listitems vector of products
                    listitems.push(servicedetails);
                }
                // make the ViewServiceList struct for this list
                let viewlist = ViewServiceList {
                    owner: listdetails.owner,
                    list_id: listdetails.list_id,
                    list_name: listdetails.list_name,
                    items: listitems
                };
                // add the viewlist to the productlists vector
                servicelists.push(viewlist);
            }

            // get bookmarked store accounts from account_store_bookmarks: Mapping<AccountId, AccountVector>
            let store_accounts = self.account_store_bookmarks.get(&caller).unwrap_or_default();

            // for each seller account get the account_profile_seller: Mapping<AccountId, SellerProfile>
            for seller in store_accounts.accountvector.iter() {
                let profile = self.account_profile_seller.get(seller).unwrap_or_default();
                // add that profile to the bookmarkedstores vector of profiles
                bookmarkedstores.push(profile);
            }

            // get the digital product ids
            let digital_ids = self.account_owned_digital_items.get(&caller).unwrap_or_default();
            // for each digital item owned, get the details and create the Download struct
            for id in digital_ids.hashvector.iter() {
                // get the product details
                let details = self.product_details.get(id).unwrap_or_default();
                // create the Download struct for that id
                let download_item = Download {
                    product_id: details.product_id,
                    title: details.title,
                    brand: details.brand,
                    seller_account: details.seller_account,
                    seller_name: details.seller_name,
                    description: details.description,
                    photo: details.photo_or_youtube_link1,
                    more_info: details.more_info_link,
                    file_url: details.digital_file_url,
                };
                // add the download_item to the downloads vector
                downloads.push(download_item);
            }

            // package the results
            let my_account = ViewBuyerAccount {
                buyer: buyerprofile,
                product_lists: productlists,
                service_lists: servicelists,
                bookmarked_stores: bookmarkedstores,
                digital_downloads: downloads,
            };

            // return the results
            my_account
        }


        // 36 🟢 View My (Unpaid) Cart
        #[ink(message)]
        pub fn view_my_cart (&self) -> ViewUnpaidCart {
            // set the caller
            let caller = Self::env().caller();
            // get the callers current unpaid cart from account_current_cart: Mapping<AccountId, UnpaidCart>
            let current_cart = self.account_current_cart.get(&caller).unwrap_or_default();

            // set up return structures
            let mut cartproducts = <Vec<UnpaidCartProduct>>::default();
            let mut cartservices = <Vec<UnpaidCartService>>::default();
            let mut carttotal_products: Balance = 0;
            let mut carttotal_services: Balance = 0;

            // each item in current_cart.cart_items looks like (Hash, u128) meaning (itemid, quantity)
            // for each item, determine product or service
            for (item, number) in current_cart.cart_items.iter() {
                if self.all_products.contains(item) {
                    // get the product details
                    let productdetails = self.product_details.get(item).unwrap_or_default();
                    let zenobuyers = productdetails.zeno_buyers.len().try_into().unwrap();

                    // make the UnpaidCartProduct structure
                    let unpaidproduct = UnpaidCartProduct {
                        product_id: *item,
                        quantity: *number,
                        digital: productdetails.digital,
                        title: productdetails.title,
                        price: productdetails.price,
                        brand: productdetails.brand,
                        seller_account: productdetails.seller_account,
                        seller_name: productdetails.seller_name,
                        photo_or_youtube_link1: productdetails.photo_or_youtube_link1,
                        inventory: productdetails.inventory, 
                        delivery_info: productdetails.delivery_info,
                        product_location: productdetails.product_location,
                        zeno_percent: productdetails.zeno_percent,
                        zeno_buyers: zenobuyers
                    };

                    // add that to the cartproducts vector
                    cartproducts.push(unpaidproduct);

                    // add the price to the cart total for products IF there is enough inventory
                    if productdetails.inventory >= *number {
                        carttotal_products += productdetails.price * *number;
                    }
                    
                }
                else {
                    if self.all_services.contains(item) {
                        // get the service details
                        let servicedetails = self.service_details.get(item).unwrap_or_default();
                        let zenobuyers = servicedetails.zeno_buyers.len().try_into().unwrap();

                        // make the UnpaidCartService structure
                        let unpaidservice = UnpaidCartService {
                            service_id: *item,
                            quantity: *number,
                            online: servicedetails.online,
                            title: servicedetails.title,
                            price: servicedetails.price,
                            seller_account: servicedetails.seller_account,
                            seller_name: servicedetails.seller_name,
                            photo_or_youtube_link1: servicedetails.photo_or_youtube_link1,
                            booking_link: servicedetails.booking_link,
                            inventory: servicedetails.inventory, 
                            service_location: servicedetails.service_location,
                            zeno_percent: servicedetails.zeno_percent,
                            zeno_buyers: zenobuyers
                        };

                        // add that to the cartservices vector
                        cartservices.push(unpaidservice);

                        // add the price to the cart total for services IF there is enough invetory
                        if servicedetails.inventory >= *number {
                            carttotal_services += servicedetails.price * *number;
                        }

                    }
                }
            }

            // the cart total is the total of all items in the cart for which there
            // is sufficient inventory to fulfil the order if you order right now
            let carttotal: Balance = carttotal_products + carttotal_services;
            
            // package the results
            let my_cart = ViewUnpaidCart {
                buyer: caller,
                cart_total: carttotal,
                total_items: current_cart.total_items,
                cart_products: cartproducts,
                cart_services: cartservices
            };

            // return the results
            my_cart
        }


        // 37 🟢 Go To Store
        #[ink(message)]
        pub fn go_to_store (&self,
            seller: AccountId
        ) -> ViewStore {
            // get the seller's profile from account_profile_seller: Mapping<AccountId, SellerProfile>
            let store_owner = self.account_profile_seller.get(&seller).unwrap_or_default();
            // set up return structures
            let mut store_products = <Vec<Product>>::default();
            let mut store_services = <Vec<Service>>::default();

            // get the seller's products from account_seller_products: Mapping<AccountId, HashVector>
            let product_ids = self.account_seller_products.get(&seller).unwrap_or_default();
            for id in product_ids.hashvector.iter() {
                // get the product details struct and add it to the store_products vector
                let productdetails = self.product_details.get(id).unwrap_or_default();
                store_products.push(productdetails);
            }

            // get the seller's services from account_seller_services: Mapping<AccountId, HashVector> 
            let service_ids = self.account_seller_services.get(&seller).unwrap_or_default();
            for id in service_ids.hashvector.iter() {
                // get the service details struct and add it to the store_service vector
                let servicedetails = self.service_details.get(id).unwrap_or_default();
                store_services.push(servicedetails);
            }

            // package the results
            let view_store = ViewStore {
                owner: store_owner,
                products: store_products,
                services: store_services
            };

            // return the results
            view_store
        }


        // 38 🟢 View My Seller Account
        #[ink(message)]
        pub fn view_my_seller_account (&self) -> ViewSellerAccount {
            // set the caller
            let caller = Self::env().caller();
            let store_owner = self.account_profile_seller.get(&caller).unwrap_or_default();
            // set up return structures
            let mut store_products = <Vec<Product>>::default();
            let mut store_services = <Vec<Service>>::default();
            let mut store_orders = <Vec<Order>>::default();

            // get the seller's products from account_seller_products: Mapping<AccountId, HashVector>
            let product_ids = self.account_seller_products.get(&caller).unwrap_or_default();
            for id in product_ids.hashvector.iter() {
                // get the product details struct and add it to the store_products vector
                let productdetails = self.product_details.get(id).unwrap_or_default();
                store_products.push(productdetails);
            }

            // get the seller's services from account_seller_services: Mapping<AccountId, HashVector> 
            let service_ids = self.account_seller_services.get(&caller).unwrap_or_default();
            for id in service_ids.hashvector.iter() {
                // get the service details struct and ad it to the store_service vector
                let servicedetails = self.service_details.get(id).unwrap_or_default();
                store_services.push(servicedetails);
            }

            // get the seller's orders from account_seller_orders: Mapping<AccountId, HashVector>
            let order_ids = self.account_seller_orders.get(&caller).unwrap_or_default();
            for id in order_ids.hashvector.iter() {
                // get the order details struct and add it to the store_orders vector
                // order_details: Mapping<Hash, Order>
                let orderdetails = self.order_details.get(id).unwrap_or_default();
                store_orders.push(orderdetails);
            }

            // package the results
            let my_seller_account = ViewSellerAccount {
                seller: store_owner,
                current_orders: store_orders,
                products: store_products,
                services: store_services
            };

            // return the results
            my_seller_account
        }


        // 39 🟢 Get Market Statistics
        // get various stats about all sellers, all buyers, all products, all services, all orders for analysis
        #[ink(message)]
        pub fn get_market_statistics (&self) -> MarketStatistics {
            // set the caller
            let caller = Self::env().caller();
            let rightnow = self.env().block_timestamp();
            // set up return structures
            let seller_count = self.all_sellers.len().try_into().unwrap();
            let buyer_count = self.all_buyers.len().try_into().unwrap();
            let product_count = self.all_products.len().try_into().unwrap();
            let service_count = self.all_services.len().try_into().unwrap();
            let allorders = &self.all_orders;
            let order_count = allorders.len().try_into().unwrap();
            let mut orderdata = <Vec<OrderData>>::default();
            
            // for each order id, get the details from order_details: Mapping<Hash, Order>
            for id in allorders.iter() {
                let details = self.order_details.get(id).unwrap_or_default();
                let data = OrderData {
                    timestamp: details.order_timestamp,
                    total: details.total_order_price,
                    status: details.order_status,
                    problem: details.problem,
                    resolution: details.resolution,
                };
                orderdata.push(data);
            }

            // package the results
            let stats = MarketStatistics {
                called_by: caller,
                timestamp: rightnow,
                number_of_sellers: seller_count,
                number_of_buyers: buyer_count,
                number_of_products: product_count,
                number_of_services: service_count,
                number_of_orders: order_count,
                orders: orderdata,
            };

            // return the results
            stats
        }


        // 40 🟢 Find Products And Services By Zeno Incentive
        // returns products and services that have zeno_percent > 0 AND < 20 zeno buyers
        #[ink(message)]
        pub fn search_by_zeno (&self) -> ViewZeno {
            // set up return structures
            let mut store_products = <Vec<Product>>::default();
            let mut store_services = <Vec<Service>>::default();

            // get the set of all products from all_products: Vec<Hash>
            let product_ids = &self.all_products;
            for id in product_ids.iter() {
                // get the product details struct 
                let productdetails = self.product_details.get(id).unwrap_or_default();
                // check for zeno_percent > 0 AND zeno_buyers < 20
                let count: u128 = productdetails.zeno_buyers.len().try_into().unwrap();
                if productdetails.zeno_percent > 0 && count < 20 {
                    // add it to the store_products vector
                    store_products.push(productdetails);
                }
            }

            // get the set of all services from all_services: Vec<Hash> 
            let service_ids = &self.all_services;
            for id in service_ids.iter() {
                // get the service details struct 
                let servicedetails = self.service_details.get(id).unwrap_or_default();
                // check for zeno_percent > 0 AND zeno_buyers < 20
                let count: u128 = servicedetails.zeno_buyers.len().try_into().unwrap();
                if servicedetails.zeno_percent > 0 && count < 20 {
                    // add it to the store_services vector
                    store_services.push(servicedetails);
                }
            }

            // package the results
            let view_zeno = ViewZeno {
                products: store_products,
                services: store_services
            };

            // return the results
            view_zeno
        }


        // >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
        // >>>>>>>>>>>>>>>>>>>>>>>>>> SECONDARY GET MESSAGES <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<
        // >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

        // no secondary messages in this contract

        // END OF MESSAGE LIST

    }
    // END OF CONTRACT STORAGE

}
