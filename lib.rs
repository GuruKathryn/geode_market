/*
ABOUT THIS CONTRACT...
This contract lets users buy and sell products (digital and physical) and services 
(online and in person) in the Geode ecosystem.
*/ 

#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod geode_marketplace {

    use ink::prelude::vec::Vec;
    use ink::prelude::vec;
    use ink::prelude::string::String;
    use ink::storage::Mapping;
    use ink::storage::StorageVec;
    use ink::env::hash::{Sha2x256, HashOutput};

    // PRELIMINARY STORAGE STRUCTURES >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

    #[derive(Clone, Debug, PartialEq, Eq, Default)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std",derive(ink::storage::traits::StorageLayout,))]
    pub struct AccountVector {
        accountvector: Vec<AccountId>,
    }

    #[derive(Clone, Debug, PartialEq, Eq, Default)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std",derive(ink::storage::traits::StorageLayout,))]
    pub struct HashVector {
        hashvector: Vec<Hash>,
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std",derive(ink::storage::traits::StorageLayout,))]
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
        review_average: u64,
        review_count: u64,
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
                seller_account: AccountId::from([0x0; 32]),
                seller_name: <Vec<u8>>::default(),
                description: <Vec<u8>>::default(),
                review_average: u64::default(),
                review_count: u64::default(), 
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

    #[derive(Clone, Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std",derive(ink::storage::traits::StorageLayout,))]
    pub struct PublicProduct { 
        product_id: Hash,
        digital: bool,
        title: Vec<u8>,
        price: Balance,
        brand: Vec<u8>,
        category: Vec<u8>,
        seller_account: AccountId,
        seller_name: Vec<u8>,
        description: Vec<u8>,
        review_average: u64,
        review_count: u64,
        inventory: u128, 
        photo_or_youtube_link1: Vec<u8>, 
        photo_or_youtube_link2: Vec<u8>,
        photo_or_youtube_link3: Vec<u8>,
        more_info_link: Vec<u8>,
        delivery_info: Vec<u8>,
        product_location: Vec<u8>,
        // include zip code, city, state, country, etc
        zeno_percent: u128,
        // must be 0-100, default is 0
        zeno_buyers: Vec<AccountId>
        // tracks the first 20 buyers for zeno's incentive
    }

    impl Default for PublicProduct {
        fn default() -> PublicProduct {
            PublicProduct {
                product_id: Hash::default(),
                digital: bool::default(),
                title: <Vec<u8>>::default(),
                price: Balance::default(),
                brand:<Vec<u8>>::default(),
                category: <Vec<u8>>::default(),
                seller_account: AccountId::from([0x0; 32]),
                seller_name: <Vec<u8>>::default(),
                description: <Vec<u8>>::default(),
                review_average: u64::default(),
                review_count: u64::default(), 
                inventory: u128::default(), 
                photo_or_youtube_link1: <Vec<u8>>::default(), 
                photo_or_youtube_link2: <Vec<u8>>::default(),
                photo_or_youtube_link3: <Vec<u8>>::default(),
                more_info_link: <Vec<u8>>::default(),
                delivery_info: <Vec<u8>>::default(),
                product_location: <Vec<u8>>::default(),
                zeno_percent: 0,
                zeno_buyers: <Vec<AccountId>>::default()
            }
        }
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std",derive(ink::storage::traits::StorageLayout,))]
    pub struct Service { 
        service_id: Hash,
        online: bool,
        title: Vec<u8>,
        price: Balance,
        category: Vec<u8>,
        seller_account: AccountId,
        seller_name: Vec<u8>,
        description: Vec<u8>,
        review_average: u64,
        review_count: u64,
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
                seller_account: AccountId::from([0x0; 32]),
                seller_name: <Vec<u8>>::default(),
                description: <Vec<u8>>::default(),
                review_average: u64::default(),
                review_count: u64::default(),
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

    #[derive(Clone, Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std",derive(ink::storage::traits::StorageLayout,))]
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
                seller_account: AccountId::from([0x0; 32]),
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

    #[derive(Clone, Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std",derive(ink::storage::traits::StorageLayout,))]
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
                seller_account: AccountId::from([0x0; 32]),
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

    #[derive(Clone, Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std",derive(ink::storage::traits::StorageLayout,))]
    pub struct UnpaidCart { 
        buyer: AccountId,
        cart_total: Balance,
        total_items: u128,
        cart_items: Vec<(Hash, u128)>
    }

    impl Default for UnpaidCart {
        fn default() -> UnpaidCart {
            UnpaidCart {
                buyer: AccountId::from([0x0; 32]),
                cart_total: Balance::default(), 
                total_items: 0,
                cart_items: <Vec<(Hash, u128)>>::default()
            }
        }
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std",derive(ink::storage::traits::StorageLayout,))]
    pub struct Order {
        order_id: Hash,
        cart_id: Hash,
        order_timestamp: u64,
        buyer: AccountId,
        buyer_rating: u64,
        buyer_rating_count: u64,
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
        discussion: Vec<MessageDetails>,
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
                buyer: AccountId::from([0x0; 32]),
                buyer_rating: u64::default(),
                buyer_rating_count: u64::default(),
                seller: AccountId::from([0x0; 32]),
                seller_name: <Vec<u8>>::default(),
                image: <Vec<u8>>::default(),
                item_id: Hash::default(),
                item_name: <Vec<u8>>::default(),
                quantity: u128::default(),
                price_each: Balance::default(),
                total_order_price: Balance::default(),
                deliver_to_address: <Vec<u8>>::default(),
                deliver_to_account: AccountId::from([0x0; 32]),
                tracking_info: <Vec<u8>>::default(),
                order_status: 0, 
                time_delivered: u64::default(),
                discussion: <Vec<MessageDetails>>::default(),
                problem: 0,
                resolution: 0,
                zeno_total: Balance::default()
            }
        }
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std",derive(ink::storage::traits::StorageLayout,))]
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
                from_acct: AccountId::from([0x0; 32]),
                to_acct: AccountId::from([0x0; 32]),
                order_id: Hash::default(),
                message: <Vec<u8>>::default(),
                media_url: <Vec<u8>>::default(),
                timestamp: u64::default()
            }
        }
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std",derive(ink::storage::traits::StorageLayout,))]
    pub struct BuyerProfile { 
        buyer_account: AccountId,
        buyer_name: Vec<u8>,
        buyer_location: Vec<u8>,
        member_since: u64,
        review_average: u64,
        review_count: u64,
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
                buyer_account: AccountId::from([0x0; 32]),
                buyer_name: <Vec<u8>>::default(),
                buyer_location: <Vec<u8>>::default(),
                member_since: u64::default(),
                review_average: u64::default(),
                review_count: u64::default(),
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
    
    #[derive(Clone, Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std",derive(ink::storage::traits::StorageLayout,))]
    pub struct SellerProfile { 
        seller_account: AccountId,
        seller_name: Vec<u8>,
        store_description: Vec<u8>,
        seller_location: Vec<u8>,
        member_since: u64,
        banner_url: Vec<u8>,
        youtube_url: Vec<u8>,
        external_link: Vec<u8>,
        review_average: u64,
        review_count: u64,
        total_orders: u128,
        awaiting: u128,
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
                seller_account: AccountId::from([0x0; 32]),
                seller_name: <Vec<u8>>::default(),
                store_description: <Vec<u8>>::default(),
                seller_location: <Vec<u8>>::default(),
                member_since: u64::default(),
                banner_url: <Vec<u8>>::default(),
                youtube_url: <Vec<u8>>::default(),
                external_link: <Vec<u8>>::default(),
                review_average: u64::default(),
                review_count: u64::default(),
                total_orders: u128::default(),
                awaiting: u128::default(),
                total_delivered: u128::default(),
                total_damaged: u128::default(),
                total_wrong: u128::default(),
                total_not_received: u128::default(),
                total_resolved: u128::default(),
                total_refused: u128::default(),
            }
        }
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std",derive(ink::storage::traits::StorageLayout,))]
    pub struct ProductServiceReview {
        review_id: Hash,
        item_id: Hash,
        reviewer: AccountId,
        rating: u64,
        // error if not 1-5
        review: Vec<u8>,
        timestamp: u64,
    }

    impl Default for ProductServiceReview {
        fn default() -> ProductServiceReview {
            ProductServiceReview {
                review_id: Hash::default(),
                item_id: Hash::default(),
                reviewer: AccountId::from([0x0; 32]),
                rating: u64::default(),
                review: <Vec<u8>>::default(),
                timestamp: u64::default(),
            }
        }
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std",derive(ink::storage::traits::StorageLayout,))]
    pub struct BuyerSellerReview {
        review_id: Hash,
        account_id: AccountId,
        reviewer: AccountId,
        rating: u64,
        // error if not 1-5
        review: Vec<u8>,
        timestamp: u64,
    }

    impl Default for BuyerSellerReview {
        fn default() -> BuyerSellerReview {
            BuyerSellerReview {
                review_id: Hash::default(),
                account_id: AccountId::from([0x0; 32]),
                reviewer: AccountId::from([0x0; 32]),
                rating: u64::default(),
                review: <Vec<u8>>::default(),
                timestamp: u64::default(),
            }
        }
    }
   
    // STORAGE STRUCTURES FOR PRIMARY GET MESSAGES >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
    
    #[derive(Clone, Debug, PartialEq, Eq, Default)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std",derive(ink::storage::traits::StorageLayout,))]
    pub struct ProductSearchResults {
        search: Vec<Vec<u8>>,
        products: Vec<PublicProduct>
    }

    #[derive(Clone, Debug, PartialEq, Eq, Default)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std",derive(ink::storage::traits::StorageLayout,))]
    pub struct ServiceSearchResults {
        search: Vec<Vec<u8>>,
        services: Vec<Service>
    }

    #[derive(Clone, Debug, PartialEq, Eq, Default)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std",derive(ink::storage::traits::StorageLayout,))]
    pub struct StoreSearchResults {
        search: Vec<Vec<u8>>,
        stores: Vec<SellerProfile>
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std",derive(ink::storage::traits::StorageLayout,))]

    pub struct ViewBuyerOrders {
        buyer: AccountId,
        carts: Vec<Order>
    }

    impl Default for ViewBuyerOrders {
        fn default() -> ViewBuyerOrders {
            ViewBuyerOrders {
                buyer: AccountId::from([0x0; 32]),
                carts: <Vec<Order>>::default()
            }
        }
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std",derive(ink::storage::traits::StorageLayout,))]
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
                seller_account: AccountId::from([0x0; 32]),
                seller_name: <Vec<u8>>::default(),
                description: <Vec<u8>>::default(),
                photo: <Vec<u8>>::default(),
                more_info: <Vec<u8>>::default(),
                file_url: <Vec<u8>>::default(),
            }
        }
    }

    #[derive(Clone, Debug, PartialEq, Eq, Default)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std",derive(ink::storage::traits::StorageLayout,))]
    pub struct ViewBuyerAccount {
        buyer: BuyerProfile,
        bookmarked_stores: Vec<SellerProfile>,
        digital_downloads: Vec<Download>,
        orders: Vec<Order>
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std",derive(ink::storage::traits::StorageLayout,))]
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
                buyer: AccountId::from([0x0; 32]),
                cart_total: Balance::default(), 
                total_items: 0,
                cart_products: <Vec<UnpaidCartProduct>>::default(),
                cart_services: <Vec<UnpaidCartService>>::default()
            }
        }
    }

    #[derive(Clone, Debug, PartialEq, Eq, Default)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std",derive(ink::storage::traits::StorageLayout,))]
    pub struct ViewStore {
        owner: SellerProfile,
        products: Vec<PublicProduct>,
        services: Vec<Service>
    }

    #[derive(Clone, Debug, PartialEq, Eq, Default)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std",derive(ink::storage::traits::StorageLayout,))]
    pub struct ViewSellerAccount {
        owner: SellerProfile,
        products: Vec<Product>,
        services: Vec<Service>
    }

    #[derive(Clone, Debug, PartialEq, Eq, Default)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std",derive(ink::storage::traits::StorageLayout,))]
    pub struct OrderData {
        timestamp: u64,
        total: Balance,
        status: u8,
        problem: u8,
        resolution: u8,
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std",derive(ink::storage::traits::StorageLayout,))]
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
                called_by: AccountId::from([0x0; 32]),
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

    
    // EVENT DEFINITIONS >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>> 

    #[ink(event)]
    // writes a new order to the blockchain at cart checkout
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

    #[ink(event)]
    // affirms ownership of a digital file on checkout
    pub struct DigitalDownload {
        #[ink(topic)]
        buyer: AccountId,
        #[ink(topic)]
        product_id: Hash,
        order_timestamp: u64,
    }

    // new product or service rating and details
    #[ink(event)]
    pub struct NewProductRating {
        #[ink(topic)]
        review_id: Hash,
        #[ink(topic)]
        item_id: Hash,
        #[ink(topic)]
        reviewer: AccountId,
        rating: u64,
        review: Vec<u8>,
        timestamp: u64,
    }

    // order problem reported
    #[ink(event)]
    pub struct ProblemReported {
        #[ink(topic)]
        order_id: Hash,
        #[ink(topic)]
        buyer: AccountId,
        #[ink(topic)]
        seller: AccountId,
        problem: u8,
    }

    // buyer has updted their account settings
    #[ink(event)]
    pub struct BuyerSettingsUpdated {
        #[ink(topic)]
        buyer_account: AccountId,
        #[ink(topic)]
        name: Vec<u8>,
        #[ink(topic)]
        location: Vec<u8>,
    }

    // seller has updated account settings
    #[ink(event)]
    pub struct SellerSettingsUpdated {
        #[ink(topic)]
        seller_account: AccountId,
        #[ink(topic)]
        seller_name: Vec<u8>,
        store_description: Vec<u8>,
        #[ink(topic)]
        seller_location: Vec<u8>,
        banner_url: Vec<u8>,
        youtube_url: Vec<u8>,
        external_link: Vec<u8>,
    }

    // seller updated order tracking - delivered
    #[ink(event)]
    pub struct OrderDelivered {
        #[ink(topic)]
        seller: AccountId,
        #[ink(topic)]
        buyer: AccountId,
        #[ink(topic)]
        order_id: Hash,
        order_status: u8,
        time_delivered: u64,
    }

    // seller updated order tracking - shipped
    #[ink(event)]
    pub struct OrderShipped {
        #[ink(topic)]
        seller: AccountId,
        #[ink(topic)]
        buyer: AccountId,
        #[ink(topic)]
        order_id: Hash,
        order_status: u8,
    }

    // seller refused an order
    #[ink(event)]
    pub struct OrderRefused {
        #[ink(topic)]
        seller: AccountId,
        #[ink(topic)]
        buyer: AccountId,
        #[ink(topic)]
        order_id: Hash,
        order_status: u8,
    }

    // seller issued a refund
    #[ink(event)]
    pub struct OrderRefunded {
        #[ink(topic)]
        seller: AccountId,
        #[ink(topic)]
        buyer: AccountId,
        #[ink(topic)]
        order_id: Hash,
        order_status: u8,
        problem: u8,
        resolution: u8,
    }

    // seller issued a replacement
    #[ink(event)]
    pub struct OrderReplaced {
        #[ink(topic)]
        seller: AccountId,
        #[ink(topic)]
        buyer: AccountId,
        #[ink(topic)]
        order_id: Hash,
        order_status: u8,
        problem: u8,
        resolution: u8,
    }

    // seller denied resolution request
    #[ink(event)]
    pub struct OrderResolutionDenied {
        #[ink(topic)]
        seller: AccountId,
        #[ink(topic)]
        buyer: AccountId,
        #[ink(topic)]
        order_id: Hash,
        order_status: u8,
        problem: u8,
        resolution: u8,
    }

    // new rating for a buyer and details
    #[ink(event)]
    pub struct NewBuyerRating {
        #[ink(topic)]
        review_id: Hash,
        #[ink(topic)]
        buyer: AccountId,
        #[ink(topic)]
        reviewer: AccountId,
        rating: u64,
        review: Vec<u8>,
        timestamp: u64,
    }

    // new product added
    #[ink(event)]
    pub struct NewProduct {
        #[ink(topic)]
        product_id: Hash,
        digital: bool,
        #[ink(topic)]
        title: Vec<u8>,
        price: Balance,
        brand: Vec<u8>,
        category: Vec<u8>,
        #[ink(topic)]
        seller_account: AccountId,
        seller_name: Vec<u8>,
        description: Vec<u8>, 
        inventory: u128, 
        photo_or_youtube_link1: Vec<u8>, 
        photo_or_youtube_link2: Vec<u8>,
        photo_or_youtube_link3: Vec<u8>,
        more_info_link: Vec<u8>,
        delivery_info: Vec<u8>,
        product_location: Vec<u8>,
        zeno_percent: u128,
    }

    // product details updated
    #[ink(event)]
    pub struct UpdatedProduct {
        #[ink(topic)]
        product_id: Hash,
        digital: bool,
        #[ink(topic)]
        title: Vec<u8>,
        price: Balance,
        brand: Vec<u8>,
        category: Vec<u8>,
        #[ink(topic)]
        seller_account: AccountId,
        seller_name: Vec<u8>,
        description: Vec<u8>, 
        inventory: u128, 
        photo_or_youtube_link1: Vec<u8>, 
        photo_or_youtube_link2: Vec<u8>,
        photo_or_youtube_link3: Vec<u8>,
        more_info_link: Vec<u8>,
        delivery_info: Vec<u8>,
        product_location: Vec<u8>,
    }

    // new service added
    #[ink(event)]
    pub struct NewService {
        #[ink(topic)]
        service_id: Hash,
        online: bool,
        #[ink(topic)]
        title: Vec<u8>,
        price: Balance,
        category: Vec<u8>,
        #[ink(topic)]
        seller_account: AccountId,
        seller_name: Vec<u8>,
        description: Vec<u8>,
        inventory: u128,
        photo_or_youtube_link1: Vec<u8>, 
        photo_or_youtube_link2: Vec<u8>,
        photo_or_youtube_link3: Vec<u8>,
        booking_link: Vec<u8>,
        service_location: Vec<u8>,
        zeno_percent: u128,
    }

    // service details updated
    #[ink(event)]
    pub struct UpdatedService {
        #[ink(topic)]
        service_id: Hash,
        online: bool,
        #[ink(topic)]
        title: Vec<u8>,
        price: Balance,
        category: Vec<u8>,
        #[ink(topic)]
        seller_account: AccountId,
        seller_name: Vec<u8>,
        description: Vec<u8>,
        inventory: u128,
        photo_or_youtube_link1: Vec<u8>, 
        photo_or_youtube_link2: Vec<u8>,
        photo_or_youtube_link3: Vec<u8>,
        booking_link: Vec<u8>,
        service_location: Vec<u8>,
    }



    // ERROR DEFINITIONS >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

    #[derive(Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub enum Error {
        // if the payment fails
        PayoutFailed,
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
        // input data is too large for storage
        DataTooLarge,
        // storage for that item is full, please delete some
        StorageFull,
    }


    // ACTUAL CONTRACT STORAGE STRUCT >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

    #[ink(storage)]
    pub struct ContractStorage {
        all_sellers: StorageVec<AccountId>,
        total_count_buyers: u128,
        total_count_services: u128,
        total_count_products: u128,
        total_count_orders: u128,
        all_orders: Vec<Hash>,
        account_profile_seller: Mapping<AccountId, SellerProfile>,
        account_profile_buyer: Mapping<AccountId, BuyerProfile>,
        account_store_bookmarks: Mapping<AccountId, AccountVector>,
        account_buyer_orders: Mapping<AccountId, HashVector>,
        account_buyer_items_bought: Mapping<AccountId, HashVector>,
        account_buyer_items_reviewed: Mapping<AccountId, HashVector>,
        account_seller_buyers_reviewed: Mapping<AccountId, AccountVector>,
        account_owned_digital_items: Mapping<AccountId, HashVector>,
        account_seller_orders_0awaiting: Mapping<AccountId, HashVector>,
        account_seller_orders_1shipped: Mapping<AccountId, HashVector>,
        account_seller_orders_2delivered: Mapping<AccountId, HashVector>,
        account_seller_orders_3resolved: Mapping<AccountId, HashVector>,
        account_seller_orders_4problem: Mapping<AccountId, HashVector>,
        account_seller_orders_5refused: Mapping<AccountId, HashVector>,
        account_current_cart: Mapping<AccountId, UnpaidCart>,
        account_seller_products: Mapping<AccountId, HashVector>,
        account_seller_services: Mapping<AccountId, HashVector>,
        product_details: Mapping<Hash, Product>,
        service_details: Mapping<Hash, Service>,
        order_details: Mapping<Hash, Order>,
    }


    // BEGIN CONTRACT LOGIC >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

    impl ContractStorage {
        
        // CONSTRUCTORS >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
        // Constructors are implicitly payable.

        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                all_sellers: StorageVec::default(),
                total_count_buyers: 0,
                total_count_services: 0,
                total_count_products: 0,
                total_count_orders: 0,
                all_orders: <Vec<Hash>>::default(),
                account_profile_seller: Mapping::default(),
                account_profile_buyer: Mapping::default(),
                account_store_bookmarks: Mapping::default(),
                account_buyer_orders: Mapping::default(),
                account_buyer_items_bought: Mapping::default(),
                account_buyer_items_reviewed: Mapping::default(),
                account_seller_buyers_reviewed: Mapping::default(),
                account_owned_digital_items: Mapping::default(),
                account_seller_orders_0awaiting: Mapping::default(),
                account_seller_orders_1shipped: Mapping::default(),
                account_seller_orders_2delivered: Mapping::default(),
                account_seller_orders_3resolved: Mapping::default(),
                account_seller_orders_4problem: Mapping::default(),
                account_seller_orders_5refused: Mapping::default(),
                account_current_cart: Mapping::default(),
                account_seller_products: Mapping::default(),
                account_seller_services: Mapping::default(),
                product_details: Mapping::default(),
                service_details: Mapping::default(),
                order_details: Mapping::default(),
            }
        }


        // >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
        // MESSAGE FUNCTIONS THAT CHANGE DATA IN THE CONTRACT STORAGE >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
        // >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>


        // 0 🟢 Add Item To Cart
        #[ink(message)]
        pub fn add_item_to_cart (&mut self, 
            add_item_id: Hash, 
            quantity: u128
        ) -> Result<(), Error> {
            // set up the caller
            let caller = Self::env().caller();
            // get the current unpaid cart for this caller from account_current_cart
            let mut cart = self.account_current_cart.get(caller).unwrap_or_default();
            // if the cart is full, send an error
            if cart.cart_items.len() > 49 {
                return Err(Error::StorageFull);
            }
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
                        new_quantity = number.saturating_add(quantity);
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
                if self.product_details.contains(item) {
                    item_price = self.product_details.get(item).unwrap_or_default().price;
                }
                else {
                    if self.service_details.contains(item) {
                        item_price = self.service_details.get(item).unwrap_or_default().price;
                    }
                }
                // add the price to the total price
                carttotal = carttotal.saturating_add(item_price.saturating_mul(*number));
            }

            // perpare the updated UnpaidCart
            let updated_cart = UnpaidCart {
                buyer: caller,
                cart_total: carttotal,
                total_items: totalitems,
                cart_items: cart.cart_items
            };

            // update mappings
            self.account_current_cart.insert(caller, &updated_cart);
            
            Ok(())
        }


        // 1 🟢 Bookmark A Store
        #[ink(message)]
        pub fn bookmark_a_store (&mut self, 
            seller: AccountId,
        ) -> Result<(), Error> {
            // set up clones
            // set up the caller
            let caller = Self::env().caller();
            // get the account_store_boookmarks list
            let mut my_list = self.account_store_bookmarks.get(caller).unwrap_or_default();
            // if their bookmarks are full or the seller is already on the list, send an error
            if my_list.accountvector.len() > 19 || my_list.accountvector.contains(&seller) {
                return Err(Error::StorageFull);
            }
            else {
                // if the seller is not there already, add them to the vector
                my_list.accountvector.push(seller);
                // update mapping account_store_bookmarks: Mapping<AccountId, AccountVector>
                self.account_store_bookmarks.insert(caller, &my_list);

                Ok(())
            }
        }

        // 2 🟢 Remove A Store Bookmark
        #[ink(message)]
        pub fn remove_store_bookmark (&mut self, 
            seller: AccountId,
        ) -> Result<(), Error> {
            // set up the caller
            let caller = Self::env().caller();
            // get the account_store_boookmarks list
            let mut my_list = self.account_store_bookmarks.get(caller).unwrap_or_default();
            if my_list.accountvector.contains(&seller) {
                // remove the store account from the bookmark list
                my_list.accountvector.retain(|value| *value != seller);
                // update mapping account_store_bookmarks: Mapping<AccountId, AccountVector>
                self.account_store_bookmarks.insert(caller, &my_list);
            }
            
            Ok(())
        }

        
        // 3 🟢 Remove Item From Cart
        #[ink(message)]
        pub fn remove_item_from_cart (&mut self, 
            item_id: Hash,
        ) -> Result<(), Error> {
            // set up the caller
            let caller = Self::env().caller();
            // get the caller's current unpaid cart id
            let mut cart = self.account_current_cart.get(caller).unwrap_or_default();
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
                if self.product_details.contains(item) {
                    item_price = self.product_details.get(item).unwrap_or_default().price;
                }
                else {
                    if self.service_details.contains(item) {
                        item_price = self.service_details.get(item).unwrap_or_default().price;
                    }
                }
                // add the price to the total price
                carttotal = carttotal.saturating_add(item_price.saturating_mul(*number));
            }
 
            // perpare the updated UnpaidCart
            let updated_cart = UnpaidCart {
                buyer: caller,
                cart_total: carttotal,
                total_items: totalitems,
                cart_items: cart.cart_items
            };
 
            // update mappings
            self.account_current_cart.insert(caller, &updated_cart);           
            
            Ok(())
        }
        
    
        // 4 🟢 Update Cart Item Quantity
        #[ink(message)]
        pub fn update_cart_item_quantity (&mut self, 
            item_id: Hash,
            new_quantity: u128
        ) -> Result<(), Error> {
            // set up the caller
            let caller = Self::env().caller();
            // get the caller's current unpaid cart id
            let mut cart = self.account_current_cart.get(caller).unwrap_or_default();
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
                if self.product_details.contains(item) {
                    item_price = self.product_details.get(item).unwrap_or_default().price;
                }
                else {
                    if self.service_details.contains(item) {
                        item_price = self.service_details.get(item).unwrap_or_default().price;
                    }
                }
                // add the price to the total price
                carttotal = carttotal.saturating_add(item_price.saturating_mul(*number));
            }
 
            // perpare the updated UnpaidCart
            let updated_cart = UnpaidCart {
                buyer: caller,
                cart_total: carttotal,
                total_items: totalitems,
                cart_items: cart.cart_items
            };
 
            // update mappings
            self.account_current_cart.insert(caller, &updated_cart);   
            
            Ok(())
        }

        
        // 5 🟢 Checkout Cart
        #[ink(message, payable)]
        pub fn checkout_cart (&mut self, 
            deliver_to_address: Vec<u8>
        ) -> Result<(), Error> {
            // make sure the address is not too long
            if deliver_to_address.len() > 300 {
                return Err(Error::DataTooLarge);
            }

            // set up the caller and timestamp
            let caller = Self::env().caller();
            let rightnow = self.env().block_timestamp();

            // get the caller's unpaid cart
            let current_cart = self.account_current_cart.get(caller).unwrap_or_default();

            // UPDATE THE CART TOTAL AND REMOVE ITEMS THAT DO NOT HAVE ENOUGH INVENTORY
            // make a new cart items vector to work with
            let mut final_cart_items = <Vec<(Hash, u128)>>::default();
            let mut item_inventory: u128 = 0;
            let mut item_price: Balance = 0;
            let mut carttotal: Balance = 0;
            // iterate through the cart to keep only items that have enough inventory
            for (item, number) in &current_cart.cart_items {
                // get the inventory and price for that item
                if self.product_details.contains(item) {
                    item_inventory = self.product_details.get(item).unwrap_or_default().inventory;
                    item_price = self.product_details.get(item).unwrap_or_default().price;
                }
                else {
                    if self.service_details.contains(item) {
                        item_inventory = self.service_details.get(item).unwrap_or_default().inventory;
                        item_price = self.service_details.get(item).unwrap_or_default().price;
                    }
                }
                // if the item has enough inventory, add it to the official cart items
                if item_inventory >= *number {
                    // add this item to the total price
                    carttotal = carttotal.saturating_add(item_price.saturating_mul(*number));
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

                // FOR EACH ITEM IN THE CART ...
                for (item, number) in &final_cart_items {

                    // set up clones
                    let deliver_to_address_clone1 = deliver_to_address.clone();
                    
                    // CREATE THE ORDER STRUCT FOR THIS ITEM...

                    let mut item_seller: AccountId = AccountId::from([0x0; 32]);
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
                        details.inventory = details.inventory.saturating_sub(*number);

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
                            details.inventory = details.inventory.saturating_sub(*number);

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
                    let item_order_total: Balance = number.saturating_mul(item_price);

                    // calculate the zeno total
                    let item_zeno_total: Balance = item_order_total.saturating_mul(item_zeno_percent).saturating_div(100); 

                    // account for alternate order status when the product is digital
                    let mut status: u8 = 0;
                    if item_is_digital || item_is_service {
                        status = 2;
                    }

                    // get the buyer profile
                    let mut buyer_profile = self.account_profile_buyer.get(caller).unwrap_or_default();

                    // set up the Order structure
                    let mut new_order = Order {
                        order_id: new_order_id,
                        cart_id: new_cart_id,
                        order_timestamp: rightnow,
                        buyer: caller,
                        buyer_rating: buyer_profile.review_average,
                        buyer_rating_count: buyer_profile.review_count,
                        seller: item_seller,
                        seller_name: item_seller_name,
                        image: item_image,
                        item_id: *item,
                        item_name: item_name,
                        quantity: *number,
                        price_each: item_price,
                        total_order_price: item_order_total,
                        deliver_to_address: deliver_to_address_clone1,
                        deliver_to_account: caller,
                        tracking_info: <Vec<u8>>::default(),
                        order_status: status, 
                        time_delivered: u64::default(),
                        discussion: <Vec<MessageDetails>>::default(),
                        problem: 0,
                        resolution: 0,
                        zeno_total: item_zeno_total
                    };

                    // SPECIAL ACTIONS FOR DIGITAL PRODUCTS...
                    // if the item is a digital product, send ownership to the buyer and pay the seller
                    if item_is_digital {
                        // get this account's set of owned digital items
                        let mut owned = self.account_owned_digital_items.get(caller).unwrap_or_default();
                        // is this item already in the owned list?
                        if owned.hashvector.contains(item) {
                            // do nothing
                        }
                        else {
                            // if the owner already has 400 digital dowloads in storage, remove the oldest
                            if owned.hashvector.len() > 399 {
                                owned.hashvector.remove(0);
                            }
                            owned.hashvector.push(*item);
                            // update account_owned_digital_items: Mapping<AccountId, HashVector>
                            self.account_owned_digital_items.insert(caller, &owned);
                        }
                        
                        // mark the order as delivered
                        new_order.time_delivered = rightnow;
                        
                        // payout the seller for the digital product
                        if self.env().transfer(item_seller, item_order_total).is_err() {
                            return Err(Error::PayoutFailed);
                        }
                    }

                    // PAYOUT SERVICES
                    if item_is_service {
                        // mark the order as delivered
                        new_order.time_delivered = rightnow;
                        
                        // payout the seller for the service
                        // self.env().transfer(item_seller, item_order_total).expect("payout failed");
                        if self.env().transfer(item_seller, item_order_total).is_err() {
                            return Err(Error::PayoutFailed);
                        }
                    }

                    // update order_details: Mapping<Hash, Order>
                    self.order_details.insert(new_order_id, &new_order);
                    // update all_orders: Vec<Hash> keep the 490 most recent
                    if self.all_orders.len() > 489 {
                        // kick out the oldest
                        self.all_orders.remove(0);
                    }
                    self.all_orders.push(new_order_id);
                    // increase total_count_orders by one
                    self.total_count_orders = self.total_count_orders.saturating_add(1);

                    // update all_cart_orders, all_cart_items, total_items_count, and total_orders_count
                    all_cart_orders.push(new_order);
                    all_cart_items.push(*item);
                    total_items_count = total_items_count.saturating_add(*number);
                    total_orders_count = total_orders_count.saturating_add(1);
                                        
                    // update account_buyer_orders: Mapping<AccountId, HashVector>
                    let mut buyer_orders = self.account_buyer_orders.get(caller).unwrap_or_default();
                    // if there are more than 24 orders here, remove the oldest
                    if buyer_orders.hashvector.len() > 24 {
                        buyer_orders.hashvector.remove(0);
                    }
                    buyer_orders.hashvector.push(new_order_id);
                    self.account_buyer_orders.insert(caller, &buyer_orders);

                    // update account_buyer_items_bought: Mapping<AccountId, HashVector>
                    let mut buyer_items = self.account_buyer_items_bought.get(caller).unwrap_or_default();
                    if buyer_items.hashvector.contains(item) {
                        // do nothing
                    }
                    else {
                        // if there are more than 399 items here, remove the oldest
                        if buyer_items.hashvector.len() > 399 {
                            buyer_items.hashvector.remove(0);
                        }
                        buyer_items.hashvector.push(*item);
                        self.account_buyer_items_bought.insert(caller, &buyer_items);
                    }

                    // update account_seller_orders: Mapping<AccountId, HashVector> based on status (0 or 2)
                    if status == 0 {
                        let mut seller_orders = self.account_seller_orders_0awaiting.get(item_seller).unwrap_or_default();
                        // if the seller_orders.hashvector AWAITING is full, send error
                        if seller_orders.hashvector.len() > 69 && status == 0 {
                            return Err(Error::StorageFull);
                        }
                        // otherwise, add this order and update the mapping
                        seller_orders.hashvector.push(new_order_id);
                        self.account_seller_orders_0awaiting.insert(item_seller, &seller_orders);
                    }
                    if status == 2 {
                        let mut seller_orders = self.account_seller_orders_2delivered.get(item_seller).unwrap_or_default();
                        // if the seller_orders.hashvector DELIVERED full, remove the oldest
                        if seller_orders.hashvector.len() > 69 && status == 2 {
                            // remove the oldest
                            seller_orders.hashvector.remove(0);
                        }
                        // add this order and update the mapping
                        seller_orders.hashvector.push(new_order_id);
                        self.account_seller_orders_2delivered.insert(item_seller, &seller_orders);
                    }
                    
                    // update account_profile_seller: Mapping<AccountId, SellerProfile>
                    let mut seller_profile = self.account_profile_seller.get(item_seller).unwrap_or_default();
                    // increment total_orders
                    seller_profile.total_orders = seller_profile.total_orders.saturating_add(1);
                    if item_is_digital || item_is_service {
                        seller_profile.total_delivered = seller_profile.total_delivered.saturating_add(1);
                    }
                    // update the total awaiting orders
                    let awaiting_now = self.account_seller_orders_0awaiting.get(item_seller).unwrap_or_default().hashvector.len();
                    seller_profile.awaiting = awaiting_now.try_into().unwrap();
                    // update the map 
                    self.account_profile_seller.insert(item_seller, &seller_profile);

                    // update account_profile_buyer: Mapping<AccountId, BuyerProfile>
                    // increment total_orders
                    buyer_profile.total_orders = buyer_profile.total_orders.saturating_add(1);
                    if item_is_digital || item_is_service {
                        buyer_profile.total_delivered = buyer_profile.total_delivered.saturating_add(1);
                    }
                    // if this is the first order, set the member_since timestamp
                    // and add one to the total_count_buyers
                    if buyer_profile.member_since == u64::default() {
                        buyer_profile.member_since = rightnow;
                        self.total_count_buyers = self.total_count_buyers.saturating_add(1);
                    }
                    self.account_profile_buyer.insert(caller, &buyer_profile);

                    // EMIT EVENT to register the order to the chain
                    Self::env().emit_event(OrderPlaced {
                        order_id: new_order_id,
                        order_timestamp: rightnow,
                        buyer: caller,
                        seller: item_seller,
                        total_order_price: item_order_total,
                    });

                    // EMIT EVENT For DIGITAL DOWNLOAD OWNERSHIP
                    if item_is_digital {
                        Self::env().emit_event(DigitalDownload {
                            buyer: caller,
                            product_id: *item,
                            order_timestamp: rightnow,
                        });
                    }
                }

                // UPDATE CART RELATED STORAGE MAPPINGS...

                // update account_profile_buyer: Mapping<AccountId, BuyerProfile>
                let mut buyer_profile = self.account_profile_buyer.get(caller).unwrap_or_default();
                // increment total_carts and total_orders
                buyer_profile.total_carts = buyer_profile.total_carts.saturating_add(1);
                self.account_profile_buyer.insert(caller, &buyer_profile);

                // delete caller's unpaid cart in account_current_cart: Mapping<AccountId, UnpaidCart>
                self.account_current_cart.remove(caller);

                Ok(())
            }
        }

        
        // 6 🟢 Rate A Product or Service
        #[ink(message)]
        pub fn rate_a_product_or_service (&mut self, 
            item_id: Hash,
            rating: u64,
            review: Vec<u8>
        ) -> Result<(), Error> {
            // if the X if full, send an error
            if review.len() > 600 {
                return Err(Error::DataTooLarge);
            }
            // if the rating is between 1 and 5
            if rating > 0 && rating < 6 {
                // set up the caller
                let caller = Self::env().caller();
                let now = self.env().block_timestamp();

                // account_buyer_items_bought: Mapping<AccountId, HashVector>
                let bought = self.account_buyer_items_bought.get(caller).unwrap_or_default();
                // account_buyer_items_reviewed: Mapping<AccountId, HashVector>
                let mut reviewed = self.account_buyer_items_reviewed.get(caller).unwrap_or_default();
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
                        
                        // update mappings...
                        // account_buyer_items_reviewed: Mapping<AccountId, HashVector>
                        // if the reviewed hashvector if full, send an error
                        if reviewed.hashvector.len() > 399 {
                            // remove the oldest
                            reviewed.hashvector.remove(0);
                        }
                        reviewed.hashvector.push(item_id);
                        self.account_buyer_items_reviewed.insert(caller, &reviewed);
                        
                        if self.product_details.contains(item_id) {
                            // update product_details: Mapping<Hash, Product>
                            let mut details = self.product_details.get(item_id).unwrap_or_default();
                            let oldsum = details.review_count.saturating_mul(details.review_average);
                            let newsum = oldsum.saturating_add(rating);
                            // recalculate the review count
                            details.review_count = details.review_count.saturating_add(1);
                            // recalcualte the review average
                            details.review_average = newsum.div_euclid(details.review_count);
                            // return to storage
                            self.product_details.insert(item_id, &details);

                            // add this review to the list of all reviews for this seller on their profile
                            // instead of seller ratings, just use the aggregate of all product and service ratings
                            // get the seller
                            let seller = details.seller_account;
                            // get the seller profile
                            // account_profile_seller: Mapping<AccountId, SellerProfile>
                            let mut profile = self.account_profile_seller.get(seller).unwrap_or_default();
                            let oldsum = profile.review_count.saturating_mul(profile.review_average);
                            let newsum = oldsum.saturating_add(rating);
                            // recalculate the review count
                            profile.review_count = profile.review_count.saturating_add(1);
                            // recalcualte the review average
                            profile.review_average = newsum.div_euclid(profile.review_count);
                            // return to storage
                            self.account_profile_seller.insert(seller, &profile);

                        }
                        else {
                            if self.service_details.contains(item_id) {
                                // update service_details: Mapping<Hash, Service>
                                let mut details = self.service_details.get(item_id).unwrap_or_default();
                                let oldsum = details.review_count.saturating_mul(details.review_average);
                                let newsum = oldsum.saturating_add(rating);
                                // recalculate the review count
                                details.review_count = details.review_count.saturating_add(1);
                                // recalcualte the review average
                                details.review_average = newsum.div_euclid(details.review_count);
                                // return to storage
                                self.service_details.insert(item_id, &details);

                                // add this review to the list of all reviews for this seller on their profile
                                // instead of seller ratings, just use the aggregate of all product and service ratings
                                // get the seller
                                let seller = details.seller_account;
                                // get the seller profile
                                // account_profile_seller: Mapping<AccountId, SellerProfile>
                                let mut profile = self.account_profile_seller.get(seller).unwrap_or_default();
                                let oldsum = profile.review_count.saturating_mul(profile.review_average);
                                let newsum = oldsum.saturating_add(rating);
                                // recalculate the review count
                                profile.review_count = profile.review_count.saturating_add(1);
                                // recalcualte the review average
                                profile.review_average = newsum.div_euclid(profile.review_count);
                                // return to storage
                                self.account_profile_seller.insert(seller, &profile);

                            }
                            else {
                                return Err(Error::ItemDoesNotExist)
                            }
                        }

                        // EMIT EVENT NewProductRating
                        Self::env().emit_event(NewProductRating {
                            review_id: new_review_id,
                            item_id: item_id,
                            reviewer: caller,
                            rating: rating,
                            review: review,
                            timestamp: now,
                        });

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


        // 7 🟢 Report Problem Damaged
        #[ink(message)]
        pub fn report_problem_damaged (&mut self, 
            order_id: Hash,
            problem_photo_or_youtube_url: Vec<u8>,
            message: Vec<u8>,
        ) -> Result<(), Error> {
            // if the inputs are too big, send an error
            if problem_photo_or_youtube_url.len() > 200 || message.len() > 200 {
                return Err(Error::DataTooLarge);
            }
            // set up clones
            let message_clone = message.clone();
            // set up the caller
            let caller = Self::env().caller();
            // is this your order? account_buyer_orders: Mapping<AccountId, HashVector>
            let myorders = self.account_buyer_orders.get(caller).unwrap_or_default();
            if myorders.hashvector.contains(&order_id) {
                // get the order details order_details: Mapping<Hash, Order>
                let mut details = self.order_details.get(order_id).unwrap_or_default();
                let seller = details.seller;
                // Check the status. To report a problem, the status must be delivered (2)
                // and it must have been less than 24 hours since it was marked delivered
                let now = self.env().block_timestamp();
                let time_since_delivered = now.saturating_sub(details.time_delivered);
                if time_since_delivered < 86400000 && details.order_status == 2 {
                    // make the message_id hash
                    let encodable = (caller, now, order_id, message); // Implements `scale::Encode`
                    let mut new_id_u8 = <Sha2x256 as HashOutput>::Type::default(); // 256-bit buffer
                    ink::env::hash_encoded::<Sha2x256, _>(&encodable, &mut new_id_u8);
                    let new_message_id: Hash = Hash::from(new_id_u8);
                    
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
                    
                    // update order details
                    details.order_status = 4;
                    details.problem = 1;
                    // if there is room in the discussion, add this message
                    if details.discussion.len() < 10 {
                        details.discussion.push(message_details);
                    }
                    // update order_details: Mapping<Hash, Order>
                    self.order_details.insert(order_id, &details);

                    // move the order from seller's orders_2delivered to orders_4problem map
                    let mut delivered = self.account_seller_orders_2delivered.get(seller).unwrap_or_default();
                    let mut problems = self.account_seller_orders_4problem.get(seller).unwrap_or_default();
                    // if the seller's problem list is full, send an Error
                    if problems.hashvector.len() > 69 {
                        return Err(Error::StorageFull);
                    }
                    else {
                        delivered.hashvector.retain(|value| *value != order_id);
                        problems.hashvector.push(order_id);
                        // update the maps
                        self.account_seller_orders_2delivered.insert(seller, &delivered);
                        self.account_seller_orders_4problem.insert(seller, &problems);
                    }
                    
                    // update Buyer profile
                    // account_profile_buyer: Mapping<AccountId, BuyerProfile>
                    let mut buyerprofile = self.account_profile_buyer.get(caller).unwrap_or_default();
                    buyerprofile.total_damaged = buyerprofile.total_damaged.saturating_add(1);
                    buyerprofile.total_delivered = buyerprofile.total_delivered.saturating_sub(1);
                    self.account_profile_buyer.insert(caller, &buyerprofile);

                    // update Seller profile
                    // account_profile_seller: Mapping<AccountId, SellerProfile>
                    let mut sellerprofile = self.account_profile_seller.get(seller).unwrap_or_default();
                    sellerprofile.total_damaged = sellerprofile.total_damaged.saturating_add(1);
                    sellerprofile.total_delivered = sellerprofile.total_delivered.saturating_sub(1);
                    self.account_profile_seller.insert(seller, &sellerprofile);

                    // EMIT EVENT
                    Self::env().emit_event(ProblemReported {
                        order_id: order_id,
                        buyer: caller,
                        seller: seller,
                        problem: 1,
                    });

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


        // 8 🟢 Report Problem Wrong Item 
        #[ink(message)]
        pub fn report_problem_wrong_item (&mut self, 
            order_id: Hash,
            problem_photo_or_youtube_url: Vec<u8>,
            message: Vec<u8>,
        ) -> Result<(), Error> {
            // if the inputs are too big, send an error
            if problem_photo_or_youtube_url.len() > 200 || message.len() > 200 {
                return Err(Error::DataTooLarge);
            }
            // set up clones
            let message_clone = message.clone();
            // set up the caller
            let caller = Self::env().caller();
            // is this your order? account_buyer_orders: Mapping<AccountId, HashVector>
            let myorders = self.account_buyer_orders.get(caller).unwrap_or_default();
            if myorders.hashvector.contains(&order_id) {
                // get the order details order_details: Mapping<Hash, Order>
                let mut details = self.order_details.get(order_id).unwrap_or_default();
                let seller = details.seller;
                // Check the status. To report a problem, the status must be delivered
                // and it must have been less than 24 hours since it was marked delivered
                let now = self.env().block_timestamp();
                let time_since_delivered = now.saturating_sub(details.time_delivered);
                if time_since_delivered < 86400000 && details.order_status == 2 {
                    // make the message_id hash
                    let encodable = (caller, now, order_id, message); // Implements `scale::Encode`
                    let mut new_id_u8 = <Sha2x256 as HashOutput>::Type::default(); // 256-bit buffer
                    ink::env::hash_encoded::<Sha2x256, _>(&encodable, &mut new_id_u8);
                    let new_message_id: Hash = Hash::from(new_id_u8);
                    
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

                    // update order details
                    details.order_status = 4;
                    details.problem = 2;
                    // if there is room in the discussion, add this message
                    if details.discussion.len() < 10 {
                        details.discussion.push(message_details);
                    }
                    // update order_details: Mapping<Hash, Order>
                    self.order_details.insert(order_id, &details);

                    // move the order from seller's orders_2delivered to orders_4problem map
                    let mut delivered = self.account_seller_orders_2delivered.get(seller).unwrap_or_default();
                    let mut problems = self.account_seller_orders_4problem.get(seller).unwrap_or_default();
                    // if the seller's problem list is full, send an Error
                    if problems.hashvector.len() > 69 {
                        return Err(Error::StorageFull);
                    }
                    else {
                        delivered.hashvector.retain(|value| *value != order_id);
                        problems.hashvector.push(order_id);
                        // update the maps
                        self.account_seller_orders_2delivered.insert(seller, &delivered);
                        self.account_seller_orders_4problem.insert(seller, &problems);
                    }

                    // update Buyer profile
                    // account_profile_buyer: Mapping<AccountId, BuyerProfile>
                    let mut buyerprofile = self.account_profile_buyer.get(caller).unwrap_or_default();
                    buyerprofile.total_wrong = buyerprofile.total_wrong.saturating_add(1);
                    buyerprofile.total_delivered = buyerprofile.total_delivered.saturating_sub(1);
                    self.account_profile_buyer.insert(caller, &buyerprofile);

                    // update Seller profile
                    // account_profile_seller: Mapping<AccountId, SellerProfile>
                    let mut sellerprofile = self.account_profile_seller.get(seller).unwrap_or_default();
                    sellerprofile.total_wrong = sellerprofile.total_wrong.saturating_add(1);
                    sellerprofile.total_delivered = sellerprofile.total_delivered.saturating_sub(1);
                    self.account_profile_seller.insert(seller, &sellerprofile);

                    // EMIT EVENT
                    Self::env().emit_event(ProblemReported {
                        order_id: order_id,
                        buyer: caller,
                        seller: seller,
                        problem: 2,
                    });

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
        

        // 9 🟢 Report Problem Not Received
        #[ink(message)]
        pub fn report_problem_not_received (&mut self, 
            order_id: Hash,
            problem_photo_or_youtube_url: Vec<u8>,
            message: Vec<u8>,
        ) -> Result<(), Error> {
            // if the inputs are too big, send an error
            if problem_photo_or_youtube_url.len() > 200 || message.len() > 200 {
                return Err(Error::DataTooLarge);
            }
            // set up clones
            let message_clone = message.clone();
            // set up the caller
            let caller = Self::env().caller();
            // is this your order? account_buyer_orders: Mapping<AccountId, HashVector>
            let myorders = self.account_buyer_orders.get(caller).unwrap_or_default();
            if myorders.hashvector.contains(&order_id) {
                // get the order details order_details: Mapping<Hash, Order>
                let mut details = self.order_details.get(order_id).unwrap_or_default();
                let seller = details.seller;
                // Check the status. To report a problem, the status must be delivered
                // and it must have been less than 24 hours since it was marked delivered
                let now = self.env().block_timestamp();
                let time_since_delivered = now.saturating_sub(details.time_delivered);
                if time_since_delivered < 86400000 && details.order_status == 2 {
                    // make the message_id hash
                    let encodable = (caller, now, order_id, message); // Implements `scale::Encode`
                    let mut new_id_u8 = <Sha2x256 as HashOutput>::Type::default(); // 256-bit buffer
                    ink::env::hash_encoded::<Sha2x256, _>(&encodable, &mut new_id_u8);
                    let new_message_id: Hash = Hash::from(new_id_u8);
                    
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

                    // update order details
                    details.order_status = 4;
                    details.problem = 3;
                    // if there is room in the discussion, add this message
                    if details.discussion.len() < 10 {
                        details.discussion.push(message_details);
                    }
                    // update order_details: Mapping<Hash, Order>
                    self.order_details.insert(order_id, &details);

                    // move the order from seller's orders_2delivered to orders_4problem map
                    let mut delivered = self.account_seller_orders_2delivered.get(seller).unwrap_or_default();
                    let mut problems = self.account_seller_orders_4problem.get(seller).unwrap_or_default();
                    // if the seller's problem list is full, send an Error
                    if problems.hashvector.len() > 69 {
                        return Err(Error::StorageFull);
                    }
                    else {
                        delivered.hashvector.retain(|value| *value != order_id);
                        problems.hashvector.push(order_id);
                        // update the maps
                        self.account_seller_orders_2delivered.insert(seller, &delivered);
                        self.account_seller_orders_4problem.insert(seller, &problems);
                    }

                    // update Buyer profile
                    // account_profile_buyer: Mapping<AccountId, BuyerProfile>
                    let mut buyerprofile = self.account_profile_buyer.get(caller).unwrap_or_default();
                    buyerprofile.total_not_received = buyerprofile.total_not_received.saturating_add(1);
                    buyerprofile.total_delivered = buyerprofile.total_delivered.saturating_sub(1);
                    self.account_profile_buyer.insert(caller, &buyerprofile);

                    // update Seller profile
                    // account_profile_seller: Mapping<AccountId, SellerProfile>
                    let mut sellerprofile = self.account_profile_seller.get(seller).unwrap_or_default();
                    sellerprofile.total_not_received = sellerprofile.total_not_received.saturating_add(1);
                    sellerprofile.total_delivered = sellerprofile.total_delivered.saturating_sub(1);
                    self.account_profile_seller.insert(seller, &sellerprofile);

                    // EMIT EVENT
                    Self::env().emit_event(ProblemReported {
                        order_id: order_id,
                        buyer: caller,
                        seller: seller,
                        problem: 3,
                    });

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


        // 10 🟢 Message The Seller
        #[ink(message)]
        pub fn message_the_seller (&mut self, 
            order_id: Hash,
            photo_or_youtube_url: Vec<u8>,
            message: Vec<u8>,
        ) -> Result<(), Error> {
            // if the inputs are too big, send an error
            if photo_or_youtube_url.len() > 200 || message.len() > 200 {
                return Err(Error::DataTooLarge);
            }
            // set up clones
            let message_clone = message.clone();
            // set up the caller
            let caller = Self::env().caller();
            // is this your order? account_buyer_orders: Mapping<AccountId, HashVector>
            let myorders = self.account_buyer_orders.get(caller).unwrap_or_default();
            if myorders.hashvector.contains(&order_id) {
                // get the order details order_details: Mapping<Hash, Order>
                let mut details = self.order_details.get(order_id).unwrap_or_default();
                let now = self.env().block_timestamp();
                // make the message_id hash
                let encodable = (caller, now, order_id, message); // Implements `scale::Encode`
                let mut new_id_u8 = <Sha2x256 as HashOutput>::Type::default(); // 256-bit buffer
                ink::env::hash_encoded::<Sha2x256, _>(&encodable, &mut new_id_u8);
                let new_message_id: Hash = Hash::from(new_id_u8);
                
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

                // update order discussion
                // if there is room in the discussion, add this message
                if details.discussion.len() < 10 {
                    details.discussion.push(message_details);
                    // update order_details: Mapping<Hash, Order>
                    self.order_details.insert(order_id, &details);
                }
                else {
                    // if this discussion is full, send an error
                    return Err(Error::StorageFull);
                }
                
            }
            else {
                return Err(Error::NotYourOrder)
            }
            Ok(())
        }


        // 11 🟢 Update Buyer Account Settings
        #[ink(message)]
        pub fn update_buyer_account_settings (&mut self, 
            name: Vec<u8>,
            location: Vec<u8>
        ) -> Result<(), Error> {
            // if the inputs are too big, send an error
            if name.len() > 100 || location.len() > 100 {
                return Err(Error::DataTooLarge);
            }
            // set up the caller
            let caller = Self::env().caller();
            // get the caller's Buyer Profile
            // account_profile_buyer: Mapping<AccountId, BuyerProfile>
            let mut profile = self.account_profile_buyer.get(caller).unwrap_or_default();
            // update specific aspects of the BuyerProfile
            profile.buyer_account = caller;
            profile.buyer_name = name.clone();
            profile.buyer_location = location.clone();
            // update mappings
            self.account_profile_buyer.insert(caller, &profile);

            // EMIT EVENT
            Self::env().emit_event(BuyerSettingsUpdated {
                buyer_account: caller,
                name: name,
                location: location,
            });


            Ok(())
        }
 

        // 12 🟢 Update Seller Account Settings
        #[ink(message)]
        pub fn update_seller_account_settings (&mut self, 
            name: Vec<u8>,
            location: Vec<u8>,
            description: Vec<u8>,
            banner_url: Vec<u8>,
            youtube_url: Vec<u8>,
            external_link: Vec<u8>
        ) -> Result<(), Error> {
            if name.len() > 100 || location.len() > 100 || description.len() > 600 
            || banner_url.len() > 200 || youtube_url.len() > 200 || external_link.len() > 200 {
                return Err(Error::DataTooLarge);
            }
            // set up the caller
            let caller = Self::env().caller();
            // get the caller's Seller Profile
            // account_profile_seller: Mapping<AccountId, SellerProfile>
            let mut profile = self.account_profile_seller.get(caller).unwrap_or_default();
            // update specific aspects of the SellerProfile
            profile.seller_account = caller;
            profile.seller_name = name.clone();
            profile.seller_location = location.clone();
            profile.store_description = description.clone();
            profile.banner_url = banner_url.clone();
            profile.youtube_url = youtube_url.clone();
            profile.external_link = external_link.clone();
            // update mappings
            self.account_profile_seller.insert(caller, &profile);

            // EMIT EVENT
            Self::env().emit_event(SellerSettingsUpdated {
                seller_account: caller,
                seller_name: name,
                store_description: description,
                seller_location: location,
                banner_url: banner_url,
                youtube_url: youtube_url,
                external_link: external_link,
            });

            Ok(())
        }


        // 13 🟢 Update Order Tracking Information 
        #[ink(message)]
        pub fn update_order_tracking_information (&mut self, 
            order_id: Hash,
            tracking_update: Vec<u8>,
            shipped: bool,
            delivered: bool
        ) -> Result<(), Error> {
            // if the inputs are too big, send an error
            if tracking_update.len() > 200 {
                return Err(Error::DataTooLarge);
            }
            // set up the caller
            let caller = Self::env().caller();
            // make sure the caller is the seller on this order
            // get the order details and compare to the seller
            let mut details = self.order_details.get(order_id).unwrap_or_default();
            if details.seller == caller {
                // make sure the order is a physical product (not a service, or digital product)
                let itemid = details.item_id;
                if self.product_details.contains(itemid) {
                    let item = self.product_details.get(itemid).unwrap_or_default();
                    if !item.digital {

                        details.tracking_info = tracking_update;

                        if details.order_status == 1 {
                            // seller can mark delivered, or can leave it as shipped
                            if delivered {
                                details.order_status = 2;
                                details.time_delivered = self.env().block_timestamp();
                                // update order_details: Mapping<Hash, Order> 
                                self.order_details.insert(order_id, &details);

                                // move this order from orders_1shipped to orders_2delivered 
                                let mut shipped = self.account_seller_orders_1shipped.get(caller).unwrap_or_default();
                                let mut delivered = self.account_seller_orders_2delivered.get(caller).unwrap_or_default();
                                // if the seller's delivered list is full, kick out the oldest
                                if delivered.hashvector.len() > 69 {
                                    delivered.hashvector.remove(0);
                                }
                                shipped.hashvector.retain(|value| *value != order_id);
                                delivered.hashvector.push(order_id);
                                // update the maps
                                self.account_seller_orders_2delivered.insert(caller, &delivered);
                                self.account_seller_orders_1shipped.insert(caller, &shipped);

                                // EMIT EVENT OrderDelivered
                                Self::env().emit_event(OrderDelivered {
                                    seller: caller,
                                    buyer: details.buyer,
                                    order_id: details.order_id,
                                    order_status: 2,
                                    time_delivered: self.env().block_timestamp(),
                                });

                                // update Buyer profile
                                // account_profile_buyer: Mapping<AccountId, BuyerProfile>
                                let buyer = details.buyer;
                                let mut buyerprofile = self.account_profile_buyer.get(buyer).unwrap_or_default();
                                buyerprofile.total_delivered = buyerprofile.total_delivered.saturating_add(1);
                                self.account_profile_buyer.insert(buyer, &buyerprofile);

                                // update Seller profile
                                // account_profile_seller: Mapping<AccountId, SellerProfile>
                                let mut sellerprofile = self.account_profile_seller.get(caller).unwrap_or_default();
                                sellerprofile.total_delivered = sellerprofile.total_delivered.saturating_add(1);
                                self.account_profile_seller.insert(caller, &sellerprofile);
                                
                            }
                        }

                        if details.order_status == 0 {
                            // seller can mark shipped but not delivered
                            if shipped {
                                details.order_status = 1;
                                // update order_details: Mapping<Hash, Order> 
                                self.order_details.insert(order_id, &details);

                                // move this order from orders_0awaiting to orders_1shipped
                                let mut shipped = self.account_seller_orders_1shipped.get(caller).unwrap_or_default();
                                let mut awaiting = self.account_seller_orders_0awaiting.get(caller).unwrap_or_default();
                                // if the seller's delivered list is full, kick out the oldest
                                if shipped.hashvector.len() > 69 {
                                    shipped.hashvector.remove(0);
                                }
                                awaiting.hashvector.retain(|value| *value != order_id);
                                shipped.hashvector.push(order_id);
                                // update the maps
                                self.account_seller_orders_0awaiting.insert(caller, &awaiting);
                                self.account_seller_orders_1shipped.insert(caller, &shipped);

                                // EMIT EVENT OrderShipped
                                Self::env().emit_event(OrderShipped {
                                    seller: caller,
                                    buyer: details.buyer,
                                    order_id: details.order_id,
                                    order_status: 1,
                                });

                                // calculate payments to seller and zeno buyers
                                let seller = details.seller;
                                let total_price: Balance = details.total_order_price;
                                let zeno_total: Balance = details.zeno_total;
                                let zeno_buyers = item.zeno_buyers;
                                let seller_payout: Balance = total_price.saturating_sub(zeno_total);

                                // pay the seller 
                                if self.env().transfer(seller, seller_payout).is_err() {
                                    return Err(Error::PayoutFailed);
                                }

                                // If the zeno_total is not zero, initiate the zeno payouts
                                if zeno_total > 0 {
                                    // pay all zeno buyers
                                    let mut remainder: Balance = zeno_total;
                                    for (n, affiliate) in zeno_buyers.iter().enumerate() {
                                        // n needs to be a u128, convert it and add 1
                                        let m: u32 = n.try_into().unwrap();
                                        let p: u32 = m.saturating_add(1);
                                        // let fraction: u32 = 2u32.saturating_pow(p);
                                        // let payment: Option<Balance> = Some(0);
                                        let payment: Balance = zeno_total.checked_div(2u128.saturating_pow(p)).unwrap();
                                        if self.env().transfer(*affiliate, payment).is_err() {
                                            return Err(Error::PayoutFailed);
                                        }
                                        remainder = remainder.saturating_sub(payment);
                                    }
                                    // pay the seller any remainder from the zeno payouts
                                    if remainder > 0 {
                                        if self.env().transfer(seller, remainder).is_err() {
                                         return Err(Error::PayoutFailed);
                                        }
                                    }
                                    
                                }
                                
                            }
                        }

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


        // 14 🟢 Refuse An Order
        #[ink(message)]
        pub fn refuse_an_order (&mut self, 
            order_id: Hash
        ) -> Result<(), Error> {
            // set up the caller
            let caller = Self::env().caller();
            // make sure the caller is the seller on this order
            let mut details = self.order_details.get(order_id).unwrap_or_default();
            if details.seller == caller {
                // you can only refuse an order that has not yet shipped. Status must be 0.
                if details.order_status == 0 {
                    
                    // issue a refund to the buyer for this order
                    let buyer = details.buyer;
                    let refund: Balance = details.total_order_price;
                    if self.env().transfer(buyer, refund).is_err() {
                        return Err(Error::PayoutFailed);
                    }

                    // update order_details: Mapping<Hash, Order>
                    details.order_status = 5;
                    self.order_details.insert(order_id, &details);

                    // move the order from orders_0awaiting to orders_5refused
                    let mut refused = self.account_seller_orders_5refused.get(caller).unwrap_or_default();
                    let mut awaiting = self.account_seller_orders_0awaiting.get(caller).unwrap_or_default();
                    // if the seller's refused list is full, kick out the oldest
                    if refused.hashvector.len() > 69 {
                        refused.hashvector.remove(0);
                    }
                    awaiting.hashvector.retain(|value| *value != order_id);
                    refused.hashvector.push(order_id);
                    // update the maps
                    self.account_seller_orders_0awaiting.insert(caller, &awaiting);
                    self.account_seller_orders_1shipped.insert(caller, &refused);

                    // update Buyer profile
                    // account_profile_buyer: Mapping<AccountId, BuyerProfile>
                    let mut buyerprofile = self.account_profile_buyer.get(buyer).unwrap_or_default();
                    buyerprofile.total_refused = buyerprofile.total_refused.saturating_add(1);
                    self.account_profile_buyer.insert(buyer, &buyerprofile);

                    // update Seller profile
                    // account_profile_seller: Mapping<AccountId, SellerProfile>
                    let mut sellerprofile = self.account_profile_seller.get(caller).unwrap_or_default();
                    sellerprofile.total_refused = sellerprofile.total_refused.saturating_add(1);
                    self.account_profile_seller.insert(caller, &sellerprofile);

                    // EMIT EVENT OrderRefused 
                    Self::env().emit_event(OrderRefused {
                        seller: caller,
                        buyer: details.buyer,
                        order_id: details.order_id,
                        order_status: 5,
                    });

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


        // 15 🟢 Issue Refund
        // note that refunds are issued as a resolution to a problem
        // seller inputs the refund amount from their own account since payouts already happened
        #[ink(message, payable)]
        pub fn issue_refund (&mut self, 
            order_id: Hash,
        ) -> Result<(), Error> {
            // set up the caller
            let caller = Self::env().caller();
            // make sure the caller is the seller on this order
            let mut details = self.order_details.get(order_id).unwrap_or_default();
            if details.seller == caller {
                // you can only refund an order that has a problem and no resolution. 
                // Status must be 4 and resolution must be 0.
                
                if details.order_status == 4 && details.resolution == 0 {

                    // COLLECT PAYMENT FROM THE CALLER
                    // the 'payable' tag on this message allows the user to send any amount
                    let refund: Balance = self.env().transferred_value();
                    
                    // issue a refund to the buyer for this order
                    let buyer = details.buyer;
                    if self.env().transfer(buyer, refund).is_err() {
                        return Err(Error::PayoutFailed);
                    }

                    // update order_details: Mapping<Hash, Order>
                    details.resolution = 1;
                    details.order_status = 3;
                    self.order_details.insert(order_id, &details);

                    // move the order from orders_4problem to orders_3resolved
                    let mut problem = self.account_seller_orders_4problem.get(caller).unwrap_or_default();
                    let mut resolved = self.account_seller_orders_3resolved.get(caller).unwrap_or_default();
                    // if the seller's refused list is full, kick out the oldest
                    if resolved.hashvector.len() > 69 {
                        resolved.hashvector.remove(0);
                    }
                    problem.hashvector.retain(|value| *value != order_id);
                    resolved.hashvector.push(order_id);
                    // update the maps
                    self.account_seller_orders_4problem.insert(caller, &problem);
                    self.account_seller_orders_3resolved.insert(caller, &resolved);

                    // update Buyer profile
                    // account_profile_buyer: Mapping<AccountId, BuyerProfile>
                    let mut buyerprofile = self.account_profile_buyer.get(buyer).unwrap_or_default();
                    buyerprofile.total_resolved = buyerprofile.total_resolved.saturating_add(1);
                    self.account_profile_buyer.insert(buyer, &buyerprofile);

                    // update Seller profile
                    // account_profile_seller: Mapping<AccountId, SellerProfile>
                    let mut sellerprofile = self.account_profile_seller.get(caller).unwrap_or_default();
                    sellerprofile.total_resolved = sellerprofile.total_resolved.saturating_add(1);
                    self.account_profile_seller.insert(caller, &sellerprofile);

                    // EMIT EVENT OrderRefunded
                    Self::env().emit_event(OrderRefunded {
                        seller: caller,
                        buyer: details.buyer,
                        order_id: details.order_id,
                        order_status: 3,
                        problem: details.problem,
                        resolution: 1,
                    });

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
        
        
        // 16 🟢 Issue Replacement
        #[ink(message)]
        pub fn issue_replacement (&mut self, 
            order_id: Hash,
            tracking: Vec<u8>
        ) -> Result<(), Error> {
            // make sure the tracking info is not too long
            if tracking.len() > 200 {
                return Err(Error::DataTooLarge);
            }
            // set up the caller
            let caller = Self::env().caller();

            // make sure the caller is the seller on this order
            let mut details = self.order_details.get(order_id).unwrap_or_default();
            if details.seller == caller {
                // you can only replace an order that has a problem. Status must be 4.
                if details.order_status == 4 && details.resolution == 0 {
                    
                    // update order_details: Mapping<Hash, Order>
                    // update the shipping details with the replacement item
                    details.tracking_info = tracking;
                    // update the resolution code
                    details.resolution = 2;
                    // update the status to resolved
                    details.order_status = 3;
                    // update the mapping
                    self.order_details.insert(order_id, &details);

                    // move the order from orders_4problem to orders_3resolved
                    let mut problem = self.account_seller_orders_4problem.get(caller).unwrap_or_default();
                    let mut resolved = self.account_seller_orders_3resolved.get(caller).unwrap_or_default();
                    // if the seller's refused list is full, kick out the oldest
                    if resolved.hashvector.len() > 69 {
                        resolved.hashvector.remove(0);
                    }
                    problem.hashvector.retain(|value| *value != order_id);
                    resolved.hashvector.push(order_id);
                    // update the maps
                    self.account_seller_orders_4problem.insert(caller, &problem);
                    self.account_seller_orders_3resolved.insert(caller, &resolved);

                    // update Buyer profile
                    let buyer = details.buyer;
                    // account_profile_buyer: Mapping<AccountId, BuyerProfile>
                    let mut buyerprofile = self.account_profile_buyer.get(buyer).unwrap_or_default();
                    buyerprofile.total_resolved = buyerprofile.total_resolved.saturating_add(1);
                    self.account_profile_buyer.insert(buyer, &buyerprofile);

                    // update Seller profile
                    // account_profile_seller: Mapping<AccountId, SellerProfile>
                    let mut sellerprofile = self.account_profile_seller.get(caller).unwrap_or_default();
                    sellerprofile.total_resolved = sellerprofile.total_resolved.saturating_add(1);
                    self.account_profile_seller.insert(caller, &sellerprofile);

                    // EMIT EVENT OrderReplaced
                    Self::env().emit_event(OrderReplaced {
                        seller: caller,
                        buyer: details.buyer,
                        order_id: details.order_id,
                        order_status: 3,
                        problem: details.problem,
                        resolution: 2,
                    });

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
       
       
        // 17 🟢 Deny Resolution Request
        #[ink(message)]
        pub fn deny_resolution_request (&mut self, 
            order_id: Hash,
        ) -> Result<(), Error> {
            // set up the caller
            let caller = Self::env().caller();
            // make sure the caller is the seller on this order
            let mut details = self.order_details.get(order_id).unwrap_or_default();
            if details.seller == caller {
                // you can only deny resolution on an order that has a problem. Status must be 4.
                if details.order_status == 4 && details.resolution == 0 {
                    
                    // update order_details: Mapping<Hash, Order>
                    details.resolution = 3;
                    details.order_status = 3;
                    self.order_details.insert(order_id, &details);

                    // move the order from orders_4problem to orders_3resolved
                    let mut problem = self.account_seller_orders_4problem.get(caller).unwrap_or_default();
                    let mut resolved = self.account_seller_orders_3resolved.get(caller).unwrap_or_default();
                    // if the seller's refused list is full, kick out the oldest
                    if resolved.hashvector.len() > 69 {
                        resolved.hashvector.remove(0);
                    }
                    problem.hashvector.retain(|value| *value != order_id);
                    resolved.hashvector.push(order_id);
                    // update the maps
                    self.account_seller_orders_4problem.insert(caller, &problem);
                    self.account_seller_orders_3resolved.insert(caller, &resolved);

                    // EMIT EVENT OrderResolutionDenied
                    Self::env().emit_event(OrderResolutionDenied {
                        seller: caller,
                        buyer: details.buyer,
                        order_id: details.order_id,
                        order_status: 3,
                        problem: details.problem,
                        resolution: 3,
                    });

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
        
        
        // 18 🟢 Message The Buyer
        #[ink(message)]
        pub fn message_the_buyer (&mut self, 
            order_id: Hash,
            photo_or_youtube_url: Vec<u8>,
            message: Vec<u8>,
        ) -> Result<(), Error> {
            // if the inputs are too big, send an error
            if photo_or_youtube_url.len() > 200 || message.len() > 200 {
                return Err(Error::DataTooLarge);
            }
            // set up clones
            let message_clone = message.clone();
            // set up the caller
            let caller = Self::env().caller();
            // is this your order? 
            // get the order details from order_details: Mapping<Hash, Order>
            let mut details = self.order_details.get(order_id).unwrap_or_default();
            if details.seller == caller {
                
                let now = self.env().block_timestamp();
                // make the message_id hash
                let encodable = (caller, now, order_id, message); // Implements `scale::Encode`
                let mut new_id_u8 = <Sha2x256 as HashOutput>::Type::default(); // 256-bit buffer
                ink::env::hash_encoded::<Sha2x256, _>(&encodable, &mut new_id_u8);
                let new_message_id: Hash = Hash::from(new_id_u8);
                
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

                // update order discussion
                // if there is room in the discussion, add the message, otherwise error
                if details.discussion.len() < 10 {
                    details.discussion.push(message_details);
                    // update order_details: Mapping<Hash, Order>
                    self.order_details.insert(order_id, &details);
                }
                else {
                    return Err(Error::StorageFull);
                }
                
            }
            else {
                return Err(Error::NotYourOrder)
            }
            Ok(())
        }


        // 19 🟢 Rate A Buyer
        #[ink(message)]
        pub fn rate_a_buyer (&mut self, 
            buyer: AccountId,
            rating: u64,
            review: Vec<u8>,
            order_id: Hash
        ) -> Result<(), Error> {
            // if the rating is between 1 and 5
            if rating > 0 && rating < 6 {
                let caller = Self::env().caller();
                let review_clone = review.clone();
                let now = self.env().block_timestamp();
                
                // get the details for this order
                let details = self.order_details.get(order_id).unwrap_or_default();
                // account_seller_buyers_reviewed: Mapping<AccountId, AccountVector>
                let mut reviewed = self.account_seller_buyers_reviewed.get(caller).unwrap_or_default();

                // did you actually sell this order to this buyer?
                if details.seller == caller {
                    // have you already reviewed this buyer recently?
                    if reviewed.accountvector.contains(&buyer) {
                        return Err(Error::NotEligibleToReview)
                    }
                    else {
                        // make the review_id hash
                        let encodable = (caller, buyer, review); // Implements `scale::Encode`
                        let mut new_id_u8 = <Sha2x256 as HashOutput>::Type::default(); // 256-bit buffer
                        ink::env::hash_encoded::<Sha2x256, _>(&encodable, &mut new_id_u8);
                        let new_review_id: Hash = Hash::from(new_id_u8);

                        // update mappings...
                        // account_seller_buyers_reviewed: Mapping<AccountId, AccountVector>
                        if reviewed.accountvector.len() > 399 {
                            // remove the oldest
                            reviewed.accountvector.remove(0);
                        }
                        reviewed.accountvector.push(buyer);
                        self.account_seller_buyers_reviewed.insert(caller, &reviewed);

                        // account_profile_buyer: Mapping<AccountId, BuyerProfile>
                        let mut profile = self.account_profile_buyer.get(buyer).unwrap_or_default();
                        let oldsum = profile.review_count.saturating_mul(profile.review_average);
                        let newsum = oldsum.saturating_add(rating);
                        // recalculate the review count
                        profile.review_count = profile.review_count.saturating_add(1);
                        // recalcualte the review average
                        profile.review_average = newsum.div_euclid(profile.review_count);
                        // return to storage
                        self.account_profile_buyer.insert(buyer, &profile);

                        // EMIT EVENT NewBuyerRating
                        Self::env().emit_event(NewBuyerRating {
                            review_id: new_review_id,
                            buyer: buyer,
                            reviewer: caller,
                            rating: rating,
                            review: review_clone,
                            timestamp: now,
                        });


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
        
        
        // 20 🟢 Add A Product
        #[ink(message)]
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
            // if the inputs are too big, send an error
            if title.len() > 200 || brand.len() > 100 || category.len() > 100 || description.len() > 600
            || photo_or_youtube_link1.len() > 200 || photo_or_youtube_link2.len() > 200
            || photo_or_youtube_link3.len() > 200 || more_info_link.len() > 200
            || delivery_info.len() > 200 || product_location.len() > 200 || digital_file_url.len() > 600 {
                return Err(Error::DataTooLarge);
            }

            // set up clones
            let title_clone = title.clone();
            // set up the caller
            let caller = Self::env().caller();

            let mut seller_products = self.account_seller_products.get(caller).unwrap_or_default();
            // if the seller's product list is full, send an error
            if seller_products.hashvector.len() > 49 {
                return Err(Error::StorageFull);
            }
            else {

                let now = self.env().block_timestamp();

                // make the product id hash
                let encodable = (caller, title, now); // Implements `scale::Encode`
                let mut new_id_u8 = <Sha2x256 as HashOutput>::Type::default(); // 256-bit buffer
                ink::env::hash_encoded::<Sha2x256, _>(&encodable, &mut new_id_u8);
                let new_product_id: Hash = Hash::from(new_id_u8);

                // get the seller profile 
                let mut sellerprofile = self.account_profile_seller.get(caller).unwrap_or_default();

                // set up the product details
                let product = Product {
                    product_id: new_product_id,
                    digital: digital,
                    title: title_clone.clone(),
                    price: price,
                    brand: brand.clone(),
                    category: category.clone(),
                    seller_account: caller,
                    seller_name: sellerprofile.seller_name.clone(),
                    description: description.clone(), 
                    review_average: u64::default(),
                    review_count: u64::default(),
                    inventory: inventory, 
                    photo_or_youtube_link1: photo_or_youtube_link1.clone(), 
                    photo_or_youtube_link2: photo_or_youtube_link2.clone(),
                    photo_or_youtube_link3: photo_or_youtube_link3.clone(),
                    more_info_link: more_info_link.clone(),
                    delivery_info: delivery_info.clone(),
                    product_location: product_location.clone(),
                    digital_file_url: digital_file_url,
                    zeno_percent: zeno_percent,
                    zeno_buyers: <Vec<AccountId>>::default(),
                };

                // UPDATE MAPPINGS...
                // total_count_products: u128
                self.total_count_products = self.total_count_products.saturating_add(1);

                // account_seller_products: Mapping<AccountId, HashVector>
                seller_products.hashvector.push(new_product_id);
                self.account_seller_products.insert(caller, &seller_products);

                // product_details: Mapping<Hash, Product>
                self.product_details.insert(new_product_id, &product);

                // if this is the first product, set the member_since timestamp
                if sellerprofile.member_since == u64::default() {
                    sellerprofile.member_since = now;
                    // add this seller to all_sellers StorageVec
                    self.all_sellers.push(&caller);
                }
                self.account_profile_seller.insert(caller, &sellerprofile);

                // EMIT EVENT NewProduct
                Self::env().emit_event(NewProduct {
                    product_id: new_product_id,
                    digital: digital,
                    title: title_clone,
                    price: price,
                    brand: brand,
                    category: category,
                    seller_account: caller,
                    seller_name: sellerprofile.seller_name,
                    description: description, 
                    inventory: inventory, 
                    photo_or_youtube_link1: photo_or_youtube_link1, 
                    photo_or_youtube_link2: photo_or_youtube_link2,
                    photo_or_youtube_link3: photo_or_youtube_link3,
                    more_info_link: more_info_link,
                    delivery_info: delivery_info,
                    product_location: product_location,
                    zeno_percent: zeno_percent,
                });


            }
            
            Ok(())
        }

        
        // 21 🟢 Update Product Details 
        #[ink(message)]
        pub fn update_product_details (&mut self,
            product_id: Hash, 
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
            if brand.len() > 100 || category.len() > 100 || description.len() > 600
            || photo_or_youtube_link1.len() > 200 || photo_or_youtube_link2.len() > 200
            || photo_or_youtube_link3.len() > 200 || more_info_link.len() > 200
            || delivery_info.len() > 200 || product_location.len() > 200 || digital_file_url.len() > 600 {
                return Err(Error::DataTooLarge);
            }
            // set up the caller
            let caller = Self::env().caller();

            // is this your product? Check the product id
            // account_seller_products: Mapping<AccountId, HashVector>
            let seller_products = self.account_seller_products.get(caller).unwrap_or_default();
            if seller_products.hashvector.contains(&product_id) {
                
                // get the current product details
                let details = self.product_details.get(product_id).unwrap_or_default();

                // set up the product details update
                let update = Product {
                    product_id: product_id,
                    digital: details.digital,
                    title: details.title.clone(),
                    price: price,
                    brand: brand.clone(),
                    category: category.clone(),
                    seller_account: caller,
                    seller_name: details.seller_name.clone(),
                    description: description.clone(),
                    review_average: details.review_average,
                    review_count: details.review_count, 
                    inventory: inventory, 
                    photo_or_youtube_link1: photo_or_youtube_link1.clone(), 
                    photo_or_youtube_link2: photo_or_youtube_link2.clone(),
                    photo_or_youtube_link3: photo_or_youtube_link3.clone(),
                    more_info_link: more_info_link.clone(),
                    delivery_info: delivery_info.clone(),
                    product_location: product_location.clone(),
                    digital_file_url: digital_file_url,
                    zeno_percent: details.zeno_percent,
                    zeno_buyers: details.zeno_buyers,
                };

                // update product_details: Mapping<Hash, Product>
                self.product_details.insert(product_id, &update);

                // EMIT EVENT UpdatedProduct
                Self::env().emit_event(UpdatedProduct {
                    product_id: product_id,
                    digital: details.digital,
                    title: details.title,
                    price: price,
                    brand: brand,
                    category: category,
                    seller_account: caller,
                    seller_name: details.seller_name,
                    description: description, 
                    inventory: inventory, 
                    photo_or_youtube_link1: photo_or_youtube_link1, 
                    photo_or_youtube_link2: photo_or_youtube_link2,
                    photo_or_youtube_link3: photo_or_youtube_link3,
                    more_info_link: more_info_link,
                    delivery_info: delivery_info,
                    product_location: product_location,
                });

            }
            
            else {
                return Err(Error::NotYourProduct)
            }

            Ok(())
        }
        

        // 22 🟢 Add A Service
        #[ink(message)]
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
            if title.len() > 200 || category.len() > 100 || description.len() > 600
            || photo_or_youtube_link1.len() > 200 || photo_or_youtube_link2.len() > 200
            || photo_or_youtube_link3.len() > 200 || booking_link.len() > 200
            || service_location.len() > 200 {
                return Err(Error::DataTooLarge);
            }
            // set up clones
            let title_clone = title.clone();
            // set up the caller
            let caller = Self::env().caller();

            let mut seller_services = self.account_seller_services.get(caller).unwrap_or_default();
            // if the seller's service list is full, send an error
            if seller_services.hashvector.len() > 49 {
                return Err(Error::StorageFull);
            }
            else {
                let now = self.env().block_timestamp();

                // make the product id hash
                let encodable = (caller, title, now); // Implements `scale::Encode`
                let mut new_id_u8 = <Sha2x256 as HashOutput>::Type::default(); // 256-bit buffer
                ink::env::hash_encoded::<Sha2x256, _>(&encodable, &mut new_id_u8);
                let new_service_id: Hash = Hash::from(new_id_u8);

                // get the seller profile 
                let mut sellerprofile = self.account_profile_seller.get(caller).unwrap_or_default();

                // set up the service details
                let service = Service {
                    service_id: new_service_id,
                    online: online,
                    title: title_clone.clone(),
                    price: price,
                    category: category.clone(),
                    seller_account: caller,
                    seller_name: sellerprofile.seller_name.clone(),
                    description: description.clone(),
                    review_average: u64::default(),
                    review_count: u64::default(),
                    inventory: inventory,
                    photo_or_youtube_link1: photo_or_youtube_link1.clone(), 
                    photo_or_youtube_link2: photo_or_youtube_link2.clone(),
                    photo_or_youtube_link3: photo_or_youtube_link3.clone(),
                    booking_link: booking_link.clone(),
                    service_location: service_location.clone(),
                    zeno_percent: zeno_percent,
                    zeno_buyers: <Vec<AccountId>>::default(),
                };

                // UPDATE MAPPINGS ...
                // total_count_services u128
                self.total_count_services = self.total_count_services.saturating_add(1);

                // account_seller_services: Mapping<AccountId, HashVector>
                seller_services.hashvector.push(new_service_id);
                self.account_seller_services.insert(caller, &seller_services);

                // service_details: Mapping<Hash, Service>
                self.service_details.insert(new_service_id, &service);

                // if this is the first product or service, set the member_since timestamp
                // and add the seller to the all_sellers StorageVec
                if sellerprofile.member_since == u64::default() {
                    sellerprofile.member_since = now;
                    self.all_sellers.push(&caller);
                }
                self.account_profile_seller.insert(caller, &sellerprofile);

                // EMIT EVENT NewService
                Self::env().emit_event(NewService {
                    service_id: new_service_id,
                    online: online,
                    title: title_clone,
                    price: price,
                    category: category,
                    seller_account: caller,
                    seller_name: sellerprofile.seller_name,
                    description: description,
                    inventory: inventory,
                    photo_or_youtube_link1: photo_or_youtube_link1, 
                    photo_or_youtube_link2: photo_or_youtube_link2,
                    photo_or_youtube_link3: photo_or_youtube_link3,
                    booking_link: booking_link,
                    service_location: service_location,
                    zeno_percent: zeno_percent,
                });

            }
            
            Ok(())
        }
        

        // 23 🟢 Update Service Details
        #[ink(message)]
        pub fn update_service_details (&mut self,
            service_id: Hash, 
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
            if category.len() > 100 || description.len() > 600
            || photo_or_youtube_link1.len() > 200 || photo_or_youtube_link2.len() > 200
            || photo_or_youtube_link3.len() > 200 || booking_link.len() > 200
            || service_location.len() > 200 {
                return Err(Error::DataTooLarge);
            }
            // set up the caller
            let caller = Self::env().caller();

            // is this your service? Check the service id
            // account_seller_services: Mapping<AccountId, HashVector>
            let seller_services = self.account_seller_services.get(caller).unwrap_or_default();
            if seller_services.hashvector.contains(&service_id) {
                
                // get the current service details
                let details = self.service_details.get(service_id).unwrap_or_default();

                // set up the service details update
                let update = Service {
                    service_id: service_id,
                    online: details.online,
                    title: details.title.clone(),
                    price: price,
                    category: category.clone(),
                    seller_account: caller,
                    seller_name: details.seller_name.clone(),
                    description: description.clone(),
                    review_average: details.review_average,
                    review_count: details.review_count,
                    inventory: inventory,
                    photo_or_youtube_link1: photo_or_youtube_link1.clone(), 
                    photo_or_youtube_link2: photo_or_youtube_link2.clone(),
                    photo_or_youtube_link3: photo_or_youtube_link3.clone(),
                    booking_link: booking_link.clone(),
                    service_location: service_location.clone(),
                    zeno_percent: details.zeno_percent,
                    zeno_buyers: details.zeno_buyers,
                };

                // update service_details: Mapping<Hash, Service>
                self.service_details.insert(service_id, &update);

                // EMIT EVENT UpdatedService
                Self::env().emit_event(UpdatedService {
                    service_id: service_id,
                    online: details.online,
                    title: details.title,
                    price: price,
                    category: category,
                    seller_account: caller,
                    seller_name: details.seller_name,
                    description: description,
                    inventory: inventory,
                    photo_or_youtube_link1: photo_or_youtube_link1, 
                    photo_or_youtube_link2: photo_or_youtube_link2,
                    photo_or_youtube_link3: photo_or_youtube_link3,
                    booking_link: booking_link,
                    service_location: service_location,
                });

            }
            else {
                return Err(Error::NotYourProduct)
            }

            Ok(())
        }


        // 24 🟢 Delete A Product
        #[ink(message)]
        pub fn delete_a_product (&mut self, product_id_to_delete: Hash) -> Result<(), Error> {
            // set up the caller
            let caller = Self::env().caller();
            // is this your product?
            // get the seller's products
            let mut products = self.account_seller_products.get(caller).unwrap_or_default();
            if products.hashvector.contains(&product_id_to_delete) {
                // remove this product from account_seller_products
                products.hashvector.retain(|value| *value != product_id_to_delete);
                self.account_seller_products.insert(caller, &products);
                // remove this product from product_details
                self.product_details.remove(product_id_to_delete);
                // reduce total_count_products by one
                self.total_count_products = self.total_count_products.saturating_sub(1);
            }
            else {
                return Err(Error::NotYourProduct);
            }
            
            Ok(())
        }

        // 25 🟢 Delete A Service
        #[ink(message)]
        pub fn delete_a_service (&mut self, service_id_to_delete: Hash) -> Result<(), Error> {
            // set up the caller
            let caller = Self::env().caller();
            // is this your service?
            // get the seller's services
            let mut services = self.account_seller_services.get(caller).unwrap_or_default();
            if services.hashvector.contains(&service_id_to_delete) {
                // remove this service from account_seller_services
                services.hashvector.retain(|value| *value != service_id_to_delete);
                self.account_seller_services.insert(caller, &services);
                // remove this service from service_details
                self.service_details.remove(service_id_to_delete);
                // reduce total_count_services by one
                self.total_count_services = self.total_count_services.saturating_sub(1);
            }
            else {
                return Err(Error::NotYourProduct);
            }
            
            Ok(())
        }


        // >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
        // >>>>>>>>>>>>>>>>>>>>>>>>>> PRIMARY GET MESSAGES <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<
        // >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
 

        // 26 🟢 Search Products By Keyword
        #[ink(message)]
        pub fn search_products_by_keyword (&self,
            keywords1: Vec<u8>,
            keywords2: Vec<u8>,
            keywords3: Vec<u8>
        ) -> ProductSearchResults {
            // set up the search targets
            let targetvecu81 = keywords1.clone();
            let target_string1 = String::from_utf8(targetvecu81).unwrap_or_default();
            let targetvecu82 = keywords2.clone();
            let target_string2 = String::from_utf8(targetvecu82).unwrap_or_default();
            let targetvecu83 = keywords3.clone();
            let target_string3 = String::from_utf8(targetvecu83).unwrap_or_default();

            // set up return structures
            let mut product_results = <Vec<PublicProduct>>::default();

            // iterate over all_sellers: StorageVec<AccountId> to find matching results
            if self.all_sellers.len() > 0 {
                for i in 0..self.all_sellers.len() {
                    let seller = self.all_sellers.get(i).unwrap();
                    // get the seller's products
                    let seller_products = self.account_seller_products.get(seller).unwrap_or_default();
                    for item in seller_products.hashvector.iter() {
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

                        // if the target_string is in the details
                        if (title_string.contains(&target_string1) || brand_string.contains(&target_string1) ||
                        category_string.contains(&target_string1) || description_string.contains(&target_string1) ||
                        delivery_string.contains(&target_string1) || location_string.contains(&target_string1) ||
                        seller_string.contains(&target_string1))
                        && (title_string.contains(&target_string2) || brand_string.contains(&target_string2) ||
                        category_string.contains(&target_string2) || description_string.contains(&target_string2) ||
                        delivery_string.contains(&target_string2) || location_string.contains(&target_string2) ||
                        seller_string.contains(&target_string2))
                        && (title_string.contains(&target_string3) || brand_string.contains(&target_string3) ||
                        category_string.contains(&target_string3) || description_string.contains(&target_string3) ||
                        delivery_string.contains(&target_string3) || location_string.contains(&target_string3) ||
                        seller_string.contains(&target_string3)) {
                            
                            // make the public product structure
                            let public_product = PublicProduct {
                                product_id: details.product_id,
                                digital: details.digital,
                                title: details.title,
                                price: details.price,
                                brand: details.brand,
                                category: details.category,
                                seller_account: details.seller_account,
                                seller_name: details.seller_name,
                                description: details.description,
                                review_average: details.review_average,
                                review_count: details.review_count,
                                inventory: details.inventory, 
                                photo_or_youtube_link1: details.photo_or_youtube_link1, 
                                photo_or_youtube_link2: details.photo_or_youtube_link2,
                                photo_or_youtube_link3: details.photo_or_youtube_link3,
                                more_info_link: details.more_info_link,
                                delivery_info: details.delivery_info,
                                product_location: details.product_location,
                                zeno_percent: details.zeno_percent,
                                zeno_buyers: details.zeno_buyers
                            };

                            // add it to the results vector
                            product_results.push(public_product);
                        }
                        // continue iterating on this seller's products
                    }
                    // continue iterating on all the sellers
                }
            }

            // package the results
            let results = ProductSearchResults {
                search: vec![keywords1, keywords2, keywords3],
                products: product_results
            };

            // return the results
            results
        }


        // 27 🟢 Search Services By Keyword
        #[ink(message)]
        pub fn search_services_by_keyword (&self,
            keywords1: Vec<u8>,
            keywords2: Vec<u8>,
            keywords3: Vec<u8>
        ) -> ServiceSearchResults {
            // set up search targets
            let targetvecu81 = keywords1.clone();
            let target_string1 = String::from_utf8(targetvecu81).unwrap_or_default();
            let targetvecu82 = keywords2.clone();
            let target_string2 = String::from_utf8(targetvecu82).unwrap_or_default();
            let targetvecu83 = keywords3.clone();
            let target_string3 = String::from_utf8(targetvecu83).unwrap_or_default();

            // set up return structures
            let mut service_results = <Vec<Service>>::default();

            // iterate over all_sellers: StorageVec<AccountId> to find matching results
            if self.all_sellers.len() > 0 {
                for i in 0..self.all_sellers.len() {
                    let seller = self.all_sellers.get(i).unwrap();
                    // get the seller's services
                    let seller_services = self.account_seller_services.get(seller).unwrap_or_default();
                    for item in seller_services.hashvector.iter() {
                        // get the details
                        let details = self.service_details.get(item).unwrap_or_default();
                        // check to see if the keywords are there
                        let title_string = String::from_utf8(details.title.clone()).unwrap_or_default();
                        let seller_string = String::from_utf8(details.seller_name.clone()).unwrap_or_default();
                        let category_string = String::from_utf8(details.category.clone()).unwrap_or_default();
                        let description_string = String::from_utf8(details.description.clone()).unwrap_or_default();
                        let location_string = String::from_utf8(details.service_location.clone()).unwrap_or_default();

                        // if the target_string is in the details
                        if (title_string.contains(&target_string1) || seller_string.contains(&target_string1) ||
                        category_string.contains(&target_string1) || description_string.contains(&target_string1) ||
                        location_string.contains(&target_string1)) 
                        && (title_string.contains(&target_string2) || seller_string.contains(&target_string2) ||
                        category_string.contains(&target_string2) || description_string.contains(&target_string2) ||
                        location_string.contains(&target_string2)) 
                        && (title_string.contains(&target_string3) || seller_string.contains(&target_string3) ||
                        category_string.contains(&target_string3) || description_string.contains(&target_string3) ||
                        location_string.contains(&target_string3)) {
                            // add it to the results vector
                            service_results.push(details);
                        }
                        // continue iterating over this seller's services
                    }
                    // continue iterating over the other sellers
                }
            }

            // package the results
            let results = ServiceSearchResults {
                search: vec![keywords1, keywords2, keywords3],
                services: service_results
            };

            // return the results
            results
        }
        
        
        // 28 🟢 Search Stores by Keyword
        #[ink(message)]
        pub fn search_stores_by_keyword (&self,
            keywords1: Vec<u8>,
            keywords2: Vec<u8>,
            keywords3: Vec<u8>
        ) -> StoreSearchResults {
            // set up search targets
            let targetvecu81 = keywords1.clone();
            let target_string1 = String::from_utf8(targetvecu81).unwrap_or_default();
            let targetvecu82 = keywords2.clone();
            let target_string2 = String::from_utf8(targetvecu82).unwrap_or_default();
            let targetvecu83 = keywords3.clone();
            let target_string3 = String::from_utf8(targetvecu83).unwrap_or_default();

            // set up return structures
            let mut store_results = <Vec<SellerProfile>>::default();

            // iterate over all_sellers: StorageVec<AccountId> to find matching results
            if self.all_sellers.len() > 0 {
                for i in 0..self.all_sellers.len() {
                    let acct = self.all_sellers.get(i).unwrap();
                    // get the profile
                    let profile = self.account_profile_seller.get(acct).unwrap_or_default();
                    
                    // check to see if the keywords are there
                    let name_string = String::from_utf8(profile.seller_name.clone()).unwrap_or_default();
                    let description_string = String::from_utf8(profile.store_description.clone()).unwrap_or_default();
                    let location_string = String::from_utf8(profile.seller_location.clone()).unwrap_or_default();

                    // if the target_string is in the profile
                    if (name_string.contains(&target_string1) || description_string.contains(&target_string1) ||
                    location_string.contains(&target_string1))
                    && (name_string.contains(&target_string2) || description_string.contains(&target_string2) ||
                    location_string.contains(&target_string2))
                    && (name_string.contains(&target_string3) || description_string.contains(&target_string3) ||
                    location_string.contains(&target_string3)) {
                        // add it to the results vector
                        store_results.push(profile);
                    }
                }
                //continue iterating on the other sellers
            }

            // package the results
            let results = StoreSearchResults {
                search: vec![keywords1, keywords2, keywords3],
                stores: store_results
            };

            // return the results
            results
        }


        // 29 🟢 View My Orders
        #[ink(message)]
        pub fn view_my_orders (&self) -> ViewBuyerOrders {
            // set the caller
            let caller = Self::env().caller();
            // set up return structures
            let mut myorders = <Vec<Order>>::default();
            
            // get the buyer's completed Orders...
            // get the vector of order ids from storage
            let order_ids = self.account_buyer_orders.get(caller).unwrap_or_default();
            // get the details of each order and add them to the Vec<Order> myorders
            for id in order_ids.hashvector.iter() {
                // order_details: Mapping<Hash, Order>
                let orderdetails = self.order_details.get(id).unwrap_or_default();
                myorders.push(orderdetails);
            }

            // package the results
            let my_orders = ViewBuyerOrders {
                buyer: caller,
                carts: myorders,
            };

            // return the results
            my_orders
        }
        

        // 30 🟢 View My (Buyer) Account
        // Front end: if the product/service inventory is zero, note as unavailable
        #[ink(message)]
        pub fn view_my_buyer_account (&self) -> ViewBuyerAccount {
            // set the caller
            let caller = Self::env().caller();
            // get the buyer profile from account_profile_buyer: Mapping<AccountId, BuyerProfile>
            let buyerprofile = self.account_profile_buyer.get(caller).unwrap_or_default();

            // set up return structures
            let mut bookmarkedstores = <Vec<SellerProfile>>::default();
            let mut downloads = <Vec<Download>>::default();
            let mut myorders = <Vec<Order>>::default();

            // get bookmarked store accounts from account_store_bookmarks: Mapping<AccountId, AccountVector>
            let store_accounts = self.account_store_bookmarks.get(caller).unwrap_or_default();
            // for each seller account get the account_profile_seller: Mapping<AccountId, SellerProfile>
            for seller in store_accounts.accountvector.iter() {
                let profile = self.account_profile_seller.get(seller).unwrap_or_default();
                // add that profile to the bookmarkedstores vector of profiles
                bookmarkedstores.push(profile);
            }

            // get the digital product ids
            let digital_ids = self.account_owned_digital_items.get(caller).unwrap_or_default();
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

            // get the buyer's Orders...
            // get the vector of order ids from storage
            let order_ids = self.account_buyer_orders.get(caller).unwrap_or_default();
            // get the details of each order and add them to the Vec<Order> myorders
            for id in order_ids.hashvector.iter() {
                // order_details: Mapping<Hash, Order>
                let orderdetails = self.order_details.get(id).unwrap_or_default();
                myorders.push(orderdetails);
            }

            // package the results
            let my_account = ViewBuyerAccount {
                buyer: buyerprofile,
                bookmarked_stores: bookmarkedstores,
                digital_downloads: downloads,
                orders: myorders,
            };

            // return the results
            my_account
        }


        // 31 🟢 View My (Unpaid) Cart
        #[ink(message)]
        pub fn view_my_cart (&self) -> ViewUnpaidCart {
            // set the caller
            let caller = Self::env().caller();
            // get the callers current unpaid cart from account_current_cart: Mapping<AccountId, UnpaidCart>
            let current_cart = self.account_current_cart.get(caller).unwrap_or_default();

            // set up return structures
            let mut cartproducts = <Vec<UnpaidCartProduct>>::default();
            let mut cartservices = <Vec<UnpaidCartService>>::default();
            let mut carttotal_products: Balance = 0;
            let mut carttotal_services: Balance = 0;

            // each item in current_cart.cart_items looks like (Hash, u128) meaning (itemid, quantity)
            // for each item, determine product or service
            for (item, number) in current_cart.cart_items.iter() {
                if self.product_details.contains(item) {
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
                        carttotal_products = carttotal_products.saturating_add(productdetails.price.saturating_mul(*number));
                    }
                    
                }
                else {
                    if self.service_details.contains(item) {
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
                            carttotal_services = carttotal_services.saturating_add(servicedetails.price.saturating_mul(*number));
                        }

                    }
                }
            }

            // the cart total is the total of all items in the cart for which there
            // is sufficient inventory to fulfil the order if you order right now
            let carttotal: Balance = carttotal_products.saturating_add(carttotal_services);
            
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


        // 32 🟢 Go To Store
        #[ink(message)]
        pub fn go_to_store (&self,
            seller: AccountId
        ) -> ViewStore {
            // get the seller's profile from account_profile_seller: Mapping<AccountId, SellerProfile>
            let store_owner = self.account_profile_seller.get(seller).unwrap_or_default();
            // set up return structures
            let mut store_products = <Vec<PublicProduct>>::default();
            let mut store_services = <Vec<Service>>::default();

            // get the seller's products from account_seller_products: Mapping<AccountId, HashVector>
            let product_ids = self.account_seller_products.get(seller).unwrap_or_default();
            for id in product_ids.hashvector.iter() {
                // get the product details struct and add it to the store_products vector
                let details = self.product_details.get(id).unwrap_or_default();
                // make the public product structure
                let public_product = PublicProduct {
                    product_id: details.product_id,
                    digital: details.digital,
                    title: details.title,
                    price: details.price,
                    brand: details.brand,
                    category: details.category,
                    seller_account: details.seller_account,
                    seller_name: details.seller_name,
                    description: details.description,
                    review_average: details.review_average,
                    review_count: details.review_count,
                    inventory: details.inventory, 
                    photo_or_youtube_link1: details.photo_or_youtube_link1, 
                    photo_or_youtube_link2: details.photo_or_youtube_link2,
                    photo_or_youtube_link3: details.photo_or_youtube_link3,
                    more_info_link: details.more_info_link,
                    delivery_info: details.delivery_info,
                    product_location: details.product_location,
                    zeno_percent: details.zeno_percent,
                    zeno_buyers: details.zeno_buyers
                };
                store_products.push(public_product);
            }

            // get the seller's services from account_seller_services: Mapping<AccountId, HashVector> 
            let service_ids = self.account_seller_services.get(seller).unwrap_or_default();
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


        // 33 🟢 View My Seller Account - PROFILE, PRODUCTS and SERVICES
        #[ink(message)]
        pub fn view_my_seller_profile (&self) -> ViewSellerAccount {
            // set the caller
            let caller = Self::env().caller();
            let store_owner = self.account_profile_seller.get(caller).unwrap_or_default();
            let mut store_products = <Vec<Product>>::default();
            let mut store_services = <Vec<Service>>::default();

            // get the seller's products from account_seller_products: Mapping<AccountId, HashVector>
            let product_ids = self.account_seller_products.get(caller).unwrap_or_default();
            for id in product_ids.hashvector.iter() {
                // get the product details struct and add it to the store_products vector
                let productdetails = self.product_details.get(id).unwrap_or_default();
                store_products.push(productdetails);
            }

            // get the seller's services from account_seller_services: Mapping<AccountId, HashVector> 
            let service_ids = self.account_seller_services.get(caller).unwrap_or_default();
            for id in service_ids.hashvector.iter() {
                // get the service details struct and ad it to the store_service vector
                let servicedetails = self.service_details.get(id).unwrap_or_default();
                store_services.push(servicedetails);
            }

            // return the results
            let results = ViewSellerAccount {
                owner: store_owner,
                products: store_products,
                services: store_services
            };
            
            results
        }


        // 34 🟢 View My Seller Account - ORDERS - AWAITING
        #[ink(message)]
        pub fn view_my_seller_orders_awaiting (&self) -> Vec<Order> {
            // set the caller
            let caller = Self::env().caller();
            // set up return structures
            let mut store_orders = <Vec<Order>>::default();

            // get the seller's orders from account_seller_orders: Mapping<AccountId, HashVector>
            let order_ids0 = self.account_seller_orders_0awaiting.get(caller).unwrap_or_default();
            for id in order_ids0.hashvector.iter() {
                // get the order details and add it to the store_orders vector order_details: Mapping<Hash, Order>
                let orderdetails = self.order_details.get(id).unwrap_or_default();
                store_orders.push(orderdetails);
            }

            // return the results
            store_orders
        }

        // 35 🟢 View My Seller Account - ORDERS - SHIPPED
        #[ink(message)]
        pub fn view_my_seller_orders_shipped (&self) -> Vec<Order> {
            // set the caller
            let caller = Self::env().caller();
            // set up return structures
            let mut store_orders = <Vec<Order>>::default();

            let order_ids1 = self.account_seller_orders_1shipped.get(caller).unwrap_or_default();
            for id in order_ids1.hashvector.iter() {
                // get the order details and add it to the store_orders vector order_details: Mapping<Hash, Order>
                let orderdetails = self.order_details.get(id).unwrap_or_default();
                store_orders.push(orderdetails);
            }
            
            // return the results
            store_orders
        }

        // 36 🟢 View My Seller Account - ORDERS - DELIVERED
        #[ink(message)]
        pub fn view_my_seller_orders_delivered (&self) -> Vec<Order> {
            // set the caller
            let caller = Self::env().caller();
            // set up return structures
            let mut store_orders = <Vec<Order>>::default();

            // get the seller's orders from account_seller_orders: Mapping<AccountId, HashVector>
            let order_ids2 = self.account_seller_orders_2delivered.get(caller).unwrap_or_default();
            for id in order_ids2.hashvector.iter() {
                // get the order details and add it to the store_orders vector order_details: Mapping<Hash, Order>
                let orderdetails = self.order_details.get(id).unwrap_or_default();
                store_orders.push(orderdetails);
            }

            // return the results
            store_orders
        }

        // 37 🟢 View My Seller Account - ORDERS - RESOLVED
        #[ink(message)]
        pub fn view_my_seller_orders_resolved (&self) -> Vec<Order> {
            // set the caller
            let caller = Self::env().caller();
            // set up return structures
            let mut store_orders = <Vec<Order>>::default();

            // get the seller's orders from account_seller_orders: Mapping<AccountId, HashVector>
            let order_ids3 = self.account_seller_orders_3resolved.get(caller).unwrap_or_default();
            for id in order_ids3.hashvector.iter() {
                // get the order details and add it to the store_orders vector order_details: Mapping<Hash, Order>
                let orderdetails = self.order_details.get(id).unwrap_or_default();
                store_orders.push(orderdetails);
            }

            // return the results
            store_orders
        }

        // 38 🟢 View My Seller Account - ORDERS - PROBLEM
        #[ink(message)]
        pub fn view_my_seller_orders_problem (&self) -> Vec<Order> {
            // set the caller
            let caller = Self::env().caller();
            // set up return structures
            let mut store_orders = <Vec<Order>>::default();

            // get the seller's orders from account_seller_orders: Mapping<AccountId, HashVector>
            let order_ids4 = self.account_seller_orders_4problem.get(caller).unwrap_or_default();
            for id in order_ids4.hashvector.iter() {
                // get the order details and add it to the store_orders vector order_details: Mapping<Hash, Order>
                let orderdetails = self.order_details.get(id).unwrap_or_default();
                store_orders.push(orderdetails);
            }

            // return the results
            store_orders
        }

        // 39 🟢 View My Seller Account - ORDERS - REFUSED
        #[ink(message)]
        pub fn view_my_seller_orders_refused (&self) -> Vec<Order> {
            // set the caller
            let caller = Self::env().caller();
            // set up return structures
            let mut store_orders = <Vec<Order>>::default();

            // get the seller's orders from account_seller_orders: Mapping<AccountId, HashVector>
            let order_ids5 = self.account_seller_orders_5refused.get(caller).unwrap_or_default();
            for id in order_ids5.hashvector.iter() {
                // get the order details and add it to the store_orders vector order_details: Mapping<Hash, Order>
                let orderdetails = self.order_details.get(id).unwrap_or_default();
                store_orders.push(orderdetails);
            }

            // return the results
            store_orders
        }


        // 40 🟢 Get Market Statistics
        // get various stats about all sellers, all buyers, all products, all services, all orders for analysis
        #[ink(message)]
        pub fn get_market_statistics (&self) -> MarketStatistics {
            // set the caller
            let caller = Self::env().caller();
            let rightnow = self.env().block_timestamp();
            // set up return structures
            let seller_count = self.all_sellers.len().try_into().unwrap();
            let buyer_count = self.total_count_buyers;
            let product_count = self.total_count_products;
            let service_count = self.total_count_services;
            let order_count = self.total_count_orders;
            let mut orderdata = <Vec<OrderData>>::default();
            
            // for each order id, get the details from order_details: Mapping<Hash, Order>
            for id in self.all_orders.iter() {
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


        // >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
        // >>>>>>>>>>>>>>>>>>>>>>>>>> SECONDARY MESSAGES <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<
        // >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>


        // 41 🟢 Verify That An Account Has Set Up Buyer and/or Seller Info
        #[ink(message)]
        pub fn verify_account (&self, verify: AccountId) -> (u8, u8) {
            // set up return structures
            let mut buyer: u8 = 0;
            let mut seller: u8 = 0;
            // check the account_profile_buyer map
            if self.account_profile_buyer.contains(verify) {
                buyer = 1;
            }
            if self.account_profile_seller.contains(verify) {
                seller = 1;
            }
            // return the results
            let result: (u8, u8) = (buyer, seller);
            result
        }

        

        // END OF MESSAGE LIST

    }
    // END OF CONTRACT STORAGE

}
