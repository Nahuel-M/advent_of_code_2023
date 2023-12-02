use derive_more::{AddAssign, Sub};

#[derive(Clone, Copy, Eq, PartialEq, Hash, Default, AddAssign, Sub, Debug)]
pub struct Vec2(pub i32, pub i32);

impl Vec2 {
    pub fn orthogonal_neighbors(&self) -> impl Iterator<Item = Vec2> {
        [
            Vec2(self.0 + 1, self.1),
            Vec2(self.0 - 1, self.1),
            Vec2(self.0, self.1 + 1),
            Vec2(self.0, self.1 - 1),
        ]
        .into_iter()
    }
}
