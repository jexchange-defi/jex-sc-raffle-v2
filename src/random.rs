multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait RandomModule {
    /// return a set of 'nb_ids' IDs between 1u32 and 'max' (both included)
    fn pick_random_ids(&self, max: u32, nb_ids: u16) -> ManagedVec<u32> {
        let mut rng = RandomnessSource::new();

        let mut picked_ids = ManagedVec::<Self::Api, u32>::new();

        if max > 0u32 {
            for _ in 0u16..nb_ids {
                let id = rng.next_u32_in_range(1u32, max + 1u32);

                picked_ids.push(id);
            }
        }

        picked_ids
    }
}
