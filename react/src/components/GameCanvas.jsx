import { useEffect, useRef, useState } from "react";
import init, {
  play,
  move_left,
  move_right,
  jump,
  move_down,
  draw,
  apply_physics
} from "../../../rust/pkg/wararar.js";

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
  });

  useEffect(() => {
    init().then(() => setReady(true));
  }, []);

  useEffect(() => {
    if (!ready || !isPlaying) return;

    const canvas = canvasRef.current;
    if (!canvas) return;

    const resizeCanvas = () => {
      canvas.width = window.innerWidth - 40;
      canvas.height = window.innerHeight - 200;
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
      if (keys.current.KeyW || keys.current.ArrowUp) jump();
      if (keys.current.KeyS || keys.current.ArrowDown) move_down();
      if (keys.current.KeyA || keys.current.ArrowLeft) move_left();
      if (keys.current.KeyD || keys.current.ArrowRight) move_right();
      apply_physics();
      draw();
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
