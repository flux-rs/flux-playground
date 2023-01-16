import { Alert } from "@mui/material";
import api from "./api";

interface ResultAlertProps {
  status?: api.Status;
  onClose?: (event: React.SyntheticEvent) => void;
}

function ResultAlert({ status, onClose }: ResultAlertProps) {
  const severity = status == "safe" ? "success" : "error";
  let message;
  switch (status) {
    case "safe":
      message = (
        <span>
          The program is <strong>safe!</strong>
        </span>
      );
      break;
    case "unsafe":
      message = "The program might be unsafe";
      break;
    case "error":
      message = "The program contains errors";
      break;
  }

  return (
    <div>
      {status !== undefined && (
        <Alert severity={severity} onClose={onClose}>
          {message}
        </Alert>
      )}
    </div>
  );
}

export default ResultAlert;
