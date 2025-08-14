import { useEffect, useRef } from "react";

export default function useKeyTracking(keysToTrack) {
  const keys = useRef(Object.fromEntries(keysToTrack.map(k => [k, false])));

  useEffect(() => {
    const down = e => { if (keys.current[e.code] !== undefined) keys.current[e.code] = true; };
    const up = e => { if (keys.current[e.code] !== undefined) keys.current[e.code] = false; };

    window.addEventListener("keydown", down);
    window.addEventListener("keyup", up);
    return () => {
      window.removeEventListener("keydown", down);
      window.removeEventListener("keyup", up);
    };
  }, []);

  return keys;
}
