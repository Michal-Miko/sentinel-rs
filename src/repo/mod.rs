mod local_repository;

pub use local_repository::LocalRepository;
pub use local_repository::LocalRepositoryError;

pub trait Repository<E> {
    fn fetch_file_contents(&self, path: &str) -> Result<String, E>;
}
