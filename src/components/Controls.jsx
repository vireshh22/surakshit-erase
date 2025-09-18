import { useReducer } from "react";
import { reducer, initialState } from "../reducer";

export default function Controls() {
    const [state, dispatch] = useReducer(reducer, initialState);
    const { 
        status, 
        selectedDrive, 
        driveFileSystem, 
        error, 
        selectedMethod 
    } = state;
    return (
        <div className="controls">
            <h3>Select Wipe Method</h3>

            <div className="method-selection">
                {["Clear", "Purge", "Destroy"].map((method) => (
                    <div key={method} className="method-option">
                        <div
                            className={`method-card ${
                                selectedMethod === method ? "selected" : ""
                            }`}
                            onClick={() =>
                                dispatch({
                                    type: "SELECT_METHOD",
                                    payload: method,
                                })
                            }
                        >
                            <div
                                className={`security-level ${
                                    method.toLowerCase() === "clear"
                                        ? "low"
                                        : method.toLowerCase() === "purge"
                                        ? "medium"
                                        : "high"
                                }`}
                            >
                                {method === "Clear"
                                    ? "Low Security"
                                    : method === "Purge"
                                    ? "Medium Security"
                                    : "High Security"}
                            </div>
                            <div className="method-header">
                                <div
                                    className={`method-icon ${method.toLowerCase()}`}
                                >
                                    {method === "Clear"
                                        ? "🧹"
                                        : method === "Purge"
                                        ? "🔥"
                                        : "💥"}
                                </div>
                                <div className="method-title">{method}</div>
                            </div>
                            <div className="method-description">
                                {method === "Clear" &&
                                    "Basic overwrite with zeros. Fast but minimal security. Suitable for non-sensitive data."}
                                {method === "Purge" &&
                                    "Multiple-pass overwrite with random data. Balanced security and speed for sensitive information."}
                                {method === "Destroy" &&
                                    "Military-grade multi-pass overwrite. Maximum security for highly classified data. Takes longer."}
                            </div>
                        </div>
                    </div>
                ))}
            </div>

            <button
                className="start-button"
                onClick={() => dispatch({ type: "START_WIPE" })}
            >
                🚀 Start Security Wipe
            </button>
        </div>
    );
}
