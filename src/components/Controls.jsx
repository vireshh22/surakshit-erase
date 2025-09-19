// import { useReducer } from "react";
// import { reducer, initialState } from "../reducer";

// export default function Controls() {
//     const [state, dispatch] = useReducer(reducer, initialState);
//     const { 
//         status, 
//         selectedDrive, 
//         driveFileSystem, 
//         error, 
//         selectedMethod 
//     } = state;
//     return (
//         <div className="controls">
//             <h3>Select Wipe Method</h3>

//             <div className="method-selection">
//                 {["Clear", "Purge", "Destroy"].map((method) => (
//                     <div key={method} className="method-option">
//                         <div
//                             className={`method-card ${
//                                 selectedMethod === method ? "selected" : ""
//                             }`}
//                             onClick={() =>
//                                 dispatch({
//                                     type: "SELECT_METHOD",
//                                     payload: method,
//                                 })
//                             }
//                         >
//                             <div
//                                 className={`security-level ${
//                                     method.toLowerCase() === "clear"
//                                         ? "low"
//                                         : method.toLowerCase() === "purge"
//                                         ? "medium"
//                                         : "high"
//                                 }`}
//                             >
//                                 {method === "Clear"
//                                     ? "Low Security"
//                                     : method === "Purge"
//                                     ? "Medium Security"
//                                     : "High Security"}
//                             </div>
//                             <div className="method-header">
//                                 <div
//                                     className={`method-icon ${method.toLowerCase()}`}
//                                 >
//                                     {method === "Clear"
//                                         ? "🧹"
//                                         : method === "Purge"
//                                         ? "🔥"
//                                         : "💥"}
//                                 </div>
//                                 <div className="method-title">{method}</div>
//                             </div>
//                             <div className="method-description">
//                                 {method === "Clear" &&
//                                     "Basic overwrite with zeros. Fast but minimal security. Suitable for non-sensitive data."}
//                                 {method === "Purge" &&
//                                     "Multiple-pass overwrite with random data. Balanced security and speed for sensitive information."}
//                                 {method === "Destroy" &&
//                                     "Military-grade multi-pass overwrite. Maximum security for highly classified data. Takes longer."}
//                             </div>
//                         </div>
//                     </div>
//                 ))}
//             </div>

//             <button
//                 className="start-button"
//                 onClick={() => dispatch({ type: "START_WIPE" })}
//             >
//                 🚀 Start Security Wipe
//             </button>
//         </div>
//     );
// }


import React from 'react';
import { Zap, Shield, Bomb } from 'lucide-react';

const Controls = ({ selectedMethod, dispatch }) => {

    const methods = [
        {
            id: 'Clear',
            title: 'Clear',
            icon: <Zap size={24} />,
            security: 'Low Security',
            securityClass: 'low',
            description: 'Basic overwrite with zeros. Fast but minimal security. Suitable for non-sensitive data.',
            iconClass: 'clear'
        },
        {
            id: 'Purge',
            title: 'Purge',
            icon: <Shield size={24} />,
            security: 'Medium Security',
            securityClass: 'medium',
            description: 'Multiple-pass overwrite with random data. Balanced security and speed for sensitive information.',
            iconClass: 'purge'
        },
        {
            id: 'Destroy',
            title: 'Destroy',
            icon: <Bomb size={24} />,
            security: 'High Security',
            securityClass: 'high',
            description: 'Military-grade multi-pass overwrite. Maximum security for highly classified data. Takes longer.',
            iconClass: 'destroy'
        }
    ];

    const handleStartWipe = () => {
        dispatch({ type: 'START_WIPE' });
    };

    return (
        <div className="controls">
            <h3>Select Wipe Method</h3>
            
            <div className="method-selection">
                {methods.map((method) => (
                    <div key={method.id} className="method-option">
                        <div
                            className={`method-card ${
                                selectedMethod === method.id ? "selected" : ""
                            }`}
                            onClick={() => dispatch({
                                type: 'SELECT_METHOD',
                                payload: method.id
                            })}
                        >
                            <div className={`security-level ${method.securityClass}`}>
                                {method.security}
                            </div>
                            
                            <div className="method-header">
                                <div className={`method-icon ${method.iconClass}`}>
                                    {method.icon}
                                </div>
                                <h4 className="method-title">{method.title}</h4>
                            </div>
                            
                            <div className="method-description">
                                {method.description}
                            </div>
                        </div>
                    </div>
                ))}
            </div>

            <button
                className="start-button"
                onClick={handleStartWipe}
            >
                🚀 Start Security Wipe
            </button>
        </div>
    );
};

export default Controls;