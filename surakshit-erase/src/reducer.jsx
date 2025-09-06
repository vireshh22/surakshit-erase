// src/reducer.js

export const initialState = {
  status: 'scanning', // 'scanning', 'ready', 'wiping', 'complete'
  driveData: null,
  error: null,
  selectedMethod: 'Purge',
};

export function reducer(state, action) {
  switch (action.type) {
    case 'FETCH_SUCCESS':
      return {
        ...state,
        status: 'ready',
        driveData: action.payload,
      };
    case 'FETCH_FAILURE':
      return {
        ...state,
        status: 'error',
        error: action.payload,
      };
    case 'SELECT_METHOD':
      return {
        ...state,
        selectedMethod: action.payload,
      };
    case 'START_WIPE':
      // Can only start wipe if in 'ready' state
      if (state.status === 'ready') {
        return {
          ...state,
          status: 'wiping',
        };
      }
      return state; // No change if not in the right state
    case 'WIPE_COMPLETE':
      return {
        ...state,
        status: 'complete',
      };
    default:
      throw new Error(`Unhandled action type: ${action.type}`);
  }
}