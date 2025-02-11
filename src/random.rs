multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait RandomModule {
    fn pick_random_ids(&self, max_id: u32, nb_ids: u16) -> ManagedVec<u32> {
        let mut rng = RandomnessSource::new();

        let mut picked_ids = ManagedVec::<Self::Api, u32>::new();

        for _ in 0u16..nb_ids {
            let id = rng.next_u32_in_range(1u32, max_id);

            picked_ids.push(id);
        }

        picked_ids
    }
}
