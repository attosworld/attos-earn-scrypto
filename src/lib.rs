use scrypto::prelude::*;

#[blueprint]
mod attos_earn {
    struct AttosEarn {}

    impl AttosEarn {
        pub fn instantiate(owner_badge: ResourceAddress) -> Global<AttosEarn> {
            Self {}
                .instantiate()
                .prepare_to_globalize(OwnerRole::Fixed(rule!(require(owner_badge))))
                .enable_component_royalties(component_royalties! {
                    roles {
                        royalty_setter => OWNER; // #1
                        royalty_setter_updater => rule!(deny_all); // #2
                        royalty_locker => OWNER;
                        royalty_locker_updater => OWNER;
                        royalty_claimer => OWNER; // #3
                        royalty_claimer_updater => OWNER;
                    },
                    init {
                        charge_royalty => Xrd(100.into()), updatable;
                    }
                })
                .globalize()
        }

        pub fn charge_royalty(&self) {}
    }
}
