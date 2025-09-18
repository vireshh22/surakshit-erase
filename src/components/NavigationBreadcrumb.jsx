// NavigationBreadcrumb.jsx
import React from "react";
import { ChevronLeft, Home, HardDrive } from "lucide-react";
import "../styles/NavigationBreadcrumb.css";

const NavigationBreadcrumb = ({ currentView, selectedDrive, onNavigateBack, onNavigateHome }) => {
    const getBreadcrumbItems = () => {
        const items = [];
        
        // Home
        items.push({
            label: "Drive Selection",
            icon: <Home size={14} />,
            onClick: onNavigateHome,
            active: currentView === "drive_selection"
        });

        // Selected Drive
        if (selectedDrive) {
            items.push({
                label: `${selectedDrive.name} Contents`,
                icon: <HardDrive size={14} />,
                onClick: null,
                active: currentView === "drive_contents"
            });
        }

        return items;
    };

    const breadcrumbItems = getBreadcrumbItems();

    return (
        <div className="navigation-breadcrumb">
            <div className="breadcrumb-container">
                {currentView !== "drive_selection" && (
                    <button
                        className="back-button"
                        onClick={onNavigateBack}
                        title="Go Back"
                    >
                        <ChevronLeft size={16} />
                        Back
                    </button>
                )}

                <div className="breadcrumb-items">
                    {breadcrumbItems.map((item, index) => (
                        <React.Fragment key={index}>
                            <div 
                                className={`breadcrumb-item ${item.active ? 'active' : ''} ${item.onClick ? 'clickable' : ''}`}
                                onClick={item.onClick}
                            >
                                {item.icon}
                                <span>{item.label}</span>
                            </div>
                            {index < breadcrumbItems.length - 1 && (
                                <span className="breadcrumb-separator">/</span>
                            )}
                        </React.Fragment>
                    ))}
                </div>
            </div>

            {selectedDrive && currentView === "drive_contents" && (
                <div className="drive-info-bar">
                    <div className="drive-info-item">
                        <span className="info-label">Selected Drive:</span>
                        <span className="info-value">{selectedDrive.path}</span>
                    </div>
                    {selectedDrive.size && (
                        <div className="drive-info-item">
                            <span className="info-label">Size:</span>
                            <span className="info-value">{selectedDrive.size}</span>
                        </div>
                    )}
                    {selectedDrive.model && (
                        <div className="drive-info-item">
                            <span className="info-label">Model:</span>
                            <span className="info-value">{selectedDrive.model}</span>
                        </div>
                    )}
                </div>
            )}
        </div>
    );
};

export default NavigationBreadcrumb;