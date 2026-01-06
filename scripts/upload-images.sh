#!/bin/bash
# Upload images to Cloudflare R2 bucket
# Run this from the riverview-complete directory

BUCKET_NAME="riverview-assets"

echo "Uploading images to R2 bucket: $BUCKET_NAME"

# Upload each image
for file in static/images/*; do
    filename=$(basename "$file")
    echo "Uploading images/$filename..."
    wrangler r2 object put "$BUCKET_NAME/images/$filename" --file="$file" --content-type="$(file -b --mime-type "$file")"
done

# Upload favicon (use logo.png as favicon)
echo "Uploading favicon..."
wrangler r2 object put "$BUCKET_NAME/favicon.ico" --file="static/images/logo.png" --content-type="image/png"

echo "Done! All images uploaded to R2."
echo ""
echo "Verify with: wrangler r2 object list $BUCKET_NAME"
