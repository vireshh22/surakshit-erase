// reducer.js - Updated with navigation states

export const initialState = {
  status: 'drive_selection', // 'drive_selection', 'drive_contents', 'ready', 'wiping', 'complete'
  driveData: null,
  selectedDrive: null,
  driveFileSystem: null,
  error: null,
  selectedMethod: 'Purge',
};

export function reducer(state, action) {
  switch (action.type) {
    case 'SET_SELECTED_DRIVE':
      return {
        ...state,
        selectedDrive: action.payload,
        status: 'drive_contents',
        driveFileSystem: null,
        error: null,
      };

    case 'FETCH_DRIVE_CONTENTS_SUCCESS':
      return {
        ...state,
        driveFileSystem: action.payload,
        status: 'drive_contents',
        error: null,
      };

    case 'FETCH_DRIVE_CONTENTS_FAILURE':
      return {
        ...state,
        error: action.payload,
        status: 'drive_contents',
      };

    case 'NAVIGATE_BACK_TO_DRIVE_SELECTION':
      return {
        ...state,
        status: 'drive_selection',
        selectedDrive: null,
        driveFileSystem: null,
        error: null,
      };

    case 'PROCEED_TO_METHOD_SELECTION':
      return {
        ...state,
        status: 'ready',
      };

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