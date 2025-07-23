import { useEffect, useRef, useState } from "react";
import init, {
  play,
  update,
  resize,
  init_player,
  set_image_data
} from "../../../rust/pkg/wararar.js";
import backgroundImage from '../../assets/tile_ground.png';

const GameCanvas = () => {
  const canvasRef = useRef(null);
  const [ready, setReady] = useState(false);
  const [isPlaying, setIsPlaying] = useState(false);

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
      ctx.drawImage(img, 0, 0, img.width, img.height, 0, 0, scaledWidth, canvas.height);

      const imageData = ctx.getImageData(0, 0, canvas.width, canvas.height);
      set_image_data(imageData);
    };


    const resizeCanvas = () => {
      const canvas = canvasRef.current;
      if (!canvas) return;
      const ctx = canvas.getContext("2d");
      if (!ctx) return;

      canvas.width = window.innerWidth - 40;
      canvas.height = window.innerHeight - 200;
      ctx.imageSmoothingEnabled = false;

      const img = new Image();
      img.src = backgroundImage;
      img.onload = () => {
        const scale = canvas.height / img.height;
        const scaledWidth = img.width * scale;

        ctx.clearRect(0, 0, canvas.width, canvas.height);
        ctx.drawImage(img, 0, 0, img.width, img.height, 0, 0, scaledWidth, canvas.height);

        const imageData = ctx.getImageData(0, 0, canvas.width, canvas.height);
        set_image_data(imageData);

        // Тільки після цього викликаємо resize в Rust, щоб оновити розміри
        resize(canvas.width, canvas.height);
      };
    };

    resizeCanvas();
    window.addEventListener("resize", resizeCanvas);

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

    try {
      play();
    } catch (e) {
      console.error("play() error:", e);
    }

    return () => {
      clearInterval(interval);
      window.removeEventListener("resize", resizeCanvas);
      window.removeEventListener("keydown", handleKeyDown);
      window.removeEventListener("keyup", handleKeyUp);
    };
  }, [ready, isPlaying]);

  const handlePlayClick = () => {
    setIsPlaying(true);
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
          style={{ border: "1px solid black", margin: "20px" }}
        />
      )}
    </div>
  );
};

export default GameCanvas;
