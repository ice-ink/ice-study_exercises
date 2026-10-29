import { useReducer } from 'react';

type Action =
  | { type: 'increment' }
  | { type: 'decrement' }
  | { type: 'reset' };

interface State {
  count: number;
}

const initialState: State = { count: 0 };

function reducer(state: State, action: Action): State {
  switch (action.type) {
    case 'increment':
      return { count: state.count + 1 };
    case 'decrement':
      return { count: state.count - 1 };
    case 'reset':
      return { count: 0 };
    default:
      return state;
  }
}

export function CounterReducer() {
  const [state, dispatch] = useReducer(reducer, initialState);

  return (
    <div>
      <span data-testid="count">{state.count}</span>
      <button type="button" onClick={() => dispatch({ type: 'increment' })}>
        Increment
      </button>
      <button type="button" onClick={() => dispatch({ type: 'decrement' })}>
        Decrement
      </button>
      <button type="button" onClick={() => dispatch({ type: 'reset' })}>
        Reset
      </button>
    </div>
  );
}
