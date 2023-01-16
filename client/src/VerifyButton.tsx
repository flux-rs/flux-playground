import { MouseEventHandler } from "react";
import "./VerifyButton.css";

function VerifyButton({ onClick }: { onClick?: MouseEventHandler }) {
  const options = { minimap: { enabled: false } };

  return (
    <div className="verify-button">
      <button onClick={onClick}>
        <div className="button-inner">
          Verify
          <svg
            className="icon"
            height="14"
            viewBox="8 4 10 16"
            width="12"
            xmlns="http://www.w3.org/2000/svg"
          >
            <path d="M8 5v14l11-7z" />
          </svg>
        </div>
      </button>
    </div>
  );
}

export default VerifyButton;
