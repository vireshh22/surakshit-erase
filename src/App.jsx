// // import { useState, useEffect } from 'react';
// // // import { invoke } from '@tauri-apps/api/tauri';
// // import DriveTree from './components/DriveTree';
// // import WipeProgress from './components/WipeProgress';
// // import Certificate from './components/Certificate';
// // import './App.css';

// // function App() {
// //   const [appState, setAppState] = useState('scanning'); // 'scanning' | 'ready' | 'wiping' | 'complete'
// //   const [driveData, setDriveData] = useState(null);
// //   const [error, setError] = useState(null);
// //   const [selectedMethod, setSelectedMethod] = useState('Purge'); // 'Clear' | 'Purge' | 'Destroy'

// //   // Fetch drive data from the Rust backend when the component mounts
// //   // useEffect(() => {
// //   //   invoke('scan_drives')
// //   //     .then(data => {
// //   //       setDriveData(data);
// //   //       setAppState('ready');
// //   //     })
// //   //     .catch(err => {
// //   //       setError(`Failed to scan drives: ${err}`);
// //   //       console.error(err);
// //   //     });
// //   // }, []);

// //   const handleStartWipe = () => {
// //     if (driveData) {
// //       setAppState('wiping');
// //     }
// //   };

// //   const handleWipeComplete = () => {
// //     setAppState('complete');
// //   };

// //   const renderContent = () => {
// //     if (error) {
// //       return <div className="error-box">Error: {error}</div>;
// //     }

// //     switch (appState) {
// //       case 'scanning':
// //         return <div>Scanning for drives...</div>;

// //       case 'ready':
// //         return (
// //           <>
// //             {driveData && <DriveTree data={driveData} />}
// //             <div className="controls">
// //               <h3>Select Wipe Method</h3>
// //               <select
// //                 value={selectedMethod}
// //                 onChange={(e) => setSelectedMethod(e.target.value)}
// //               >
// //                 <option value="Clear">Clear</option>
// //                 <option value="Purge">Purge</option>
// //                 <option value="Destroy">Destroy</option>
// //               </select>
// //               <button onClick={handleStartWipe}>Start Wipe</button>
// //             </div>
// //           </>
// //         );

// //       case 'wiping':
// //         return <WipeProgress method={selectedMethod} onComplete={handleWipeComplete} />;

// //       case 'complete':
// //         return <Certificate method={selectedMethod} fileCount={15} />; // Using a mock file count

// //       default:
// //         return <div>Invalid application state.</div>;
// //     }
// //   };

// //   return (
// //     <div className="container">
// //       <h1>Surakshit Erase Prototype</h1>
// //       <div className="content">
// //         {renderContent()}
// //       </div>
// //     </div>
// //   );
// // }

// // export default App;

// import { useReducer, useEffect } from "react";
// import { reducer, initialState } from "./reducer";
// import DriveTree from "./components/DriveTree";
// import WipeProgress from "./components/WipeProgress";
// import Certificate from "./components/Certificate";
// import "./App.css";

// // --- DUMMY DATA --- (remains the same)
// const dummyDriveData = {
//     blockdevices: [
//         {
//             name: "sda",
//             size: "50G",
//             model: "VBOX HARDDISK",
//             children: [{ name: "sda1", size: "45G", model: null }],
//         },
//     ],
//     hpa: `/dev/sda:\n max sectors   = 41943040/500118192, HPA is enabled.`.trim(),
//     dco: `/dev/sda:\n DCO Revision: 0x0002\n The DCO configuration is not frozen.`.trim(),
// };

// function App() {
//     const [state, dispatch] = useReducer(reducer, initialState);
//     const { status, driveData, error, selectedMethod } = state;

//     // This effect simulates fetching data on component mount
//     useEffect(() => {
//         const fetchDrives = () => {
//             console.log("Simulating drive scan...");
//             setTimeout(() => {
//                 try {
//                     // On success, dispatch the success action with the data
//                     dispatch({
//                         type: "FETCH_SUCCESS",
//                         payload: dummyDriveData,
//                     });
//                     console.log("Dummy data loaded.");
//                 } catch (err) {
//                     // On failure, dispatch the failure action with the error
//                     dispatch({ type: "FETCH_FAILURE", payload: err.message });
//                 }
//             }, 1000); // 1-second delay
//         };

//         fetchDrives();
//     }, []); // Empty dependency array means this runs only once

//     const renderContent = () => {
//         switch (status) {
//             case "scanning":
//                 return <div className="scanning-state">🔍 Scanning for drives...</div>;

//             case "error":
//                 return <div className="error-box">Error: {error}</div>;

//             case "ready":
//                 return (
//                     <div className="ready-state">
//                         {driveData && <DriveTree data={driveData} />}
//                         <div className="controls">
//                             <h3>Select Wipe Method</h3>

//                             <div className="method-selection">
//                                 {["Clear", "Purge", "Destroy"].map((method) => (
//                                     <div key={method} className="method-option">
//                                         <div
//                                             className={`method-card ${
//                                                 selectedMethod === method
//                                                     ? "selected"
//                                                     : ""
//                                             }`}
//                                             onClick={() =>
//                                                 dispatch({
//                                                     type: "SELECT_METHOD",
//                                                     payload: method,
//                                                 })
//                                             }
//                                         >
//                                             <div
//                                                 className={`security-level ${
//                                                     method.toLowerCase() ===
//                                                     "clear"
//                                                         ? "low"
//                                                         : method.toLowerCase() ===
//                                                           "purge"
//                                                         ? "medium"
//                                                         : "high"
//                                                 }`}
//                                             >
//                                                 {method === "Clear"
//                                                     ? "Low Security"
//                                                     : method === "Purge"
//                                                     ? "Medium Security"
//                                                     : "High Security"}
//                                             </div>
//                                             <div className="method-header">
//                                                 <div
//                                                     className={`method-icon ${method.toLowerCase()}`}
//                                                 >
//                                                     {method === "Clear"
//                                                         ? "🧹"
//                                                         : method === "Purge"
//                                                         ? "🔥"
//                                                         : "💥"}
//                                                 </div>
//                                                 <div className="method-title">
//                                                     {method}
//                                                 </div>
//                                             </div>
//                                             <div className="method-description">
//                                                 {method === "Clear" &&
//                                                     "Basic overwrite with zeros. Fast but minimal security. Suitable for non-sensitive data."}
//                                                 {method === "Purge" &&
//                                                     "Multiple-pass overwrite with random data. Balanced security and speed for sensitive information."}
//                                                 {method === "Destroy" &&
//                                                     "Military-grade multi-pass overwrite. Maximum security for highly classified data. Takes longer."}
//                                             </div>
//                                         </div>
//                                     </div>
//                                 ))}
//                             </div>

//                             <button
//                                 className="start-button"
//                                 onClick={() => dispatch({ type: "START_WIPE" })}
//                             >
//                                 🚀 Start Security Wipe
//                             </button>
//                         </div>
//                     </div>
//                 );

//             case "wiping":
//                 return (
//                     <div className="wiping-state">
//                         <WipeProgress
//                             method={selectedMethod}
//                             onComplete={() => dispatch({ type: "WIPE_COMPLETE" })}
//                         />
//                     </div>
//                 );

//             case "complete":
//                 return <Certificate method={selectedMethod} fileCount={15} />;

//             default:
//                 return <div>Invalid application state.</div>;
//         }
//     };

//     return (
//         <div className="container">
//             <h1>Surakshit Erase Prototype</h1>
//             <div className="content">{renderContent()}</div>
//         </div>
//     );
// }

// export default App;


// App.jsx - Updated with drive selection flow
// import { useReducer } from "react";
// import { reducer, initialState } from "./reducer";
// import DriveSelector from "./components/DriveSelector";
// import DriveTree from "./components/DriveTree";
// import NavigationBreadcrumb from "./components/NavigationBreadcrumb";
// import WipeProgress from "./components/WipeProgress";
// import Certificate from "./components/Certificate";
// import Controls from "./components/Controls";
// import "./App.css";

// function App() {
//     const [state, dispatch] = useReducer(reducer, initialState);
//     const { 
//         status, 
//         selectedDrive, 
//         driveFileSystem, 
//         error, 
//         selectedMethod 
//     } = state;

//     const handleDriveSelect = (drive) => {
//         dispatch({
//             type: 'SET_SELECTED_DRIVE',
//             payload: drive
//         });
//     };

//     const handleNavigateBack = () => {
//         if (status === 'drive_contents') {
//             dispatch({ type: 'NAVIGATE_BACK_TO_DRIVE_SELECTION' });
//         } else if (status === 'ready') {
//             dispatch({ type: 'NAVIGATE_BACK_TO_DRIVE_SELECTION' });
//         }
//     };

//     const handleNavigateHome = () => {
//         dispatch({ type: 'NAVIGATE_BACK_TO_DRIVE_SELECTION' });
//     };

//     const handleProceedToMethodSelection = () => {
//         dispatch({ type: 'PROCEED_TO_METHOD_SELECTION' });
//     };

//     const renderContent = () => {
//         switch (status) {
//             case 'drive_selection':
//                 return (
//                     <DriveSelector 
//                         onDriveSelect={handleDriveSelect}
//                         selectedDrive={selectedDrive}
//                     />
//                 );

//             case 'drive_contents':
//                 return (
//                     <DriveTree 
//                         selectedDrive={selectedDrive}
//                         driveFileSystem={driveFileSystem}
//                         onProceedToMethodSelection={handleProceedToMethodSelection}
//                         dispatch={dispatch}
//                     />
//                 );

//             case 'error':
//                 return <div className="error-box">Error: {error}</div>;

//             case 'ready':
//                 return (
//                     <div className="ready-state">
//                         <div className="selected-drive-summary">
//                             <h3>Selected Drive: {selectedDrive?.name || 'Unknown'}</h3>
//                             <p>Path: <code>{selectedDrive?.path || 'Unknown'}</code></p>
//                             {selectedDrive?.size && <p>Size: {selectedDrive.size}</p>}
//                             <div className="warning-notice">
//                                 ⚠️ All data on this drive will be permanently erased!
//                             </div>
//                         </div>

//                         <Controls />
//                     </div>
//                 );

//             case 'wiping':
//                 return (
//                     <div className="wiping-state">
//                         <WipeProgress
//                             method={selectedMethod}
//                             selectedDrive={selectedDrive}
//                             onComplete={() => dispatch({ type: "WIPE_COMPLETE" })}
//                         />
//                     </div>
//                 );

//             case 'complete':
//                 return (
//                     <Certificate 
//                         method={selectedMethod} 
//                         selectedDrive={selectedDrive}
//                         fileCount={15} 
//                     />
//                 );

//             default:
//                 return <div>Invalid application state.</div>;
//         }
//     };

//     return (
//         <div className="container">
//             <h1>Surakshit Erase Prototype</h1>
            

//             <div className="content">
//                 {status !== 'drive_selection' && (
//                     <NavigationBreadcrumb
//                         currentView={status}
//                         selectedDrive={selectedDrive}
//                         onNavigateBack={handleNavigateBack}
//                         onNavigateHome={handleNavigateHome}
//                     />
//                 )}
//                 {renderContent()}
//             </div>
//         </div>
//     );
// }

// export default App;


// App.jsx - Updated with better drive selection layout
import { useReducer } from "react";
import { reducer, initialState } from "./reducer";
import DriveSelector from "./components/DriveSelector";
import DriveTree from "./components/DriveTree";
import NavigationBreadcrumb from "./components/NavigationBreadcrumb";
import WipeProgress from "./components/WipeProgress";
import Certificate from "./components/Certificate";
import Controls from "./components/Controls";
import { AlertTriangle, HardDrive, Database } from "lucide-react";
import "./App.css";

function App() {
    const [state, dispatch] = useReducer(reducer, initialState);
    const { 
        status, 
        selectedDrive, 
        driveFileSystem, 
        error, 
        selectedMethod 
    } = state;

    const handleDriveSelect = (drive) => {
        dispatch({
            type: 'SET_SELECTED_DRIVE',
            payload: drive
        });
    };

    const handleNavigateBack = () => {
        if (status === 'drive_contents') {
            dispatch({ type: 'NAVIGATE_BACK_TO_DRIVE_SELECTION' });
        } else if (status === 'ready') {
            dispatch({ type: 'NAVIGATE_BACK_TO_DRIVE_SELECTION' });
        }
    };

    const handleNavigateHome = () => {
        dispatch({ type: 'NAVIGATE_BACK_TO_DRIVE_SELECTION' });
    };

    const handleProceedToMethodSelection = () => {
        dispatch({ type: 'PROCEED_TO_METHOD_SELECTION' });
    };

    const renderContent = () => {
        switch (status) {
            case 'drive_selection':
                return (
                    <DriveSelector 
                        onDriveSelect={handleDriveSelect}
                        selectedDrive={selectedDrive}
                    />
                );

            case 'drive_contents':
                return (
                    <DriveTree 
                        selectedDrive={selectedDrive}
                        driveFileSystem={driveFileSystem}
                        onProceedToMethodSelection={handleProceedToMethodSelection}
                        dispatch={dispatch}
                    />
                );

            case 'error':
                return <div className="error-box">Error: {error}</div>;

            case 'ready':
                return (
                    <div className="ready-state">
                        {/* Drive Selection Header */}
                        <div className="drive-selection-header">
                            <div className="selected-drive-summary">
                                <h3>
                                    <HardDrive size={24} style={{display: 'inline', marginRight: '8px', verticalAlign: 'middle'}} />
                                    Selected Drive: {selectedDrive?.name || 'Unknown'}
                                </h3>
                                
                                <div className="drive-details">
                                    <div className="drive-detail-item">
                                        <span className="drive-detail-label">Device Path</span>
                                        <code className="drive-detail-value">
                                            {selectedDrive?.path || 'Unknown'}
                                        </code>
                                    </div>
                                    {selectedDrive?.size && (
                                        <div className="drive-detail-item">
                                            <span className="drive-detail-label">Total Size</span>
                                            <span className="drive-detail-value">
                                                <Database size={14} style={{display: 'inline', marginRight: '4px', verticalAlign: 'middle'}} />
                                                {selectedDrive.size}
                                            </span>
                                        </div>
                                    )}
                                    {selectedDrive?.model && (
                                        <div className="drive-detail-item">
                                            <span className="drive-detail-label">Drive Model</span>
                                            <span className="drive-detail-value">{selectedDrive.model}</span>
                                        </div>
                                    )}
                                </div>

                                <div className="warning-notice">
                                    <AlertTriangle size={20} />
                                    All data on this drive will be permanently erased!
                                </div>
                            </div>
                        </div>

                        {/* Method Selection */}
                        <Controls 
                            selectedMethod={selectedMethod}
                            dispatch={dispatch}
                        />
                    </div>
                );

            case 'wiping':
                return (
                    <div className="wiping-state">
                        <WipeProgress
                            method={selectedMethod}
                            selectedDrive={selectedDrive}
                            onComplete={() => dispatch({ type: "WIPE_COMPLETE" })}
                        />
                    </div>
                );

            case 'complete':
                return (
                    <Certificate 
                        method={selectedMethod} 
                        selectedDrive={selectedDrive}
                        fileCount={15} 
                    />
                );

            default:
                return <div>Invalid application state.</div>;
        }
    };

    return (
        <div className="container">
            <h1>Surakshit Erase Prototype</h1>
            
            {status !== 'drive_selection' && (
                <NavigationBreadcrumb
                    currentView={status}
                    selectedDrive={selectedDrive}
                    onNavigateBack={handleNavigateBack}
                    onNavigateHome={handleNavigateHome}
                />
            )}

            <div className="content">
                {renderContent()}
            </div>
        </div>
    );
}

export default App;