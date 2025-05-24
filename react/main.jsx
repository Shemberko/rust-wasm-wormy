import React, { useEffect, useState } from "react";
import ReactDOM from "react-dom/client";

// Імпортуємо init (default) та функцію play з wasm-пакету
import init, { play } from "../rust/pkg/wararar.js";

function App() {
  const [ready, setReady] = useState(false);

  useEffect(() => {
    // Ініціалізуємо wasm, після чого ставимо ready в true
    init().then(() => {
      setReady(true);
    });
  }, []);

  const handleClick = () => {
    try {
      play();
    } catch (e) {
      console.error("Error during wasm play():", e);
    }
  };

  if (!ready) {
    return <div>Loading WASM...</div>;
  }

  return (
    <div>
      <h1>WASM Loaded!</h1>
      <button onClick={handleClick}>Play</button>
    </div>
  );
}

const root = ReactDOM.createRoot(document.getElementById("root"));
root.render(<App />);
