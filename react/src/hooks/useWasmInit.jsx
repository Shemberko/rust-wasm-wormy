import { useEffect, useState } from "react";
import init, { init_player } from "../../../rust/pkg/wararar.js";

export default function useWasmInit() {
  const [ready, setReady] = useState(false);

  useEffect(() => {
    (async () => {
      await init();
      await init_player();
      setReady(true);
    })();
  }, []);

  return ready;
}
