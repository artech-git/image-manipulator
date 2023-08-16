"use client";

import React, { useState } from "react";

const ImageManipulation: React.FC = () => {
  const [selectedImage, setSelectedImage] = useState<string | null>(null);
  const [manipulatedImage, setManipulatedImage] = useState<string | null>(null);

  const handleImageChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (file) {
      setSelectedImage(URL.createObjectURL(file));
    }
  };

  const handleManipulateClick = () => {
    const canvas = document.createElement("canvas");
    const ctx = canvas.getContext("2d");
    if (ctx && selectedImage) {
      const img = new Image();
      img.src = selectedImage;
      img.onload = () => {
        canvas.width = img.width;
        canvas.height = img.height;
        ctx.drawImage(img, 0, 0, img.width, img.height);
        const imageData = ctx.getImageData(0, 0, img.width, img.height);

        for (let i = 0; i < imageData.data.length; i += 4) {
          imageData.data[i] = 255 - imageData.data[i];
          imageData.data[i + 1] = 255 - imageData.data[i + 1];
          imageData.data[i + 2] = 255 - imageData.data[i + 2];
        }

        ctx.putImageData(imageData, 0, 0);
        setManipulatedImage(canvas.toDataURL());
      };
    }
  };

  return (
    <div>
      <h1>Image Manipulation</h1>
      <input type="file" accept="image/*" onChange={handleImageChange} />
      <button onClick={handleManipulateClick}>Manipulate Image</button>
      {selectedImage && <img src={selectedImage} alt="Selected" />}
      {manipulatedImage && <img src={manipulatedImage} alt="Manipulated" />}
    </div>
  );
};

export default ImageManipulation;
