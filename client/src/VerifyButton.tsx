import LoadingButton from "@mui/lab/LoadingButton";
import { MouseEventHandler } from "react";
import ArrowRight from "./assets/arrow-right.svg?react";

interface VerifyButtonProps {
  onClick?: MouseEventHandler;
  verifying?: boolean;
}

function VerifyButton({ onClick, verifying }: VerifyButtonProps) {
  return (
    <LoadingButton
      variant="contained"
      loading={verifying}
      endIcon={<ArrowRight fill="currentColor" />}
      onClick={onClick}
      loadingPosition="end"
    >
      Verify
    </LoadingButton>
  );
}

export default VerifyButton;
