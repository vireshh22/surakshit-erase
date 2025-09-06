// import { useState, useEffect } from 'react';
// // import { invoke } from '@tauri-apps/api/tauri';
// import DriveTree from './components/DriveTree';
// import WipeProgress from './components/WipeProgress';
// import Certificate from './components/Certificate';
// import './App.css';

// function App() {
//   const [appState, setAppState] = useState('scanning'); // 'scanning' | 'ready' | 'wiping' | 'complete'
//   const [driveData, setDriveData] = useState(null);
//   const [error, setError] = useState(null);
//   const [selectedMethod, setSelectedMethod] = useState('Purge'); // 'Clear' | 'Purge' | 'Destroy'

//   // Fetch drive data from the Rust backend when the component mounts
//   // useEffect(() => {
//   //   invoke('scan_drives')
//   //     .then(data => {
//   //       setDriveData(data);
//   //       setAppState('ready');
//   //     })
//   //     .catch(err => {
//   //       setError(`Failed to scan drives: ${err}`);
//   //       console.error(err);
//   //     });
//   // }, []);

//   const handleStartWipe = () => {
//     if (driveData) {
//       setAppState('wiping');
//     }
//   };

//   const handleWipeComplete = () => {
//     setAppState('complete');
//   };

//   const renderContent = () => {
//     if (error) {
//       return <div className="error-box">Error: {error}</div>;
//     }

//     switch (appState) {
//       case 'scanning':
//         return <div>Scanning for drives...</div>;
      
//       case 'ready':
//         return (
//           <>
//             {driveData && <DriveTree data={driveData} />}
//             <div className="controls">
//               <h3>Select Wipe Method</h3>
//               <select 
//                 value={selectedMethod} 
//                 onChange={(e) => setSelectedMethod(e.target.value)}
//               >
//                 <option value="Clear">Clear</option>
//                 <option value="Purge">Purge</option>
//                 <option value="Destroy">Destroy</option>
//               </select>
//               <button onClick={handleStartWipe}>Start Wipe</button>
//             </div>
//           </>
//         );

//       case 'wiping':
//         return <WipeProgress method={selectedMethod} onComplete={handleWipeComplete} />;

//       case 'complete':
//         return <Certificate method={selectedMethod} fileCount={15} />; // Using a mock file count

//       default:
//         return <div>Invalid application state.</div>;
//     }
//   };

//   return (
//     <div className="container">
//       <h1>Surakshit Erase Prototype</h1>
//       <div className="content">
//         {renderContent()}
//       </div>
//     </div>
//   );
// }

// export default App;






import { useReducer, useEffect } from 'react';
import { reducer, initialState } from './reducer';
import DriveTree from './components/DriveTree';
import WipeProgress from './components/WipeProgress';
import Certificate from './components/Certificate';
import './App.css';

// --- DUMMY DATA --- (remains the same)
const dummyDriveData = {
  blockdevices: [{ name: "sda", size: "50G", model: "VBOX HARDDISK", children: [{ name: "sda1", size: "45G", model: null }] }],
  hpa: `/dev/sda:\n max sectors   = 41943040/500118192, HPA is enabled.`.trim(),
  dco: `/dev/sda:\n DCO Revision: 0x0002\n The DCO configuration is not frozen.`.trim(),
};

function App() {
  const [state, dispatch] = useReducer(reducer, initialState);
  const { status, driveData, error, selectedMethod } = state;

  // This effect simulates fetching data on component mount
  useEffect(() => {
    const fetchDrives = () => {
      console.log("Simulating drive scan...");
      setTimeout(() => {
        try {
          // On success, dispatch the success action with the data
          dispatch({ type: 'FETCH_SUCCESS', payload: dummyDriveData });
          console.log("Dummy data loaded.");
        } catch (err) {
          // On failure, dispatch the failure action with the error
          dispatch({ type: 'FETCH_FAILURE', payload: err.message });
        }
      }, 1000); // 1-second delay
    };
    
    fetchDrives();
  }, []); // Empty dependency array means this runs only once

  const renderContent = () => {
    switch (status) {
      case 'scanning':
        return <div>🔍 Scanning for drives...</div>;
      
      case 'error':
        return <div className="error-box">Error: {error}</div>;

      case 'ready':
        return (
          <>
            {driveData && <DriveTree data={driveData} />}
            <div className="controls">
              <h3>Select Wipe Method</h3>
              <select
                value={selectedMethod}
                onChange={(e) => dispatch({ type: 'SELECT_METHOD', payload: e.target.value })}
              >
                <option value="Clear">Clear</option>
                <option value="Purge">Purge</option>
                <option value="Destroy">Destroy</option>
              </select>
              <button onClick={() => dispatch({ type: 'START_WIPE' })}>Start Wipe</button>
            </div>
          </>
        );

      case 'wiping':
        return <WipeProgress method={selectedMethod} onComplete={() => dispatch({ type: 'WIPE_COMPLETE' })} />;

      case 'complete':
        return <Certificate method={selectedMethod} fileCount={15} />;

      default:
        return <div>Invalid application state.</div>;
    }
  };

  return (
    <div className="container">
      <h1>Surakshit Erase Prototype</h1>
      <div className="content">
        {renderContent()}
      </div>
    </div>
  );
}

export default App;