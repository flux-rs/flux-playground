import { LoadingButton } from "@mui/lab";
import { MouseEventHandler } from "react";
import "./VerifyButton.css";

interface VerifyButtonProps {
  onClick?: MouseEventHandler;
  verifying?: boolean;
}

function VerifyButton({ onClick, verifying }: VerifyButtonProps) {
  const arrowRight = (
    <span>
      <svg
        className="icon"
        height="16"
        viewBox="8 4 10 16"
        width="12"
        xmlns="http://www.w3.org/2000/svg"
      >
        <path d="M8 5v14l11-7z" />
      </svg>
    </span>
  );

  return (
    <LoadingButton
      variant="contained"
      loading={verifying}
      endIcon={arrowRight}
      onClick={onClick}
      loadingPosition="end"
    >
      Verify
    </LoadingButton>
  );
}

export default VerifyButton;
