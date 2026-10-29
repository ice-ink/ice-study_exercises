import { useState } from 'react';

export function Toggle() {
  const [visible, setVisible] = useState(false);

  return (
    <div>
      <button onClick={() => setVisible((v) => !v)}>Toggle</button>
      {visible ? <p>Now you see me</p> : null}
    </div>
  );
}
