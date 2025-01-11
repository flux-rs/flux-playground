import Alert from "@mui/material/Alert";
import api from "./api";

interface ResultAlertProps {
  status?: api.Status;
  onClose?: (event: React.SyntheticEvent) => void;
}

function ResultAlert({ status, onClose }: ResultAlertProps) {
  let message;
  switch (status) {
    case "success":
      message = (
        <span>
          The program is <strong>safe!</strong>
        </span>
      );
      break;
    case "error":
      message = "The program contains errors";
      break;
  }

  return (
    <div>
      {status !== undefined && (
        <Alert severity={status} onClose={onClose}>
          {message}
        </Alert>
      )}
    </div>
  );
}

export default ResultAlert;
