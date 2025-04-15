use storage::belt;

pub(crate) fn dvf_comp_to_belt_comp(
  comp: models::CompressionAlgorithm,
) -> belt::CompressionAlgorithm {
  match comp {
    models::CompressionAlgorithm::Zstd => belt::CompressionAlgorithm::Zstd,
  }
}
