use crate::rule::{EmptyDefinition, NonEmptyIntoIter, NonEmptyRule};
use crate::Refined;

use std::iter::Map;
use std::ops::Add;

impl<I: ExactSizeIterator + EmptyDefinition> Refined<NonEmptyRule<I>> {
    pub fn map<B, F>(self, f: F) -> Refined<NonEmptyRule<Map<I, F>>>
    where
        Self: Sized,
        F: FnMut(I::Item) -> B,
    {
        let map_into_iter = self.into_value().map(f);
        Refined::new(map_into_iter)
            .ok()
            .expect("This error is always unreachable")
    }

    pub fn collect<B: FromIterator<I::Item> + EmptyDefinition>(self) -> Refined<NonEmptyRule<B>>
    where
        Self: Sized,
    {
        Refined::new(FromIterator::from_iter(self.into_value()))
            .ok()
            .expect("This error is always unreachable")
    }
}

pub type NonEmptyVec<T> = Refined<NonEmptyVecRule<T>>;

impl<T> NonEmptyVec<T>
where
    T: EmptyDefinition,
{
    #[allow(clippy::should_implement_trait)]
    pub fn into_iter(self) -> NonEmptyIntoIter<T> {
        Refined::new(self.into_value().into_iter())
            .ok()
            .expect("This error is always unreachable")
    }
}

impl<T> Add for NonEmptyVec<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = self.into_value();
        result.append(&mut rhs.into_value());
        Refined::new(result)
            .ok()
            .expect("This error is always unreachable")
    }
}

pub type NonEmptyVecRule<T> = NonEmptyRule<Vec<T>>;

#[cfg(test)]
mod test {
    use crate::rule::non_empty::NonEmptyVecRule;
    use crate::rule::{NonEmptyVec, Rule};

    #[test]
    fn test_non_empty_vec() {
        assert!(NonEmptyVecRule::validate(vec![1, 2, 3]).is_ok());
        assert!(NonEmptyVecRule::<u8>::validate(vec![]).is_err());
    }

    #[test]
    fn test_add_vec() -> anyhow::Result<()> {
        let ne_vec_1 = NonEmptyVec::new(vec![1, 2, 3])?;
        let ne_vec_2 = NonEmptyVec::new(vec![4, 5, 6])?;
        let ne_vec = ne_vec_1 + ne_vec_2;
        assert_eq!(ne_vec.into_value(), vec![1, 2, 3, 4, 5, 6]);
        Ok(())
    }

    #[test]
    fn test_into_iter() -> anyhow::Result<()> {
        let ne_vec = NonEmptyVec::new(vec![1, 2, 3])?;
        let ne_vec = ne_vec
            .into_iter()
            .map(|n| n * 2)
            .map(|n| n * 3)
            .collect::<Vec<_>>();
        assert_eq!(ne_vec.into_value(), vec![6, 12, 18]);
        Ok(())
    }
}
