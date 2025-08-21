export default async function loadAndCropImage(src, cropPercent = 0) {
  const img = await new Promise((resolve, reject) => {
    const image = new Image();
    image.src = src;
    image.onload = () => resolve(image);
    image.onerror = reject;
  });

  const cropY = img.height * cropPercent;
  const croppedHeight = img.height * (1 - cropPercent);

  const offCanvas = document.createElement("canvas");
  offCanvas.width = img.width;
  offCanvas.height = croppedHeight;

  const offCtx = offCanvas.getContext("2d");
  offCtx.drawImage(img, 0, cropY, img.width, croppedHeight, 0, 0, img.width, croppedHeight);

  return offCtx.getImageData(0, 0, img.width, croppedHeight);
}
