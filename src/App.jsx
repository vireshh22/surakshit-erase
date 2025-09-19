// import { useReducer } from "react";
// import { reducer, initialState } from "./reducer";
// import DriveSelector from "./components/DriveSelector";
// import DriveTree from "./components/DriveTree";
// import NavigationBreadcrumb from "./components/NavigationBreadcrumb";
// import WipeProgress from "./components/WipeProgress";
// import Certificate from "./components/Certificate";
// import Controls from "./components/Controls";
// import { AlertTriangle, HardDrive, Database } from "lucide-react";
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
//                         {/* Drive Selection Header */}
//                         <div className="drive-selection-header">
//                             <div className="selected-drive-summary">
//                                 <h3>
//                                     <HardDrive size={24} style={{display: 'inline', marginRight: '8px', verticalAlign: 'middle'}} />
//                                     Selected Drive: {selectedDrive?.name || 'Unknown'}
//                                 </h3>
                                
//                                 <div className="drive-details">
//                                     <div className="drive-detail-item">
//                                         <span className="drive-detail-label">Device Path</span>
//                                         <code className="drive-detail-value">
//                                             {selectedDrive?.path || 'Unknown'}
//                                         </code>
//                                     </div>
//                                     {selectedDrive?.size && (
//                                         <div className="drive-detail-item">
//                                             <span className="drive-detail-label">Total Size</span>
//                                             <span className="drive-detail-value">
//                                                 <Database size={14} style={{display: 'inline', marginRight: '4px', verticalAlign: 'middle'}} />
//                                                 {selectedDrive.size}
//                                             </span>
//                                         </div>
//                                     )}
//                                     {selectedDrive?.model && (
//                                         <div className="drive-detail-item">
//                                             <span className="drive-detail-label">Drive Model</span>
//                                             <span className="drive-detail-value">{selectedDrive.model}</span>
//                                         </div>
//                                     )}
//                                 </div>

//                                 <div className="warning-notice">
//                                     <AlertTriangle size={20} />
//                                     All data on this drive will be permanently erased!
//                                 </div>
//                             </div>
//                         </div>

//                         {/* Method Selection */}
//                         <Controls 
//                             selectedMethod={selectedMethod}
//                             dispatch={dispatch}
//                         />
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
            
//             {status !== 'drive_selection' && (
//                 <NavigationBreadcrumb
//                     currentView={status}
//                     selectedDrive={selectedDrive}
//                     onNavigateBack={handleNavigateBack}
//                     onNavigateHome={handleNavigateHome}
//                 />
//             )}

//             <div className="content">
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
        <div className="app-layout-wrapper">
            <div className="main-content-area">
                <h1>Surakshit Erase Prototype</h1>
                
                {status !== 'drive_selection' && (
                    <NavigationBreadcrumb
                        currentView={status}
                        selectedDrive={selectedDrive}
                        onNavigateBack={handleNavigateBack}
                        onNavigateHome={handleNavigateHome}
                    />
                )}

                <div className="content-wrapper">
                    <div className="content">
                        {renderContent()}
                    </div>
                </div>
            </div>
        </div>
    );
}

export default App;