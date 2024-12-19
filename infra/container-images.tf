
resource "aws_s3_bucket" "container-image-bucket" {
  bucket = "picturepro-container-images"
}

resource "aws_s3_bucket_ownership_controls" "container-image-bucket" {
  bucket = aws_s3_bucket.container-image-bucket.id
  rule {
    object_ownership = "BucketOwnerPreferred"
  }
}

resource "aws_s3_bucket_public_access_block" "container-image-bucket" {
  bucket = aws_s3_bucket.container-image-bucket.id

  block_public_acls       = false
  block_public_policy     = false
  ignore_public_acls      = false
  restrict_public_buckets = false
}

resource "aws_s3_bucket_acl" "container-image-bucket" {
  depends_on = [
    aws_s3_bucket_ownership_controls.container-image-bucket,
    aws_s3_bucket_public_access_block.container-image-bucket,
  ]

  bucket = aws_s3_bucket.container-image-bucket.id
  acl    = "public-read"
}

resource "nix_store_path" "tikv-image-store-path" {
  installable = "https://github.com/rambit-systems/rambit#images.x86_64-linux.tikv"
}

# resource "aws_s3_object" "tikv-image-object" {
#   bucket = aws_s3_bucket.container-image-bucket.bucket

#   key    = nix_store_path.tikv-image-store-path.output_path
#   source = nix_store_path.tikv-image-store-path.output_path
# }
