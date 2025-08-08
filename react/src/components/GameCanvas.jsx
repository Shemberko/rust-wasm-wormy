import { useEffect, useRef, useState } from "react";
import init, {
  play,
  update,
  resize,
  init_player,
  shoot
} from "../../../rust/pkg/wararar.js";
import backgroundImage from '../../assets/tile_ground.png';

const GameCanvas = () => {
  const canvasRef = useRef(null);
  const [ready, setReady] = useState(false);
  const [isPlaying, setIsPlaying] = useState(false);
  const [imageData, set_image_data] = useState(null);

  const keys = useRef({
    ArrowUp: false,
    ArrowDown: false,
    ArrowLeft: false,
    ArrowRight: false,
    KeyW: false,
    KeyS: false,
    KeyA: false,
    KeyD: false,
    Space: false,
  });

  useEffect(() => {
    const setup = async () => {
      await init();
      await init_player();
      setReady(true);
    };

    setup();
  }, []);

  useEffect(() => {
    if (!ready || !isPlaying) return;

    const canvas = canvasRef.current;
    const ctx = canvas?.getContext("2d");

    if (!canvas || !ctx) return;

    const img = new Image();
    img.src = backgroundImage;

    img.onload = () => {
      const canvas = canvasRef.current;
      if (!canvas) return;
      const ctx = canvas.getContext("2d");
      if (!ctx) return;

      const scale = canvas.height / img.height;
      const scaledWidth = img.width * scale;
      ctx.clearRect(0, 0, canvas.width, canvas.height);

      // Load three images and combine them side by side
      const img1 = new Image();

      img1.src = backgroundImage;


      img1.onload = () => {
        if (!img1.complete) return;

        const cropPercent = 0.0;
        const cropY = img1.height * cropPercent;
        const croppedHeight = img1.height * (1 - cropPercent);
        const totalWidth = img1.width;

        // Створити тимчасовий канвас з обрізаною висотою
        const offCanvas = document.createElement("canvas");
        offCanvas.width = totalWidth;
        offCanvas.height = croppedHeight;
        const offCtx = offCanvas.getContext("2d");
        if (!offCtx) return;

        // Намалювати зображення, обрізавши верх
        offCtx.drawImage(
          img1,
          0, cropY,                 // джерело: зсув по Y
          totalWidth, croppedHeight, // джерело: розмір
          0, 0,                      // канвас: куди малювати
          totalWidth, croppedHeight  // канвас: розмір
        );

        // Отримати обрізаний ImageData
        const imageData = offCtx.getImageData(0, 0, totalWidth, croppedHeight);
        set_image_data(imageData);

        console.log("Cropped image:", imageData.width, imageData.height);

        // Clean up
        offCanvas.width = 0;
        offCanvas.height = 0;
      };

    };

    const resizeCanvas = () => {
      const canvas = canvasRef.current;
      if (!canvas) return;
      const ctx = canvas.getContext("2d");
      if (!ctx) return;

      canvas.width = window.innerWidth - 400;
      canvas.height = window.innerHeight - 200;

      console.log("Canvas resized to:", canvas.width, canvas.height);
      // ctx.imageSmoothingEnabled = false;

      // const img = new Image();
      // img.src = backgroundImage;
      // img.onload = () => {
      //   const scale = canvas.height / img.height;
      //   const scaledWidth = img.width * scale;

      //   ctx.clearRect(0, 0, canvas.width, canvas.height);
      //   ctx.drawImage(img, 0, 0, img.width, img.height, 0, 0, scaledWidth, canvas.height);

      //   const imageData = ctx.getImageData(0, 0, canvas.width, canvas.height);
      //   set_image_data(imageData); // mb this should be with game constructor

      //   resize(canvas.width, canvas.height);
      // };
    };

    resizeCanvas();
    // window.addEventListener("resize", resizeCanvas);

    const handleKeyDown = (e) => {
      if (keys.current.hasOwnProperty(e.code)) {
        keys.current[e.code] = true;
      }
    };

    const handleKeyUp = (e) => {
      if (keys.current.hasOwnProperty(e.code)) {
        keys.current[e.code] = false;
      }
    };

    window.addEventListener("keydown", handleKeyDown);
    window.addEventListener("keyup", handleKeyUp);

    const interval = setInterval(() => {
      const pressedKeysArray = Object.entries(keys.current)
        .filter(([_, pressed]) => pressed)
        .map(([key]) => key);

      update(pressedKeysArray);
    }, 16);

    return () => {
      clearInterval(interval);
      window.removeEventListener("resize", resizeCanvas);
      window.removeEventListener("keydown", handleKeyDown);
      window.removeEventListener("keyup", handleKeyUp);
    };
  }, [ready, isPlaying]);

  useEffect(() => {
    if (imageData && isPlaying) {
      try {
        play(imageData);
      } catch (e) {
        console.error("play() error:", e);
      }
    }
  }, [imageData, isPlaying]);

  const handlePlayClick = () => {
    setIsPlaying(true);
  };

  const handleClick = () => {
    if (ready && isPlaying) {
      shoot();
    }
  };

  return (
    <div style={{ textAlign: "center" }}>
      {!ready ? (
        <p>Loading WASM...</p>
      ) : !isPlaying ? (
        <button onClick={handlePlayClick}>Play</button>
      ) : (
        <canvas
          ref={canvasRef}
          id="mycanvas"
          onClick={handleClick}
          style={{ border: "1px solid black", margin: "20px" }}
        />
      )}
    </div>
  );
};

export default GameCanvas;
