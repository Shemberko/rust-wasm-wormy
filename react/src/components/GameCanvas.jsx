import { useEffect, useRef, useState } from "react";
import init, {
  play,
  update,
  resize,
  init_player,
  shoot
} from "../../../rust/pkg/wararar.js";
import backgroundImage from '../../assets/tile_ground.png';
import useWasmInit from "../hooks/useWasmInit";
import useKeyTracking from "../hooks/useKeyTracking";
import loadAndCropImage from "../utils/loadAndCropImage";

const GameCanvas = () => {
  const ready = useWasmInit();

  const canvasRef = useRef(null);
  const [isPlaying, setIsPlaying] = useState(false);
  const [imageData, setImageData] = useState(null);

  const keys = useKeyTracking([
    "ArrowUp", "ArrowDown", "ArrowLeft", "ArrowRight",
    "KeyW", "KeyS", "KeyA", "KeyD", "Space"
  ]);

  useEffect(() => {
    if (!ready || !isPlaying) return;

    const resizeCanvas = () => {
      const canvas = canvasRef.current;
      if (!canvas) return;

      canvas.width = window.innerWidth - 400;
      canvas.height = window.innerHeight - 200;
    };

    resizeCanvas();

    loadAndCropImage(backgroundImage).then(setImageData);
    // window.addEventListener("resize", resizeCanvas);

    const interval = setInterval(() => {
      const pressedKeysArray = Object.entries(keys.current)
        .filter(([_, pressed]) => pressed)
        .map(([key]) => key);

      update(pressedKeysArray);
    }, 16);

    return () => {
      clearInterval(interval);
      window.removeEventListener("resize", resizeCanvas);
    };
  }, [ready, isPlaying]);

  useEffect(() => {
    if (imageData && isPlaying) {
      play(imageData);
    }
  }, [imageData, isPlaying]);

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
        <button onClick={() => setIsPlaying(true)}>Play</button>
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
