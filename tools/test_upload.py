#!/usr/bin/env python3
"""Test script to upload a test JPEG file to the server"""

import requests
import io
from PIL import Image
import sys


def create_test_jpeg():
    """Create a simple test JPEG image"""
    # Create a simple 100x100 red image
    img = Image.new("RGB", (100, 100), color="red")

    # Save to bytes
    img_bytes = io.BytesIO()
    img.save(img_bytes, format="JPEG", quality=85)
    img_bytes.seek(0)

    return img_bytes.read()


def upload_image(image_data, filename="test.jpg"):
    """Upload image to the server"""
    url = "http://127.0.0.1:3000/api/upload"

    files = {"image": (filename, image_data, "image/jpeg")}

    print(f"Uploading {len(image_data)} bytes...")
    print(f"First 16 bytes: {image_data[:16].hex()}")

    try:
        response = requests.post(url, files=files)
        print(f"Status: {response.status_code}")
        print(f"Response: {response.text}")

        if response.status_code == 200:
            print("✅ Upload successful!")
        else:
            print("❌ Upload failed!")

    except requests.exceptions.ConnectionError:
        print("❌ Could not connect to server. Is it running?")
        print("Start the server with: cargo run")
    except Exception as e:
        print(f"❌ Error: {e}")


if __name__ == "__main__":
    print("Creating test JPEG image...")
    image_data = create_test_jpeg()

    print(f"Image size: {len(image_data)} bytes")
    print(f"First bytes (hex): {image_data[:20].hex()}")

    print("\nUploading to server...")
    upload_image(image_data)
